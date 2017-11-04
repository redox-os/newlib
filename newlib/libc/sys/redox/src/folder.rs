use ::{c_int, c_char};
use syscall::{self, O_CLOEXEC, O_RDONLY, O_DIRECTORY};
use core::ptr::null;
use alloc::boxed::Box;
use ::file::PATH_MAX;

#[repr(C)]
pub struct dirent {
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
        ent: dirent { d_name: [0; PATH_MAX] },
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
