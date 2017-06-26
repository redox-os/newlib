use core::str::{self, FromStr};
use core::mem;
use libc::{c_int, c_char, size_t, c_void, ssize_t};
use ::types::{socklen_t, in_addr, sockaddr, sockaddr_in};
use syscall::{self, O_RDWR};
use syscall::error::{Error, EPROTOTYPE, EPROTONOSUPPORT, EAFNOSUPPORT, EINVAL, EOPNOTSUPP, ENOBUFS};
use core::slice;
use collections::Vec;
use byteorder::{BigEndian, ByteOrder};


pub const AF_INET: c_int = 2;
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;

static mut NTOA_ADDR: Option<Vec<u8>> = None;

libc_fn!(unsafe inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int {
    // TODO: octal/hex; more modern functions
    let addr = &mut (*inp).s_addr;
    let mut octets = str::from_utf8_unchecked(::cstr_to_slice(cp)).split('.');
    for i in 0..4 {
        if let Some(n) = octets.next().and_then(|x| u8::from_str(x).ok()) {
            addr[i] = n;
        } else {
            return 0;
        }
    }
    if octets.next() == None {
        1 // Success
    } else {
        0
    }
});

libc_fn!(unsafe inet_ntoa(inp: *const in_addr) -> *const c_char {
    let s_addr = (*inp).s_addr;
    let addr = format!("{}.{}.{}.{}\0", s_addr[0], s_addr[1], s_addr[2], s_addr[3]).into_bytes();
    let ptr = addr.as_ptr();
    NTOA_ADDR = Some(addr);
    ptr as *const c_char
});

libc_fn!(unsafe socket(domain: c_int, type_: c_int, protocol: c_int) -> Result<c_int> {
    if domain != AF_INET {
        Err(Error::new(EAFNOSUPPORT))
    } else if protocol != 0 {
        Err(Error::new(EPROTONOSUPPORT))
    } else {
        let fd = match type_ {
            SOCK_STREAM => syscall::open("tcp:", O_RDWR),
            SOCK_DGRAM => syscall::open("udp:", O_RDWR),
            _ => Err(Error::new(EPROTOTYPE))
        }?;
        Ok(fd as c_int)
    }
});

libc_fn!(unsafe connect(socket: c_int, address: *const sockaddr, _address_len: socklen_t) -> Result<c_int> {
    // XXX with UDP, should recieve messages only from that peer after this
    // XXX Check address_len
    if (*address).sa_family as i32 != AF_INET {
        return Err(Error::new(EINVAL))
    };
    let address = &*(address as *const sockaddr_in);
    let s_addr =address.sin_addr.s_addr;
    let path = format!("{}.{}.{}.{}:{}", s_addr[0], s_addr[1], s_addr[2], s_addr[3], ntohs(address.sin_port));
    let fd = syscall::dup(socket as usize, path.as_bytes())?;
    let ret = syscall::dup2(fd, socket as usize, &vec![]);
    let _ = syscall::close(fd);
    ret?;
    Ok(0)
});

libc_fn!(unsafe bind(socket: c_int, address: *const sockaddr, _address_len: socklen_t) -> Result<c_int> {
    // XXX Check address_len
    if (*address).sa_family as i32 != AF_INET {
        return Err(Error::new(EINVAL))
    };
    let address = &*(address as *const sockaddr_in);
    let s_addr =address.sin_addr.s_addr;
    let path = format!("/{}.{}.{}.{}:{}", s_addr[0], s_addr[1], s_addr[2], s_addr[3], ntohs(address.sin_port));
    let fd = syscall::dup(socket as usize, path.as_bytes())?;
    let ret = syscall::dup2(fd, socket as usize, &vec![]);
    let _ = syscall::close(fd);
    ret?;
    Ok(0)
});

