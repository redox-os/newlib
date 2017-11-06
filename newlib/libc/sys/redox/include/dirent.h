#ifndef _DIRENT_H_
#define _DIRENT_H_
#ifdef __cplusplus
extern "C" {
#endif
#include <sys/cdefs.h>
#include <sys/dirent.h>

#if !defined(MAXNAMLEN) && __BSD_VISIBLE
#define MAXNAMLEN 1024
#endif

#ifdef __cplusplus
}
#endif

__BEGIN_DECLS
int closedir(DIR *);
DIR *opendir(const char *);
struct dirent *readdir(DIR *);
void rewinddir(DIR *);
int scandir(const char *dirp, struct dirent ***namelist,
              int (*filter)(const struct dirent *),
              int (*compar)(const struct dirent **, const struct dirent **));
int alphasort(const struct dirent **d1, const struct dirent **d2);
__END_DECLS

#endif /*_DIRENT_H_*/


