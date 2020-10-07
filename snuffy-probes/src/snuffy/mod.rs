pub const CONFIG_KEY: usize = 1;
pub const COMM_LEN: usize = 16;
pub const BUF_LEN: usize = 400;
pub const HOST_LEN: usize = 256;

#[repr(C)]
#[derive(Clone)]
pub struct Config {
    pub target_comm_set: usize,
    pub target_comm: [u8; COMM_LEN],
}

#[repr(C)]
pub struct DNS {
    pub addr: u32,
    pub host: [u8; HOST_LEN],
}
#[repr(C)]
#[derive(Clone)]
pub struct Connection {
    pub fd: u64,
    pub addr: u32,
    pub port: u32,
}

#[repr(C)]
pub struct SSLBuffer {
    pub ssl_ctx: usize,
    pub mode: AccessMode,
    pub len: usize,
    pub chunk_len: usize,
    pub chunk: [u8; BUF_LEN],
}

#[repr(C)]
pub struct SSLHost {
    pub ssl_ctx: usize,
    pub host: [u8; HOST_LEN],
}

#[repr(C)]
pub struct SSLFd {
    pub ssl_ctx: usize,
    pub fd: usize,
}

#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum AccessMode {
    Read,
    Write,
}
