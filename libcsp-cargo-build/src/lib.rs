use std::{
    io::{self, Write},
    path::PathBuf,
};

pub mod autoconf {
    pub const CFG_POSIX: &str = "CSP_POSIX";
    pub const CFG_ZEPHYR: &str = "CSP_ZEPHYR";

    pub const CFG_HAVE_STDIO: &str = "CSP_HAVE_STDIO";
    pub const CFG_ENABLE_CSP_PRINT: &str = "CSP_ENABLE_CSP_PRINT";
    pub const CFG_PRINT_STDIO: &str = "CSP_PRINT_STDIO";

    pub const CFG_REPRODUCIBLE_BUILDS: &str = "CSP_REPRODUCIBLE_BUILDS";

    pub const CFG_QFIFO_LEN: &str = "CSP_QFIFO_LEN";
    pub const CFG_PORT_MAX_BIND: &str = "CSP_PORT_MAX_BIND";
    pub const CFG_CONN_RXQUEUE_LEN: &str = "CSP_CONN_RXQUEUE_LEN";
    pub const CFG_CONN_MAX: &str = "CSP_CONN_MAX";
    pub const CFG_BUFFER_SIZE: &str = "CSP_BUFFER_SIZE";
    pub const CFG_BUFFER_COUNT: &str = "CSP_BUFFER_COUNT";
    pub const CFG_RDP_MAX_WINDOW: &str = "CSP_RDP_MAX_WINDOW";
    pub const CFG_RTABLE_SIZE: &str = "CSP_RTABLE_SIZE";

    pub const CFG_USE_RDP: &str = "CSP_USE_RDP";
    pub const CFG_USE_HMAC: &str = "CSP_USE_HMAC";
    pub const CFG_USE_PROMISC: &str = "CSP_USE_PROMISC";
    pub const CFG_USE_RTABLE: &str = "CSP_USE_RTABLE";
    pub const CFG_HAVE_LIBSOCKETCAN: &str = "CSP_HAVE_LIBSOCKETCAN";
    pub const CFG_HAVE_LIBZMQ: &str = "CSP_HAVE_LIBZMQ";
}

const SRCS_LIST: &[&str] = &[
    "csp_bridge.c",
    "csp_buffer.c",
    "csp_crc32.c",
    "csp_debug.c",
    "csp_id.c",
    "csp_iflist.c",
    "csp_conn.c",
    "csp_init.c",
    "csp_io.c",
    "csp_port.c",
    "csp_promisc.c",
    "csp_qfifo.c",
    "csp_port.c",
    "csp_route.c",
    "interfaces/csp_if_lo.c",
    "interfaces/csp_if_kiss.c",
    "interfaces/csp_if_tun.c",
    "interfaces/csp_if_udp.c",
];

const ARCH_SRCS_UNIX: &[&str] = &[
    "arch/posix/csp_clock.c",
    "arch/posix/csp_semaphore.c",
    "arch/posix/csp_system.c",
    "arch/posix/csp_time.c",
    "arch/posix/csp_queue.c",
    "arch/posix/pthread_queue.c",
];

pub struct Config {
    have_stdio: bool,
    print_stdio: bool,
    reproducible_builds: bool,
    qfifo_len: u32,
    port_max_bind: u32,
    conn_rx_queue_len: u32,
    conn_max: u32,
    buffer_size: u32,
    buffer_count: u32,
    rdp_max_window: u32,
    rtable_size: u32,
    hmac: bool,
    rtable: bool,
    csp_print: bool,
    promisc: bool,
    rdp: bool,
    yaml: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            have_stdio: true,
            print_stdio: true,
            reproducible_builds: false,
            qfifo_len: 16,
            port_max_bind: 16,
            conn_rx_queue_len: 16,
            conn_max: 8,
            buffer_size: 256,
            buffer_count: 15,
            rdp_max_window: 5,
            rtable_size: 10,
            hmac: true,
            rtable: false,
            csp_print: true,
            promisc: true,
            rdp: true,
            yaml: false,
        }
    }
}

pub struct Builder {
    generate_autoconf_file: bool,
    libcsp_path: PathBuf,
    libcsp_src_path_base: PathBuf,
    out_dir: PathBuf,
    cfg: Config,
    build: cc::Build,
}

