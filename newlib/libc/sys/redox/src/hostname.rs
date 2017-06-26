use core::ptr::null;
use core::{mem, str};
use collections::vec::IntoIter;
use collections::string::ToString;
use collections::{Vec, String};
use ::dns::{Dns, DnsQuery};
use syscall::{self, Result, EINVAL, Error};
use libc::c_char;
use ::types::{in_addr, hostent};

static mut HOST_ENTRY: hostent = hostent { h_name: null(), h_aliases: null(), h_addrtype: 0, h_length: 0, h_addr_list: null() };
static mut HOST_NAME: Option<Vec<u8>> = None;
static mut HOST_ALIASES: [*const c_char; 1] = [null()];
static mut HOST_ADDR: Option<in_addr> = None;
static mut HOST_ADDR_LIST: [*const c_char; 2] = [null(); 2];

struct LookupHost(IntoIter<in_addr>);

impl Iterator for LookupHost {
    type Item = in_addr;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// Modified from rust/sys/redox/net/mod.rs
fn lookup_host(host: &str) -> Result<LookupHost> {
    // XXX better error handling
    let ip_string = String::from_utf8(::file_read_all("/etc/net/ip")?)
        .or(Err(Error::new(syscall::EIO)))?;
    let ip: Vec<u8> = ip_string.trim().split(".").map(|part| part.parse::<u8>()
                               .unwrap_or(0)).collect();

    let dns_string = String::from_utf8(::file_read_all("/etc/net/dns")?)
        .or(Err(Error::new(syscall::EIO)))?;
    let dns: Vec<u8> = dns_string.trim().split(".").map(|part| part.parse::<u8>()
                                 .unwrap_or(0)).collect();

    if ip.len() == 4 && dns.len() == 4 {
        let mut timespec = syscall::TimeSpec::default();
        syscall::clock_gettime(syscall::CLOCK_REALTIME, &mut timespec).unwrap();
        let tid = (timespec.tv_nsec >> 16) as u16;

        let packet = Dns {
            transaction_id: tid,
            flags: 0x0100,
            queries: vec![DnsQuery {
                name: host.to_string(),
                q_type: 0x0001,
                q_class: 0x0001,
            }],
            answers: vec![]
        };

        let packet_data = packet.compile();

        let fd = syscall::open(format!("udp:{}.{}.{}.{}:0",
                                       ip[0], ip[1], ip[2], ip[3]).as_bytes(),
                               syscall::O_RDWR)?;

        let timeout = syscall::TimeSpec {
            tv_sec: 5,
            tv_nsec: 0,
        };
        let rt = syscall::dup(fd, b"read_timeout")?;
        syscall::write(rt, &timeout)?;
        syscall::close(rt)?;
        let wt = syscall::dup(fd, b"write_timeout")?;
        syscall::write(wt, &timeout)?;
        syscall::close(wt)?;

        let sendrecvfd = syscall::dup(fd, format!("{}.{}.{}.{}:53", dns[0], dns[1], dns[2], dns[3]).as_bytes())?;
        syscall::write(sendrecvfd, &packet_data)?;
        let mut buf = [0; 65536];
        let count = syscall::read(sendrecvfd, &mut buf)?;
        syscall::close(sendrecvfd)?;
        syscall::close(fd)?;

        match Dns::parse(&buf[.. count]) {
            Ok(response) => {
                let mut addrs = vec![];
                for answer in response.answers.iter() {
                    if answer.a_type == 0x0001 && answer.a_class == 0x0001
                       && answer.data.len() == 4
                    {
                        let addr = in_addr {
                            s_addr: [answer.data[0], answer.data[1], answer.data[2], answer.data[3]]
                        };
                        addrs.push(addr);
                    }
                }
                Ok(LookupHost(addrs.into_iter()))
            },
            Err(_err) => Err(Error::new(EINVAL))
        }
    } else {
        Err(Error::new(EINVAL))
    }
}

libc_fn!(unsafe gethostbyname(name: *const c_char) -> Result<*const hostent> {
    // XXX h_errno
    let mut addr = mem::uninitialized();
    let host_addr = if ::socket::inet_aton(name, &mut addr) == 1 {
        addr
    } else {
        // XXX
        let mut host = lookup_host(str::from_utf8_unchecked(::cstr_to_slice(name)))?;
        host.next().ok_or(Error::new(syscall::ENOENT))? // XXX
    };

    let host_name: Vec<u8> = ::cstr_to_slice(name).to_vec();
    HOST_ADDR_LIST = [host_addr.s_addr.as_ptr() as *const c_char, null()];
    HOST_ADDR = Some(host_addr);

    HOST_ENTRY = hostent {
        h_name: host_name.as_ptr() as *const c_char,
        h_aliases: HOST_ALIASES.as_ptr(),
        h_addrtype: ::socket::AF_INET,
        h_length: 4,
        h_addr_list: HOST_ADDR_LIST.as_ptr()
    };

    HOST_NAME = Some(host_name);

    Ok(&HOST_ENTRY as *const hostent)
});
