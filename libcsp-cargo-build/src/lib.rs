pub struct CspBuildOpts {
    rtable: bool,
    csp_print: bool,
    promisc: bool,
    rdp: bool,
    yaml: bool,
}

impl Default for CspBuildOpts {
    fn default() -> Self {
        Self {
            rtable: true,
            csp_print: true,
            promisc: true,
            rdp: true,
            yaml: true,
        }
    }
}

pub struct Builder {
    opts: CspBuildOpts,
    build: cc::Build,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            opts: CspBuildOpts::default(),
            build: cc::Build::new(),
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cc(&mut self) -> &mut cc::Build {
        &mut self.build
    }

    pub fn compile(&mut self) {
        self.build
            .file("libcsp/src/csp_bridge.c")
            .file("libcsp/src/csp_buffer.c")
            .file("libcsp/src/csp_crc32.c")
            .file("libcsp/src/csp_debug.c")
            .file("libcsp/src/csp_id.c")
            .file("libcsp/src/csp_iflist.c")
            .file("libcsp/src/csp_init.c")
            .file("libcsp/src/csp_io.c")
            .file("libcsp/src/csp_port.c")
            .file("libcsp/src/csp_promisc.c")
            .file("libcsp/src/csp_qfifo.c")
            .file("libcsp/src/csp_port.c")
            .file("libcsp/src/csp_route.c");
        if self.opts.rdp {
            self.build.file("libcsp/src/csp_rdp.c");
            self.build.file("libcsp/src/csp_rdp_queue.c");
        }
        if self.opts.promisc {
            self.build.file("libcsp/src/csp_promisc.c");
        }
        if self.opts.csp_print {
            self.build.file("libcsp/src/csp_hex_dump.c");
        }
        if self.opts.yaml {
            self.build.file("libcsp/src/csp_yaml.c");
        }
        if self.opts.rtable {
            self.build.file("libcsp/src/csp_rtable_cidr.c");
        }

        // TODO: UNIX does not necesarilly mean POSIX? Details to deal with later..
        #[cfg(unix)]
        self.posix_arch_files();

        self.build.include("cfg");
        self.build.include("libcsp/include");
        self.build.include("libcsp/src");

        self.build.compile("csp");
    }

    #[cfg(unix)]
    fn posix_arch_files(&mut self) {
        self.build
            .file("libcsp/src/arch/posix/csp_clock.c")
            .file("libcsp/src/arch/posix/csp_semaphore.c")
            .file("libcsp/src/arch/posix/csp_system.c")
            .file("libcsp/src/arch/posix/csp_time.c")
            .file("libcsp/src/arch/posix/csp_queue.c")
            .file("libcsp/src/arch/posix/pthread_queue.c");
    }
}
