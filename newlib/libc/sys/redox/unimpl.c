#include "common.h"

#include <stdio.h>

#define UNIMPL(error) { \
    errno = error; \
    fprintf(stderr, "unimplemented: %s: %s\n", __func__, strerror(error)); \
    return -1; \
}

unsigned int alarm(unsigned int seconds) {
    fprintf(stderr, "unimplemented: alarm");
    return 0;
}

int _chmod(const char * path, mode_t mode) {
    UNIMPL(EACCES);
}

int _chown(const char *path, uid_t owner, gid_t group) {
    UNIMPL(EACCES);
}

int _fcntl(int file, int cmd, ...){
    UNIMPL(EACCES);
}

int _getdtablesize() {
    return 65536;
}

struct group * _getgrnam(const char * name){
    return (struct group *) NULL;
}

struct group * _getgrgid(gid_t gid){
    return (struct group *) NULL;
}

struct passwd * _getpwnam(const char * name){
    return (struct passwd *) NULL;
}

struct passwd * _getpwuid(uid_t uid){
    return (struct passwd *) NULL;
}

int _ioctl(int file, int request, ...) {
    UNIMPL(EINVAL);
}

//TODO: Actually implement lstat, it currently just calls stat
int lstat(const char *__restrict path, struct stat *__restrict sbuf) {
    return _stat(path, sbuf);
}

int _link(const char *old, const char *new) {
    UNIMPL(EPERM);
}

long sysconf(int name) {
    UNIMPL(EINVAL);
}

clock_t _times(struct tms * buf) {
    UNIMPL(EINVAL);
}

mode_t _umask(mode_t mask) {
    //All permissions granted
    return 0777;
}

int _utime(const char * filename, const struct utimbuf * times) {
    UNIMPL(EACCES);
}

int vfork() {
    return _fork();
}
