#ifndef _SYS_DIRENT_H
#define _SYS_DIRENT_H

#include <sys/types.h>

struct dirent {
    ino_t          d_ino;       /* inode number */
    off_t          d_off;       /* offset to the next dirent */
    unsigned short d_reclen;    /* length of this record */
    unsigned char  d_type;      /* type of file; not supported */
    char           d_name[4096]; /* filename */
};

typedef struct __dirstream DIR;

#define _DIRENT_HAVE_D_RECLEN 1
#define _DIRENT_HAVE_D_TYPE 1
#define _DIRENT_HAVE_D_OFF 1

#define	DT_UNKNOWN	 0
#define	DT_FIFO		 1
#define	DT_CHR		 2
#define	DT_DIR		 4
#define	DT_BLK		 6
#define	DT_REG		 8
#define	DT_LNK		10
#define	DT_SOCK		12

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


#endif
