#include <sys/types.h>

typedef struct fsid { int32_t val[2]; } fsid_t; /*	file system id type */

     /*
      *	filesystem statistics
      */

     #define MFSNAMELEN	     16		     /*	length of type name including null */
     #define MNAMELEN	     88		     /*	size of	on/from	name bufs */
     #define STATFS_VERSION  0x20030518	     /*	current	version	number */

     struct statfs {
        __fsword_t f_type;    /* Type of filesystem (see below) */
        __fsword_t f_bsize;   /* Optimal transfer block size */
        fsblkcnt_t f_blocks;  /* Total data blocks in filesystem */
        fsblkcnt_t f_bfree;   /* Free blocks in filesystem */
        fsblkcnt_t f_bavail;  /* Free blocks available to unprivileged user */
        fsfilcnt_t f_files;   /* Total file nodes in filesystem */
        fsfilcnt_t f_ffree;   /* Free file nodes in filesystem */
        fsid_t f_fsid;    /* Filesystem ID */
        __fsword_t f_namelen; /* Maximum length of filenames */
        __fsword_t f_frsize;  /* Fragment size (since Linux 2.6) */
        __fsword_t f_flags;   /* Mount flags of filesystem  (since Linux 2.6.36) */
      //  __fsword_t f_spare[xxx]; /* Padding bytes reserved for future use */
     };
