#ifndef _SYS_REDOX_H
#define _SYS_REDOX_H

#ifdef __cplusplus
extern "C" {
#endif

#include <_ansi.h>
#include <stddef.h>

int _EXFUN(redox_fpath, (int file, char *buf, size_t len));
void *_EXFUN(redox_fmap, (int file, size_t offset, size_t size));
int _EXFUN(redox_funmap, (void *addr));
void *_EXFUN(redox_physalloc, (size_t size));
int _EXFUN(redox_physfree, (void *physical_address, size_t size));
void *_EXFUN(redox_physmap, (void *physical_address, size_t size, int flags));
int _EXFUN(redox_physunmap, (void *virtual_address));
void *_EXFUN(redox_virttophys, (void *virtual_address));

#ifdef __cplusplus
}
#endif
#endif /* _SYS_REDOX_H */