impl Builder {
    pub fn new(libcsp_path: PathBuf, out_dir: PathBuf) -> Self {
        let mut libcsp_src_path_base = libcsp_path.clone();
        libcsp_src_path_base.push("src");
        Self {
            generate_autoconf_file: true,
            libcsp_path,
            libcsp_src_path_base,
            out_dir,
            cfg: Default::default(),
            build: Default::default(),
        }
    }

    pub fn cc(&mut self) -> &mut cc::Build {
        &mut self.build
    }

    pub fn compile(&mut self) -> io::Result<()> {
        if self.generate_autoconf_file {
            self.generate_autoconf_file()?;
        }
        for src in SRCS_LIST {
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push(src);
            self.build.file(next_file);
        }
        if self.cfg.rdp {
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push("csp_rdp.c");
            self.build.file(next_file);
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push("csp_rdp_queue.c");
            self.build.file(next_file);
        }
        if self.cfg.promisc {
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push("csp_promisc.c");
            self.build.file(next_file);
        }
        if self.cfg.csp_print {
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push("csp_hex_dump.c");
            self.build.file(next_file);
        }
        if self.cfg.yaml {
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push("csp_yaml.c");
            self.build.file(next_file);
        }
        if self.cfg.rtable {
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push("csp_rtable_cidr.c");
            self.build.file(next_file);
        }

        // TODO: UNIX does not necesarilly mean POSIX? Details to deal with later..
        #[cfg(unix)]
        self.posix_arch_files();

        let mut inc_path = self.libcsp_path.clone();
        inc_path.push("include");
        self.build.include(inc_path);
        self.build.include(&self.libcsp_src_path_base);

        self.build.compile("csp");
        Ok(())
    }

    #[cfg(unix)]
    fn posix_arch_files(&mut self) {
        for src in ARCH_SRCS_UNIX {
            let mut next_file = self.libcsp_src_path_base.clone();
            next_file.push(src);
            self.build.file(next_file);
        }
    }

    pub fn generate_autoconf_file(&mut self) -> io::Result<()> {
        let mut autoconf_dir = self.out_dir.join("cfg");
        self.build.include(&autoconf_dir);
        autoconf_dir.push("csp");
        std::fs::create_dir_all(&autoconf_dir)?;
        generate_autoconf_file(autoconf_dir, &self.cfg)
    }
}

pub fn generate_autoconf_file(out_dir: PathBuf, cfg: &Config) -> io::Result<()> {
    // panic!("cargo:warning=outdir for autoconf file: {:?}", out_dir);
    let mut autoconf_file_string = String::new();
    #[cfg(unix)]
    autoconf_file_string.push_str("#define CSP_POSIX 1\n");
    autoconf_file_string.push_str("#define CSP_ZEPHYR 0\n");
    autoconf_file_string.push('\n');
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_HAVE_STDIO,
        cfg.have_stdio as u32
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_ENABLE_CSP_PRINT,
        cfg.csp_print as u32
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_PRINT_STDIO,
        cfg.print_stdio as u32
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_REPRODUCIBLE_BUILDS,
        cfg.reproducible_builds as u32
    ));
    autoconf_file_string.push('\n');

    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_QFIFO_LEN,
        cfg.qfifo_len
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_PORT_MAX_BIND,
        cfg.port_max_bind
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_CONN_RXQUEUE_LEN,
        cfg.conn_rx_queue_len
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_CONN_MAX,
        cfg.conn_max
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_BUFFER_SIZE,
        cfg.buffer_size
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_BUFFER_COUNT,
        cfg.buffer_count
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_RDP_MAX_WINDOW,
        cfg.rdp_max_window
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_RTABLE_SIZE,
        cfg.rtable_size
    ));

    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_USE_RDP,
        cfg.rdp as u32
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_USE_HMAC,
        cfg.hmac as u32
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_USE_PROMISC,
        cfg.promisc as u32
    ));
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_USE_RTABLE,
        cfg.rtable as u32
    ));

    // TODO: Maybe those will be added at some point..
    autoconf_file_string.push_str(&format!(
        "#define {} {}\n",
        autoconf::CFG_HAVE_LIBSOCKETCAN,
        0
    ));
    autoconf_file_string.push_str(&format!("#define {} {}\n", autoconf::CFG_HAVE_LIBZMQ, 0));
    let out_file = out_dir.join("autoconfig.h");
    let mut file = std::fs::File::create(out_file)?;
    file.write_all(autoconf_file_string.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO: Unittest autoconf generator.
}
