#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std;

use bitflags::bitflags;

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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
pub const CSP_ANY: u32 = 255;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_timestamp_t {
    pub tv_sec: u32,
    pub tv_nsec: u32,
}

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

#[doc = "CSP identifier/header."]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct csp_id_t {
    pub pri: u8,
    pub flags: u8,
    pub src: u16,
    pub dst: u16,
    pub dport: u8,
    pub sport: u8,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MsgPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_conn_s {
    pub _address: u8,
}

#[doc = " CSP Packet.\n\n This structure is constructed to fit with all interface and protocols to prevent the\n need to copy data (zero copy).\n\n .. note:: In most cases a CSP packet cannot be reused in case of send failure, because the\n \t\t\t lower layers may add additional data causing increased length (e.g. CRC32), convert\n \t\t\t the CSP id to different endian (e.g. I2C), etc.\n"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csp_packet_s {
    pub __bindgen_anon_1: csp_packet_s_anon_union,
    pub length: u16,
    pub id: csp_id_t,
    pub next: *mut csp_packet_s,
    #[doc = " Additional header bytes, to prepend packed data before transmission\n This must be minimum 6 bytes to accomodate CSP 2.0. But some implementations\n require much more scratch working area for encryption for example.\n\n Ultimately after csp_id_pack() this area will be filled with the CSP header"]
    pub header: [u8; 8usize],
    pub __bindgen_anon_2: csp_packet_s_data_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union csp_packet_s_anon_union {
    pub rdp_only: csp_packet_s_anon_union_field_rdp_only,
    pub rx_tx_only: csp_packet_s_anon_union_field_rx_tx_only,
}

impl Default for csp_packet_s_anon_union {
    fn default() -> Self {
        Self {
            rdp_only: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_packet_s_anon_union_field_rdp_only {
    pub rdp_quarantine: u32,
    pub timestamp_tx: u32,
    pub timestamp_rx: u32,
    pub conn: *mut csp_conn_s,
}

impl Default for csp_packet_s_anon_union_field_rdp_only {
    fn default() -> Self {
        Self {
            rdp_quarantine: Default::default(),
            timestamp_tx: Default::default(),
            timestamp_rx: Default::default(),
            conn: core::ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_packet_s_anon_union_field_rx_tx_only {
    pub rx_count: u16,
    pub remain: u16,
    pub cfpid: u32,
    pub last_used: u32,
    pub frame_begin: *mut u8,
    pub frame_length: u16,
}

impl Default for csp_packet_s_anon_union_field_rx_tx_only {
    fn default() -> Self {
        Self {
            rx_count: Default::default(),
            remain: Default::default(),
            cfpid: Default::default(),
            last_used: Default::default(),
            frame_begin: core::ptr::null_mut(),
            frame_length: Default::default(),
        }
    }
}

#[doc = " Data part of packet:"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union csp_packet_s_data_union {
    pub data: [u8; 256usize],
    pub data16: [u16; 128usize],
    pub data32: [u32; 64usize],
}

impl Default for csp_packet_s_data_union {
    fn default() -> Self {
        Self {
            data: [0; 256usize],
        }
    }
}

#[doc = " CSP Packet.\n\n This structure is constructed to fit with all interface and protocols to prevent the\n need to copy data (zero copy).\n\n .. note:: In most cases a CSP packet cannot be reused in case of send failure, because the\n \t\t\t lower layers may add additional data causing increased length (e.g. CRC32), convert\n \t\t\t the CSP id to different endian (e.g. I2C), etc.\n"]
pub type csp_packet_t = csp_packet_s;

pub struct CspPacket(pub csp_packet_s);

impl CspPacket {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CspPacket {
    fn default() -> Self {
        Self(csp_packet_s {
            __bindgen_anon_1: Default::default(),
            length: Default::default(),
            id: Default::default(),
            next: core::ptr::null_mut(),
            header: Default::default(),
            __bindgen_anon_2: Default::default(),
        })
    }
}

pub type csp_queue_handle_t = *mut core::ffi::c_void;
pub type csp_static_queue_t = *mut core::ffi::c_void;

#[doc = " @brief Connection struct"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct csp_socket_s {
    pub rx_queue: csp_queue_handle_t,
    pub rx_queue_static: csp_static_queue_t,
    pub rx_queue_static_data: [core::ffi::c_char; 128usize],
    pub opts: u32,
}

#[test]
fn bindgen_test_layout_csp_socket_s() {
    const UNINIT: ::std::mem::MaybeUninit<csp_socket_s> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<csp_socket_s>(),
        152usize,
        concat!("Size of: ", stringify!(csp_socket_s))
    );
    assert_eq!(
        ::std::mem::align_of::<csp_socket_s>(),
        8usize,
        concat!("Alignment of ", stringify!(csp_socket_s))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rx_queue) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(csp_socket_s),
            "::",
            stringify!(rx_queue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rx_queue_static) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(csp_socket_s),
            "::",
            stringify!(rx_queue_static)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rx_queue_static_data) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(csp_socket_s),
            "::",
            stringify!(rx_queue_static_data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).opts) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(csp_socket_s),
            "::",
            stringify!(opts)
        )
    );
}
#[doc = " Forward declaration of socket structure"]
pub type csp_socket_t = csp_socket_s;

