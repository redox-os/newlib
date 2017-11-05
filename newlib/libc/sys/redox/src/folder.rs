#![feature(pointer_methods)]
use ::{c_int, c_char};
extern crate std;
use syscall::{self, O_CLOEXEC, O_RDONLY, O_DIRECTORY};
use core::ptr::null;
use alloc::boxed::Box;
use alloc::raw_vec::RawVec;
use ::file::PATH_MAX;
use libc::*;
extern crate quicksort;

#[repr(C)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_off: off_t,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; PATH_MAX],
}

pub struct DIR {
    pub fd: ::RawFile,
    pub ent: dirent,
    pub buf: [u8; PATH_MAX],
    pub count: usize,
    pub pos: usize
}

libc_fn!(unsafe opendir(path: *mut c_char) -> Result<*mut DIR> {
    let path = ::cstr_to_slice(path);
    let fd = ::RawFile::open(path, O_RDONLY | O_CLOEXEC | O_DIRECTORY)?;
    let dir = Box::new(DIR {
        fd,
        ent: dirent {d_ino: 0, d_off: 0, d_reclen: 0, d_type: 0, d_name: [0; PATH_MAX] },
        buf: [0; PATH_MAX],
        count: 0,
        pos: 0

    });
    Ok(Box::into_raw(dir))
});

libc_fn!(unsafe readdir(dir: *mut DIR) -> Result<*const dirent> {
    if let Some(dir) = dir.as_mut() {
        let mut i = 0;
        'outer: while i < PATH_MAX - 1 {
            while dir.pos < dir.count {
                dir.ent.d_name[i] = dir.buf[dir.pos] as c_char;
                dir.pos += 1;
                if dir.buf[dir.pos-1] == b'\n' {
                    break 'outer;
                }
                i += 1;
            }
            dir.count = syscall::read(*dir.fd, &mut dir.buf)?;
            if dir.count == 0 {
                break;
            }
            dir.pos = 0;
        }
        if i != 0 {
            dir.ent.d_name[i] = 0;
            return Ok(&dir.ent);
        }
    }
    Ok(null())
});

libc_fn!(unsafe rewinddir(dir: *mut DIR) {
    if let Some(dir) = dir.as_mut() {
        dir.count = 0;
        let _ = syscall::lseek(*dir.fd, 0, syscall::SEEK_SET);
    }
});

libc_fn!(unsafe closedir(dir: *mut DIR) -> Result<c_int> {
    Box::from_raw(dir);
    Ok(0)
});

/*macro_rules! DIRSZ {
    ($dp:expr) => (sizeof(dirent) - sizeof($dp.d_name)) + ((($dp)->d_reclen + 1 + 3) &~ 3);
}*/


libc_fn!(unsafe scandir(dirname: *mut c_char, namelist: *mut *mut *mut dirent, filter: fn(*const dirent) -> Result<c_int, std::result::Result::Err>, compar: fn(*const *const dirent, *const *const dirent) -> Result<c_int, std::result::Result::Err>) -> Result<c_int, std::result::Result::Err> {
    let d: *mut dirent;
    let p: *mut dirent;
    let names: RawVec<*mut dirent>;
    let mut nitems: size_t;
    let mut stb: libc::stat;
    let mut arraysz: c_long;
    let mut dirp: DIR;

    dirp = opendir(dirname).read();
    if !dirp.is_ok() { 
        Err(-1);
    }
    if ::call::fstat(dirp.fd, &stb) < 0 {
        Err(-1);
    }
    
    /*
	 * estimate the array size by taking the size of the directory file
	 * and dividing it by a multiple of the minimum size entry. 
	 */
    arraysz = stb.st_size/24;
    names = RawVec::with_capacity(arraysz as usize);
    nitems = 0;
    while (d = readdir(dirp.as_mut())).is_ok()  { 
        if filter != None && !filter(d)
            { continue; } 
        //p = Box<dirent> = Box::new([0; DIRSZ(d)]);
        (*p).d_ino = (*d).d_ino;
        (*p).d_reclen = (*d).d_reclen;
        (*p).d_name = (*d).d_name;
        /*
        * Check to make sure the array has space left and
        * realloc the maximum size.
        */
        if nitems+1 >= arraysz as usize {
            if ::call::fstat(dirp.fd, &stb) < 0
                { Err(-1); }
            names.double()
        }
        names[nitems-1] = p;
    }
    closedir(dirp.as_mut());
    quicksort::quicksort_by(&names.ptr(), compar);
    *namelist = names.ptr();
    Ok(nitems as i32)
});

libc_fn!(unsafe alphasort(a: *mut c_void, b: *mut c_void) -> c_int {
    let a: &mut dirent = unsafe { &mut *(a as *mut dirent) }; 
    let b: &mut dirent = unsafe { &mut *(b as *mut dirent) };
    strcmp(a.d_name.as_ptr(), b.d_name.as_ptr())
});
