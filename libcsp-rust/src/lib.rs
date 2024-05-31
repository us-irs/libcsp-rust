#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

pub mod ffi;
use core::time::Duration;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use bitflags::bitflags;
use ffi::{csp_conn_s, csp_packet_s, csp_socket_s};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ReservedPorts {
    Cmp = 0,
    Ping = 1,
    Ps = 2,
    Memfree = 3,
    Reboot = 4,
    BufFree = 5,
    Uptime = 6,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CspError {
    None = 0,
    NoMem = -1,
    Inval = -2,
    TimedOut = -3,
    Used = -4,
    NotSup = -5,
    Busy = -6,
    Already = -7,
    Reset = -8,
    NoBufs = -9,
    Tx = -10,
    Driver = -11,
    Again = -12,
    NoSys = -38,
    Hmac = -100,
    Crc32 = -102,
    Sfp = -103,
}

/// Listen on all ports, primarily used with [csp_bind]
pub const CSP_ANY: u8 = 255;

bitflags! {
    pub struct SocketFlags: u32 {
        const NONE = 0x0000;
        /// RDP required.
        const RDPREQ = 0x0001;
        /// RDP prohibited.
        const RDPPROHIB = 0x0002;
        /// HMAC required
        const HMACREQ = 0x0004;
        /// HMAC prohibited.
        const HMACPROHIB = 0x0008;
        /// CRC32 required.
        const CRC32REQ = 0x0040;
        const CRC32PROHIB = 0x0080;
        const CONN_LESS = 0x0100;
        /// Copy opts from incoming packets. Only applies to [csp_sendto_reply]
        const SAME = 0x8000;

        // The source may set any bits
        const _ = !0;
    }
}

bitflags! {
    pub struct ConnectOpts: u32 {
        const NONE = SocketFlags::NONE.bits();

        const RDP = SocketFlags::RDPREQ.bits();
        const NORDP = SocketFlags::RDPPROHIB.bits();
        const HMAC = SocketFlags::HMACREQ.bits();
        const NOHMAC = SocketFlags::HMACPROHIB.bits();
        const CRC32 = SocketFlags::CRC32REQ.bits();
        const NOCRC32 = SocketFlags::CRC32PROHIB.bits();
        const SAME = SocketFlags::SAME.bits();

        // The source may set any bits
        const _ = !0;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MsgPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

pub struct CspPacket(pub csp_packet_s);

pub struct CspPacketRef<'a>(&'a csp_packet_s);

impl<'a> CspPacketRef<'a> {
    pub fn packet_data(&self) -> &'a [u8; ffi::CSP_BUFFER_SIZE] {
        unsafe { &self.0.packet_data_union.data }
    }

    pub fn inner(&self) -> *const csp_packet_s {
        self.0
    }

    pub fn inner_mut(&self) -> *const csp_packet_s {
        self.0
    }
}

impl CspPacket {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CspPacket {
    fn default() -> Self {
        Self(csp_packet_s {
            packet_info: Default::default(),
            length: Default::default(),
            id: Default::default(),
            next: core::ptr::null_mut(),
            header: Default::default(),
            packet_data_union: Default::default(),
        })
    }
}

#[derive(Default)]
pub struct CspSocket(pub csp_socket_s);

impl CspSocket {
    pub fn inner_as_mut_ptr(&mut self) -> *mut csp_socket_s {
        &mut self.0
    }
}

/// Rust wrapper for [ffi::csp_init]. Initialize the CSP stack.
///
/// # Safety
///
/// - You must call this function only once.
pub unsafe fn csp_init() {
    // SAFETY: FFI call
    unsafe {
        ffi::csp_init();
    }
}

/// Rust wrapper for [ffi::csp_bind].
pub fn csp_bind(socket: &mut CspSocket, port: u8) {
    // SAFETY: FFI call
    unsafe {
        ffi::csp_bind(socket.inner_as_mut_ptr(), port);
    }
}

/// Rust wrapper for [ffi::csp_listen].
pub fn csp_listen(socket: &mut CspSocket, backlog: usize) {
    // SAFETY: FFI call
    unsafe {
        ffi::csp_listen(socket.inner_as_mut_ptr(), backlog);
    }
}

/// Rust wrapper for [ffi::csp_route_work].
pub fn csp_route_work_raw() -> i32 {
    unsafe { ffi::csp_route_work() }
}

/// Rust wrapper for [ffi::csp_route_work] which also converts errors to the [CspError] type.
/// This function will panic if the returned error type is not among the known values of
/// [CspError].
///
/// [csp_route_work_raw] can be used if this is not acceptable.
pub fn csp_route_work() -> Result<(), CspError> {
    let result = unsafe { ffi::csp_route_work() };
    if result == CspError::None as i32 {
        return Ok(());
    }
    Err(CspError::try_from(result).expect("unexpected error type from csp_route_work"))
}

#[derive(Debug, Clone)]
pub struct CspConn(csp_conn_s);

impl CspConn {
    fn new(address: u8) -> Self {
        Self(csp_conn_s { address })
    }

    pub fn addr(&self) -> u8 {
        self.0.address
    }
}

/// Rust wrapper for [ffi::csp_accept].
pub fn csp_accept(socket: &mut CspSocket, timeout: Duration) -> Option<CspConn> {
    let timeout_millis = timeout.as_millis();
    if timeout_millis > u32::MAX as u128 {
        return None;
    }
    Some(CspConn::new(unsafe {
        let addr = ffi::csp_accept(socket.inner_as_mut_ptr(), timeout_millis as u32);
        if addr.is_null() {
            return None;
        }
        (*addr).address
    }))
}

/// Rust wrapper for [ffi::csp_read].
pub fn csp_read(conn: &mut CspConn, timeout: Duration) -> Option<CspPacketRef<'_>> {
    let timeout_millis = timeout.as_millis();
    if timeout_millis > u32::MAX as u128 {
        return None;
    }
    let opt_packet = unsafe { ffi::csp_read(&mut conn.0, timeout_millis as u32) };
    if opt_packet.is_null() {
        return None;
    }
    // SAFETY: FFI pointer. As long as it is used beyond the lifetime of the connection, this
    // should be fine. The passed [CspConn] value should ensure that.
    Some(CspPacketRef(unsafe { &mut *opt_packet }))
}