libc_fn!(unsafe recv(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> Result<ssize_t> {
    // XXX flags
    if flags != 0 {
        Err(Error::new(EOPNOTSUPP))
    } else {
        let buf = slice::from_raw_parts_mut(buffer as *mut u8, length);
        Ok(syscall::read(socket as usize, buf)? as ssize_t)
    }
});

libc_fn!(unsafe send(socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> Result<ssize_t> {
    if flags != 0 {
        Err(Error::new(EOPNOTSUPP))
    } else {
        let buf = slice::from_raw_parts(buffer as *const u8, length);
        Ok(syscall::write(socket as usize, buf)? as ssize_t)
    }
});

libc_fn!(unsafe recvfrom(socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int, address: *const sockaddr, _address_len: *const socklen_t) -> Result<ssize_t>  {
    let fd = syscall::dup(socket as usize, b"listen")?;
    let mut path = [0; 4096];
    syscall::fpath(socket as usize, &mut path)?;
    // XXX process path and write to address
    let ret = recv(socket, buffer, length, flags);
    syscall::close(fd)?;
    Ok(ret)
});

libc_fn!(unsafe sendto(socket: c_int, message: *const c_void, length: size_t, flags: c_int, dest_addr: *const sockaddr, _dest_len: socklen_t) -> Result<ssize_t> {
    // XXX test dest_len
    if (*dest_addr).sa_family as i32 == AF_INET {
        let addr = &*(dest_addr as *const sockaddr_in);
        let s_addr = addr.sin_addr.s_addr;
        let url = format!("{}.{}.{}.{}:{}", s_addr[0], s_addr[1], s_addr[2], s_addr[3], ntohs(addr.sin_port));
        let fd = syscall::dup(socket as usize, url.as_bytes())?;
        let ret = send(fd as c_int, message, length, flags);
        syscall::close(fd)?;
        Ok(ret)
    } else {
        Err(Error::new(EOPNOTSUPP))
    }
});

libc_fn!(unsafe getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> Result<c_int> {
    // XXX will need to be changed for other sockaddr types
    if *address_len < mem::size_of::<sockaddr_in>() {
        return Err(Error::new(ENOBUFS));
    }
    *address_len = mem::size_of::<sockaddr_in>();
    let addr = &mut *(address as *mut sockaddr_in);
    addr.sin_family = AF_INET as u16;

    let mut path = [0; 4096];
    syscall::fpath(socket as usize, &mut path)?;
    let start;
    let sep;
    let end;
    {
        let mut iter = path.iter();
        start = iter.position(|x| *x == b':').ok_or(Error::new(EINVAL))? + 1;
        sep = start + iter.position(|x| *x == b':').ok_or(Error::new(EINVAL))?;
        end = sep + 1 + iter.position(|x| *x == b'/').ok_or(Error::new(EINVAL))?;
    }
    path[sep] = b'\0';

    if inet_aton(&path[start] as *const u8 as *const c_char, &mut addr.sin_addr) == 1 {
        if let Ok(port) = u16::from_str(str::from_utf8_unchecked(&path[sep+1..end])) {
            addr.sin_port = htons(port);
            Ok(0)
        } else {
            Err(Error::new(EINVAL))
        }
    } else {
        Err(Error::new(EINVAL)) // ?
    }
});

libc_fn!(unsafe getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> Result<c_int> {
    // XXX will need to be changed for other sockaddr types
    if *address_len < mem::size_of::<sockaddr_in>() {
        return Err(Error::new(ENOBUFS));
    }
    *address_len = mem::size_of::<sockaddr_in>();
    let addr = &mut *(address as *mut sockaddr_in);
    addr.sin_family = AF_INET as u16;

    let mut path = [0; 4096];
    syscall::fpath(socket as usize, &mut path)?;
    let start;
    let sep;
    let end;
    {
        let mut iter = path.iter();
        start = iter.position(|x| *x == b'/').ok_or(Error::new(EINVAL))? + 1;
        sep = start + iter.position(|x| *x == b':').ok_or(Error::new(EINVAL))?;
        end = sep + 1 + iter.position(|x| *x == b'\0').ok_or(Error::new(EINVAL))?;
    }
    path[sep] = b'\0';

    if inet_aton(&path[start] as *const u8 as *const c_char, &mut addr.sin_addr) == 1 {
        if let Ok(port) = u16::from_str(str::from_utf8_unchecked(&path[sep+1..end])) {
            addr.sin_port = htons(port);
            Ok(0)
        } else {
            Err(Error::new(EINVAL))
        }
    } else {
        Err(Error::new(EINVAL)) // ?
    }
});

libc_fn!(htonl(hostlong: u32) -> [u8; 4] {
    let mut netlong = [0; 4];
    BigEndian::write_u32(&mut netlong, hostlong);
    netlong
});

libc_fn!(htons(hostshort: u16) -> [u8; 2] {
    let mut netshort = [0; 2];
    BigEndian::write_u16(&mut netshort, hostshort);
    netshort
});

libc_fn!(ntohl(netlong: [u8; 4]) -> u32 {
    BigEndian::read_u32(&netlong)
});

libc_fn!(ntohs(netshort: [u8; 2]) -> u16 {
    BigEndian::read_u16(&netshort)
});

libc_fn!(setsockopt(socket: c_int, level: c_int, option_name: c_int, option_value: *const c_void, option_len: socklen_t) -> Result<c_int> {
    syscall::write(2, format!("unimplemented: setsockopt({}, {}, {}, {:?}, {})\n",
                              socket, level, option_name, option_value, option_len).as_bytes()).unwrap();
    Err(Error::new(syscall::ENOSYS))
});
