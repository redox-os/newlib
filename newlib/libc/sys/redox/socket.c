#include <stddef.h>
#include <stdio.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <assert.h>
#include <string.h>
#include <inttypes.h>
#include <netdb.h>

#include "common.h"

#define DUP(file, buf) syscall3(SYS_DUP, (uint64_t)file, (uint64_t)buf, (uint64_t)strlen(buf))

static struct hostent _host_entry;
static char *_h_addr_list[2];
static char _h_addr[4];
static char* _h_aliases[1];
static char _h_name[16];
static char _ntoa_addr[16];

int inet_aton(const char *cp, struct in_addr *inp) {
    // XXX handle variants with less than 3 .'s
    unsigned char *ip = &inp->s_addr;
    if (sscanf(cp, "%hhu.%hhu.%hhu.%hhu", &ip[0], &ip[1], &ip[2], &ip[3]) != 4)
        return 0;
    return 1;
}

char *inet_ntoa(struct in_addr in) {
    unsigned char *addr = (unsigned char*)(&in.s_addr);
    sprintf(&_ntoa_addr, "%hhu.%hhu.%hhu.%hhu",
            addr[0], addr[1], addr[2], addr[3]);
    return &_ntoa_addr;
}

int socket(int domain, int type, int protocol) {
    printf( "socket(%d, %d, %d)\n", domain, type, protocol);
    assert(domain == AF_INET);
    assert(protocol == 0); // XXX These asserts should be removed
    assert(type == SOCK_STREAM || type == SOCK_DGRAM);

    int ret = 42;
    perror("open");
    if (type == SOCK_STREAM) {
	printf("TCP\n");
        ret = open("tcp:", O_RDWR);
    }
    else if (type == SOCK_DGRAM) {
	printf("UDP\n");
        ret = open("udp:", O_RDWR);
    }
    printf("ret: %d\n", ret);
    if (ret == -1) {
	perror("open");
    }
    return ret;
}

int connect(int socket, const struct sockaddr *address, socklen_t address_len) {
    // XXX with UDP, should recieve messages only from that peer after this
    // XXX errno
    printf("connect()\n");
    //XXX
    assert(address->sa_family == AF_INET);
    char *addr = inet_ntoa(((struct sockaddr_in*)address)->sin_addr);
    char *path = malloc(22);
    sprintf(path, "%s:%d", addr, ntohs(((struct sockaddr_in*)address)->sin_port));

    printf("path: %s\n", path);
    printf("DUP\n");
    int fd = DUP(socket, path);
    free(path);
    if (fd == -1)
        return -1;
    printf("dup2\n");
    if (dup2(fd, socket) == -1) {
        close(fd);
        return -1;
    }
    close(fd);

    return 0;
}

int setsockopt(int socket, int level, int option_name, const void *option_value, socklen_t option_len) {
    printf("setsockopt() NOT IMPLEMENTED\n");
}

ssize_t recv(int socket, void *buffer, size_t length, int flags) {
    printf("recv()\n");
    assert(flags == 0);
    return read(socket, buffer, length);
}

ssize_t send(int socket, const void *buffer, size_t length, int flags) {
    printf("send()\n");
    assert(flags == 0);
    return write(socket, buffer, length);
}

int bind(int socket, const struct sockaddr *address, socklen_t address_len) {
    //O_RDWR
    printf("bind() NOT IMPLEMENTED\n");
}

int getsockname(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len) {
    printf("getsockname() NOT IMPLEMENTED\n");
}

int getpeername(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len) {
    printf("getpeername() NOT IMPLEMENTED\n");
}

int listen(int socket, int backlog) {
    printf("listen() NOT IMPLEMENTED\n");
}

int accept(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len) {
    printf("accept() NOT IMPLEMENTED\n");
    //getpeername(socket, address, address_len)
}

ssize_t recvfrom(int socket, void *restrict buffer, size_t length,
                 int flags, struct sockaddr *restrict address,
                 socklen_t *restrict address_len) {
    printf("recvfrom()\n");
    int fd = DUP(socket, "listen");
    if (fd == -1)
        return -1; 
    char path[4096];
    _fpath(socket, path, sizeof(path));
    // XXX process path and write to address
    ssize_t ret = recv(socket, buffer, length, flags);
    close(fd);
    return ret;
}

ssize_t sendto(int socket, const void *message, size_t length,
               int flags, const struct sockaddr *dest_addr,
               socklen_t dest_len) {
    printf("sendto()\n");
    //XXX
    assert(dest_addr->sa_family == AF_INET);
    char *addr = inet_ntoa(((struct sockaddr_in*)dest_addr)->sin_addr);

    int fd = DUP(socket, addr);
    free(addr);
    if (fd == -1)
        ; // XXX
    ssize_t ret = send(socket, message, length, flags);
    close(fd);
    return ret;
}


// Coppied from ../pheonix/include/netinet/in.h
// XXX Must be modified if Redox is ported to a big endian architecture
uint32_t htonl(uint32_t hostlong) {
    return ((hostlong & 0xff) << 24) | ((hostlong & 0xff00) << 8) | ((hostlong & 0xff0000UL) >> 8) | ((hostlong & 0xff000000UL) >> 24);
}

uint16_t htons(uint16_t hostshort) {
    return ((hostshort & 0xff) << 8) | ((hostshort & 0xff00) >> 8);
}

uint32_t ntohl(uint32_t netlong) {
    return htonl(netlong);
}

uint16_t ntohs(uint16_t netshort) {
    return htons(netshort);
}


struct hostent *gethostbyname(const char *name) {
    printf("gethostbyname()\n");
    //TODO: handle domain names

    if (inet_aton(name, (struct in_addr*)(&_h_addr)) == 0)
        return NULL;

    _h_addr_list[0] = _h_addr;
    _h_addr_list[1] = NULL;
    _h_aliases[0] = NULL;
    strncpy(_h_name, name, 16);

    _host_entry.h_name = _h_name; // XXX
    _host_entry.h_aliases = _h_aliases;
    _host_entry.h_addrtype = AF_INET;
    _host_entry.h_length = 4;
    _host_entry.h_addr_list = _h_addr_list;
    return &_host_entry;
}
