#include <stddef.h>
#include <stdio.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <assert.h>
#include <string.h>
#include <inttypes.h>
#include <netdb.h>

#include "common.h"

#define DUP(file, buf) syscall3(SYS_DUP, (uint64_t)file, buf, strlen(buf))

static struct hostent _host_entry;
static char *_h_addr_list[2];
static char _h_addr[4];
static char* _h_aliases[1];
static char _h_name[16];

static int _parse_ip(const char *addr, unsigned char* ip) {
    if (sscanf(addr, "%hhu.%hhu.%hhu.%hhu", &ip[0], &ip[1], &ip[2], &ip[3]) != 4)
        return -1;
    return 0;
}

static char* addr_to_str(const struct sockaddr_in* insock) {
    unsigned char *addr = (unsigned char*)(&insock->sin_addr.s_addr);
    char **str;
    asprintf(str, "%hhu.%hhu.%hhu.%hhu:" SCNu16, addr[0], addr[1], addr[2], addr[3], insock->sin_port);
    return *str;
}

int socket(int domain, int type, int protocol) {
    assert(domain == AF_INET);
    assert(protocol == 0); // XXX These asserts should be removed
    assert(type == SOCK_STREAM || type == SOCK_DGRAM);

    if (type == SOCK_STREAM)
        return open("tcp:", O_RDONLY);
    else if (type == SOCK_DGRAM)
        return open("udp:", O_RDONLY);
}

int connect(int socket, const struct sockaddr *address, socklen_t address_len) {
    //XXX
    char *addr = addr_to_str((struct sockaddr_in*)address);

    int fd = DUP(socket, addr);
    free(addr);
    if (fd == -1)
        return -1;
    if (dup2(fd, socket) == -1)
        return -1;

    return 0;
}

int setsockopt(int socket, int level, int option_name, const void *option_value, socklen_t option_len) {
}

ssize_t recv(int socket, void *buffer, size_t length, int flags) {
    assert(flags == 0);
    return read(socket, buffer, length);
}

ssize_t send(int socket, const void *buffer, size_t length, int flags) {
    assert(flags == 0);
    return write(socket, buffer, length);
}

int bind(int socket, const struct sockaddr *address, socklen_t address_len) {
    //O_RDWR
}

int getsockname(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len) {
}

int getpeername(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len) {
}

int listen(int socket, int backlog) {
}

int accept(int socket, struct sockaddr *restrict address, socklen_t *restrict address_len) {
}

ssize_t recvfrom(int socket, void *restrict buffer, size_t length,
                 int flags, struct sockaddr *restrict address,
                 socklen_t *restrict address_len) {
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
    //XXX
    char *addr = addr_to_str((struct sockaddr_in*)dest_addr);

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
    //TODO: handle domain names

    if (_parse_ip(name, _h_addr) == -1)
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
