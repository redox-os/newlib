#include "common.h"

int access(const char * path, int amode){
    int fd = _open(path, O_CLOEXEC | O_STAT, 0);
    if(fd < 0){
        return fd;
    }
    _close(fd);
    return 0;
}

int _close(int file){
    return syscall1(SYS_CLOSE, (uint64_t)file);
}

int dup(int file){
    return syscall3(SYS_DUP, (uint64_t)file, 0, 0);
}

int dup2(int file, int newfile){
    return syscall4(SYS_DUP2, (uint64_t)file, (uint64_t)newfile, 0, 0);
}

int _fpath(int file, char * buf, int len) {
    return syscall2(SYS_FPATH, (uint64_t)buf, (uint64_t)len);
}

int _fstat(int file, struct stat *st) {
    return syscall3(SYS_FSTAT, (uint64_t)file, (uint64_t)st, sizeof(struct stat));
}

int _fsync(int file) {
    return syscall1(SYS_FSYNC, (uint64_t)file);
}

int _ftruncate(int file, off_t len){
    return syscall2(SYS_FTRUNCATE, (uint64_t)file, (uint64_t)len);
}

int _lseek(int file, int ptr, int dir) {
    return syscall3(SYS_LSEEK, (uint64_t)file, (uint64_t)ptr, (uint64_t)dir);
}

int mkdir(const char * path, mode_t mode) {
    printf("open(%s)\n", path);
    int fd = _open(path, O_CREAT | O_EXCL | O_CLOEXEC | O_DIRECTORY, mode);
    if(fd < 0){
        return fd;
    }
    _close(fd);
    return 0;
}

int _open(const char *path, int flags, mode_t mode) {
    return syscall3(SYS_OPEN, (uint64_t)path, (uint64_t)strlen(path), (uint64_t)flags | (uint64_t)(mode & 0777));
}

int pipe(int pipefd[2]) {
    return pipe2(pipefd, 0);
}

int pipe2(int pipefd[2], int flags) {
    uint64_t syspipefd[2];
    syspipefd[0] = (uint64_t)pipefd[0];
    syspipefd[1] = (uint64_t)pipefd[1];
    int ret = syscall2(SYS_PIPE2, (uint64_t)syspipefd, (uint64_t)flags);
    pipefd[0] = (int)syspipefd[0];
    pipefd[1] = (int)syspipefd[1];
    return ret;
}

int _read(int file, char *ptr, int len) {
    return syscall3(SYS_READ, (uint64_t)file, (uint64_t)ptr, (uint64_t)len);
}

int rmdir(const char * path){
    return syscall2(SYS_RMDIR, (uint64_t)path, (uint64_t)strlen(path));
}

int _stat(const char *__restrict path, struct stat *__restrict sbuf) {
    int fd = _open(path, O_CLOEXEC | O_STAT, 0);
    if(fd < 0){
        return fd;
    }
    int ret = _fstat(fd, sbuf);
    int err = errno;
    _close(fd);
    errno = err;
    return ret;
}

int _unlink(const char *path) {
    return syscall2(SYS_UNLINK, (uint64_t)path, (uint64_t)strlen(path));
}

int _write(int file, const char *ptr, int len) {
    return syscall3(SYS_WRITE, (uint64_t)file, (uint64_t)ptr, (uint64_t)len);
}

int chmod(const char * path, mode_t mode) {
    return syscall3(SYS_CHMOD, (uint64_t)path, (uint64_t)strlen(path), (uint64_t)mode);
}

int ftruncate(int file, off_t len) {
    return syscall2(SYS_FTRUNCATE, (uint64_t)file, (uint64_t)len);
}
