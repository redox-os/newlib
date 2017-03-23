#ifndef _SYS_SOCKET_H
#define _SYS_SOCKET_H
#include <sys/types.h>
#include <stdint.h>

typedef size_t socklen_t;
typedef uint16_t sa_family_t;
typedef uint16_t in_port_t;
typedef uint32_t in_addr_t;

#define AF_UNSPEC 0
#define AF_INET 2

#define SOCK_STREAM 1
#define SOCK_DGRAM 2

#define	SO_KEEPALIVE 0x0008

#define	SOL_SOCKET 0xffff

#define	IPPROTO_IP 0
#define	IPPROTO_ICMP 1
#define	IPPROTO_TCP 6
#define	IPPROTO_UDP 17

struct sockaddr {
    sa_family_t sa_family;
    char sa_data[14];
};

struct in_addr {
    in_addr_t s_addr;
};

struct sockaddr_in {
    sa_family_t sin_family;
    in_port_t sin_port;
    struct in_addr sin_addr;
    unsigned char __pad[8];
};



int connect(int socket, const struct sockaddr *address, socklen_t address_len);
ssize_t recv(int socket, void *buffer, size_t length, int flags);
ssize_t send(int socket, const void *buffer, size_t length, int flags);
struct hostent *gethostbyname(const char *name);
int socket(int domain, int type, int protocol);
int bind(int socket, const struct sockaddr *address, socklen_t address_len);
int setsockopt(int socket, int level, int option_name, const void *option_value, socklen_t option_len);
int getsockname(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len);
int getpeername(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len);
int listen(int socket, int backlog);
int accept(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len);
ssize_t recvfrom(int socket, void *restrict buffer, size_t length,
                 int flags, struct sockaddr *restrict address,
                 socklen_t *restrict address_len);
ssize_t sendto(int socket, const void *message, size_t length,
           int flags, const struct sockaddr *dest_addr,
           socklen_t dest_len);
uint32_t htonl(uint32_t hostlong);
uint16_t htons(uint16_t hostshort);
uint32_t ntohl(uint32_t netlong);
uint16_t ntohs(uint16_t netshort);
#endif
