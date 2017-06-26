#![allow(non_camel_case_types, dead_code)]

use libc;

pub type pid_t = libc::c_int;

pub type time_t = i64;
pub type suseconds_t = libc::c_long;

// Socket related types
pub type in_addr_t = [u8; 4];
pub type sa_family_t = u16;
pub type socklen_t = libc::size_t;
pub type in_port_t = [u8; 2];

#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    sa_data: [libc::c_char; 14]
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    __pad: [u8; 8]
}

#[repr(C)]
pub struct hostent {
    pub h_name: *const libc::c_char,
    pub h_aliases: *const *const libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *const *const libc::c_char
}

#[repr(C)]
#[derive(Debug)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t
}

pub type fd_mask = libc::c_ulong;
pub const FD_SETSIZE: usize = 64;
pub const NFDBITS: usize = 8 * 8; // Bits in a fd_mask

#[repr(C)]
#[derive(Debug)]
pub struct fd_set {
    pub fds_bits: [fd_mask; (FD_SETSIZE + NFDBITS - 1) / NFDBITS]
}
