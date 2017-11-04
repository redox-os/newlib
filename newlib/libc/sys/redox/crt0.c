#include <stdint.h>
#include <stdlib.h>
#include <string.h>

extern char ** environ;
extern int main(int argc, char ** argv, char ** envp);
void __libc_init_array(void);
void initialize_standard_library(void);

__attribute__((naked)) void _start() {
    asm volatile(
        ".intel_syntax noprefix;"
        "xchg bx, bx;"
        "add rsp, 0x8;" //Undo GCC's alignment
        "mov rdi, rsp;"
        "and rsp, 0xFFFFFFFFFFFFFFF0;"
        "call _start_stack;"
        ".att_syntax prefix;"
        : : : "memory");
    exit(0);
}

struct slice {
    uint64_t len;
    char * ptr;
} __attribute__((packed));

void _start_stack(uint64_t * stack) {
    int argc = (int)(stack[0]);
    struct slice * rust_argv = (struct slice *)(&stack[1]);

    char ** argv = calloc(argc + 1, sizeof(char *));
    int i;
    for(i = 0; i < argc; i++){
        struct slice rust_arg = rust_argv[i];
        char * arg = calloc(rust_arg.len + 1, 1);
        memcpy(arg, rust_arg.ptr, rust_arg.len);
        argv[i] = arg;
    }
    initialize_standard_library();
    __libc_init_array();
    exit(main(argc, argv, environ));
}
