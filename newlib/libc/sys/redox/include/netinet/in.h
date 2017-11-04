#ifndef _NETINET_IN_H
#define _NETINET_IN_H

#include <sys/types.h>
#include <stdint.h>

#define	IPPROTO_IP 0
// IPPROTO_IPV6
#define	IPPROTO_ICMP 1
#define	IPPROTO_TCP 6
#define	IPPROTO_UDP 17
#define IPPORT_RESERVED 1024

#define INADDR_ANY ((in_addr_t) 0x00000000)
#define INADDR_BROADCAST ((in_addr_t) 0xffffffff)
#define INADDR_NONE ((in_addr_t) 0xffffffff)
#define INADDR_LOOPBACK ((in_addr_t) 0x7f000001) // 127.0.0.1

typedef uint16_t sa_family_t;
typedef uint32_t in_addr_t;
typedef uint16_t in_port_t;

struct in_addr {
    in_addr_t s_addr;
};

struct sockaddr_in {
    sa_family_t sin_family;
    in_port_t sin_port;
    struct in_addr sin_addr;
    unsigned char __pad[8];
};

#endif