extern "C" {
    #[doc = " Error counters"]
    pub static mut csp_dbg_buffer_out: u8;
    pub static mut csp_dbg_conn_out: u8;
    pub static mut csp_dbg_conn_ovf: u8;
    pub static mut csp_dbg_conn_noroute: u8;
    pub static mut csp_dbg_inval_reply: u8;
    pub static mut csp_dbg_errno: u8;
    pub static mut csp_dbg_can_errno: u8;
    pub static mut csp_dbg_eth_errno: u8;
    pub static mut csp_dbg_rdp_print: u8;
    pub static mut csp_dbg_packet_print: u8;

    #[doc = " Initialize CSP.\n This will configure basic structures."]
    pub fn csp_init();

    #[cfg(feature = "std")]
    pub fn csp_print_func(fmt: *const ::std::os::raw::c_char, ...);

    #[doc = " Bind port to socket.\n\n @param[in] socket socket to bind port to\n @param[in] port port number to bind, use #CSP_ANY for all ports. Bindnig to a specific will take precedence over #CSP_ANY.\n @return #CSP_ERR_NONE on success, otherwise an error code."]
    pub fn csp_bind(socket: *mut csp_socket_t, port: u8) -> core::ffi::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};
    use std::mem::MaybeUninit;

    #[test]
    fn bindgen_test_layout_csp_timestamp_t() {
        const UNINIT: MaybeUninit<csp_timestamp_t> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            ::core::mem::size_of::<csp_timestamp_t>(),
            8usize,
            concat!("Size of: ", stringify!(csp_timestamp_t))
        );
        assert_eq!(
            std::mem::align_of::<csp_timestamp_t>(),
            4usize,
            concat!("Alignment of ", stringify!(csp_timestamp_t))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_timestamp_t),
                "::",
                stringify!(tv_sec)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).tv_nsec) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_timestamp_t),
                "::",
                stringify!(tv_nsec)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_id() {
        const UNINIT: MaybeUninit<csp_id_t> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_id_t>(),
            8usize,
            concat!("Size of: ", stringify!(__packed))
        );
        assert_eq!(
            align_of::<csp_id_t>(),
            2usize,
            concat!("Alignment of ", stringify!(__packed))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).pri) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(pri)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(flags)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).src) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(src)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).dst) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(dst)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).dport) as usize - ptr as usize },
            6usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(dport)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).sport) as usize - ptr as usize },
            7usize,
            concat!(
                "Offset of field: ",
                stringify!(__packed),
                "::",
                stringify!(sport)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_1__bindgen_ty_1() {
        const UNINIT: MaybeUninit<csp_packet_s_anon_union_field_rdp_only> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s_anon_union_field_rdp_only>(),
            24usize,
            concat!(
                "Size of: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1)
            )
        );
        assert_eq!(
            align_of::<csp_packet_s_anon_union_field_rdp_only>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).rdp_quarantine) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(rdp_quarantine)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).timestamp_tx) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(timestamp_tx)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).timestamp_rx) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(timestamp_rx)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).conn) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(conn)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_1__bindgen_ty_2() {
        const UNINIT: MaybeUninit<csp_packet_s_anon_union_field_rx_tx_only> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s_anon_union_field_rx_tx_only>(),
            32usize,
            concat!(
                "Size of: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2)
            )
        );
        assert_eq!(
            align_of::<csp_packet_s_anon_union_field_rx_tx_only>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).rx_count) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(rx_count)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).remain) as usize - ptr as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(remain)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).cfpid) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(cfpid)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).last_used) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(last_used)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).frame_begin) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(frame_begin)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).frame_length) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(frame_length)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_1() {
        assert_eq!(
            size_of::<csp_packet_s_anon_union>(),
            32usize,
            concat!("Size of: ", stringify!(csp_packet_s_anon_union))
        );
        assert_eq!(
            align_of::<csp_packet_s_anon_union>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_packet_s_anon_union))
        );
    }

    #[test]
    fn bindgen_test_layout_csp_packet_s__bindgen_ty_2() {
        const UNINIT: MaybeUninit<csp_packet_s_data_union> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s_data_union>(),
            256usize,
            concat!("Size of: ", stringify!(csp_packet_s__bindgen_ty_2))
        );
        assert_eq!(
            align_of::<csp_packet_s_data_union>(),
            4usize,
            concat!("Alignment of ", stringify!(csp_packet_s__bindgen_ty_2))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_2),
                "::",
                stringify!(data)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).data16) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_2),
                "::",
                stringify!(data16)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).data32) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s__bindgen_ty_2),
                "::",
                stringify!(data32)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_csp_packet_s() {
        const UNINIT: MaybeUninit<csp_packet_s> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();
        assert_eq!(
            size_of::<csp_packet_s>(),
            320usize,
            concat!("Size of: ", stringify!(csp_packet_s))
        );
        assert_eq!(
            align_of::<csp_packet_s>(),
            8usize,
            concat!("Alignment of ", stringify!(csp_packet_s))
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(length)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
            34usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(id)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(next)
            )
        );
        assert_eq!(
            unsafe { ::std::ptr::addr_of!((*ptr).header) as usize - ptr as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(csp_packet_s),
                "::",
                stringify!(header)
            )
        );
    }
}
