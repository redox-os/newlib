#ifndef _NETDB_H
#define _NETDB_H

#include <sys/types.h>
#include <sys/socket.h>

struct hostent {
    char *h_name;
    char **h_aliases;
    int h_addrtype;
    int h_length;
    char **h_addr_list;
};

struct netent {
    char *n_name;
    char **n_aliases;
    int n_addrtype;
    unsigned long n_net;
};

struct servent {
    char *s_name;
    char **s_aliases;
    int s_port;
    char *s_proto;
};

struct protoent {
    char *p_name;
    char **p_aliases;
    int p_proto;
};

struct addrinfo {
    int ai_flags;
    int ai_family;
    int ai_socktype;
    int ai_protocol;
    size_t ai_addrlen;
    char *ai_canonname;
    struct sockaddr *ai_addr;
    struct addrinfo *ai_next;
};

struct hostent *gethostbyname(const char *name);
struct hostent *gethostbyaddr(const void *v, socklen_t length, int format);
struct hostent *gethostent();
struct protoent *getprotobyname(const char *name);
struct protoent *getprotobynumber(int number);
struct protoent *getprotoent();
struct servent *getservbyname(const char *name, const char *proto);
struct servent *getservbyport(int port, const char* proto);
struct servent *getservent();
void sethostent(int stayopen);
void setnetent(int stayoopen);
void setprotoent(int stayopen);
void setservent(int stayopen);
void endhostent();
void endnetent();
void endprotoent();
void endservent();



#define h_addr h_addr_list[0]

#endif
