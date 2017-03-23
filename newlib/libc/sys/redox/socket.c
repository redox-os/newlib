#include <stddef.h>
#include <sys/socket.h>


int connect(int socket, const struct sockaddr *address, socklen_t address_len) {
}

ssize_t recv(int socket, void *buffer, size_t length, int flags) {
}

ssize_t send(int socket, const void *buffer, size_t length, int flags) {
}

int socket(int domain, int type, int protocol) {
}

int bind(int socket, const struct sockaddr *address, socklen_t address_len) {
}

int setsockopt(int socket, int level, int option_name, const void *option_value, socklen_t option_len) {
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
}

ssize_t sendto(int socket, const void *message, size_t length,
               int flags, const struct sockaddr *dest_addr,
               socklen_t dest_len) {
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
}
