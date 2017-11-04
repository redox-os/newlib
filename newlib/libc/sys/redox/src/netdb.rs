use libc;
use std::ffi::CStr;
use ::types::{socklen_t, in_addr, sockaddr, sockaddr_in};

struct hostent {
    h_name: std::ffi::CStr,
    h_aliases std::ffi::CStr*,
    h_addrtype: libc::c_int,
    h_length: libc::c_int,
    h_addr_list: std::ffi::CStr*,
};

struct netent {
	n_name: std::ffi::CStr,	/* official name of net */
	n_aliases: std::ffi::CStr*,	/* alias list */
	n_addrtype: libc::c_int,	/* net address type */
	n_net: libc::c_ulong,		/* network # */
};

struct servent {
	s_name: std::ffi::CStr,	/* official service name */
	s_aliases: std::ffi::CStr*,	/* alias list */
	s_port: libc::c_int,		/* port # */
	s_proto: std::ffi:CStr,	/* protocol to use */
};

struct protoent {
	p_name: std::ffi::CStr,	/* official protocol name */
	p_aliases: std::ffi::CStr*,	/* alias list */
	p_proto: libc::c_int,	/* protocol # */
};

struct addrinfo {
	ai_flags: libc::c_int,	/* AI_PASSIVE, AI_CANONNAME, AI_NUMERICHOST */
	ai_family: libc::c_int,	/* PF_xxx */
	ai_socktype: libc::c_int,	/* SOCK_xxx */
	ai_protocol: libc::c_int,	/* 0 or IPPROTO_xxx for IPv4 and IPv6 */
	ai_addrlen: libc::size_t,	/* length of ai_addr */
	ai_canonname: std::ffi:CStr;	/* canonical name for hostname */
    ai_addr: sockaddr*;	/* binary address */
	ai_next: addrinfo*;	/* next structure in linked list */
};

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

        let fd = ::RawFile::open(format!("udp:{}.{}.{}.{}:0",
                                         ip[0], ip[1], ip[2], ip[3]).as_bytes(),
                                 syscall::O_RDWR)?;

        let timeout = syscall::TimeSpec {
            tv_sec: 5,
            tv_nsec: 0,
        };
        let rt = fd.dup(b"read_timeout")?;
        syscall::write(*rt, &timeout)?;
        drop(rt);
        let wt = fd.dup(b"write_timeout")?;
        syscall::write(*wt, &timeout)?;
        drop(wt);

        let sendrecvfd = fd.dup(format!("{}.{}.{}.{}:53", dns[0], dns[1], dns[2], dns[3]).as_bytes())?;
        syscall::write(*sendrecvfd, &packet_data)?;
        let mut buf = [0; 65536];
        let count = syscall::read(*sendrecvfd, &mut buf)?;
        drop(sendrecvfd);
        drop(fd);

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

pub libc_fn!(endhostent(libc::c_void) -> libc::c_void {

});

pub libc_fn!(endhostent_r(file: libc::FILE**, i: libc::c_int*) -> libc::c_void {

});

pub libc_fn!(endnetent(libc::c_void) -> libc::c_void {

}); 

pub libc_fn!(endnetgrent(libc::c_void) -> libc::c_void {

}); 

pub libc_fn!(endprotoent(libc::c_void) -> libc::c_void {

}); 

pub libc_fn!(endservent(libc::c_void) -> libc::c_void {

});

pub libc_fn!(freehostent(mut hent: hostent*) -> libc::c_void {

});

pub libc_fn!(gethostbyaddr(libc::c_void*, length: socklen_t, format: libc::c_int) -> hostent* {
    
});

pub libc_fn!(unsafe gethostbyname(name: *const c_char) -> Result<*const hostent> {
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

pub libc_fn!(gethostbyname2(name: std::ffi::CStr, af: libc::c_int) -> hostent* {

});

pub libc_fn!(gethostent(libc::c_void) -> hostent* {

});

pub libc_fn!(gethostent_r(ret: hostent*, buf: std::ffi::CStr, buflen: libc::size_t, result: hostent**, h_errnop: libc::int*) -> libc::c_int {

});

pub libc_fn!(getipnodebyaddr(addr: libc::c_void*, type: libc::c_int, error_num: libc::c_int) -> hostent* {

});

pub libc_fn!(getipnodebyname(name: std::ffi::CStr, type: libc::c_int, flags: libc::c_int, error_num: libc::int*) -> hostent* {

});

pub libc_fn!(getnetbyaddr(net: libc::uint32_t, type: libc::c_int) -> netent* {

});

pub libc_fn!(getnetbyname(name: std::ffi::CStr) -> netent* {

});

pub libc_fn!(getnetent(libc::c_void) -> netent* {

});

pub libc_fn!(getnetgrent(host: std::ffi::CStr*, user: std::ffi::CStr*, domain: std::ffi::CStr*) -> libc::c_int {

}

pub libc_fn!(getprotobyname(name: std::ffi::CStr) -> protoent* {

});

pub libc_fn!(getprotobynumber(number: int) -> protoent* {

});

pub libc_fn!(getprotoent(libc::c_void) -> protoent* {

});

pub libc_fn!(getservbyname(name: std::ffi::CStr, proto: std::ffi::CStr) -> servent* {
    
});

pub libc_fn!(getservbyport(port: libc::c_int, proto: std::ffi::CStr) -> servent* {

});

pub libc_fn!(getservent(libc::c_void) -> servent* {

});

pub libc_fn!(herror(s: std::ffi:CStr) -> libc::c_void {

});

pub libc_fn!(hstrerror(err: libc::c_int*) -> std::ffi::CStr {

});

pub libc_fn!(innetgr(netgroup: std::ffi::CStr, host: std::ffi::CStr, user: std::ffi::CStr, domain: std::ffi::CStr) -> libc::c_int {

}

pub libc_fn!(sethostent(stayopen: libc::c_int) -> libc::c_void {

});

pub libc_fn!(sethostent_r(mut ret: hostent*, mut buf: std::ffi:CStr, buflen: size_t, mut result: hostent**, h_errnop: libc::c_int) -> libc::c_int {

});

pub libc_fn!(setnetent(stayopen: libc::c_int) -> libc::c_void {

});

pub libc_fn!(setprotoent(stayopen: libc::c_int) -> libc::c_void {

});

pub libc_fn!(getaddrinfo(node: std::ffi::CStr, service:std::ffi::CStr, hints: addrinfo*, mut res: addrinfo**) -> libc::c_int{

});

pub libc_fn!(getnameinfo(addr: sockaddr*, addrlen:socklen_t, host:std::ffi:CStr, hostlen: socklen_t, serv: std::ffi::CStr, servlen:socklen_t, flags: libc::c_int) -> libc::c_int {

});

pub libcfn!(freeaddrinfo(res: mut* addrinfo) -> libc::c_void {

});

pub libcfn!(gai_strerror(errcode: libc::c_int) -> std::ffi::CStr {

});

pub libcfn!(setnetgrent(netgroup: std::ffi::CStr) -> libc::c_int) {

});

pub libcfn!(setservent(stayopen: libc::c_int) -> libc::c_void {

});
