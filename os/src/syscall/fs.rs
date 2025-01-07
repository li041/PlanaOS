use core::{arch::asm, ptr::addr_eq};

use alloc::string::ToString;

use crate::{
    fs::{create_dir, open_file, path::Path, OpenFlags, AT_FDCWD},
    mm::copy_to_user,
    sbi::console_getchar,
    task::{current_task, sys_yield, yield_current_task},
    utils::c_str_to_string,
};

const FD_STDOUT: usize = 1;
const FD_STDIN: usize = 0;

pub fn sys_read(fd: usize, buf: *mut u8, len: usize) -> isize {
    let task = current_task();
    /* cannot use `inner` as MutexGuard will cross `await` that way */
    let fd_table_len = task.inner_handler(|inner| inner.fd_table.len());
    if fd >= fd_table_len {
        return -1;
    }
    let file = task.inner_handler(|inner| inner.fd_table[fd].clone());
    if let Some(file) = file {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        let ret = file.read(unsafe { core::slice::from_raw_parts_mut(buf, len) });
        ret as isize
    } else {
        -1
    }
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let task = current_task();
    let fd_table_len = task.inner_handler(|inner| inner.fd_table.len());
    if fd >= fd_table_len {
        return -1;
    }
    let file = task.inner_handler(|inner| inner.fd_table[fd].clone());
    if let Some(file) = file {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        let ret = file.write(unsafe { core::slice::from_raw_parts(buf as *const u8, len) });
        ret as isize
    } else {
        -1
    }
}

/// 由copy_to_user保证用户指针的合法性
pub fn sys_getcwd(buf: *mut u8, buf_size: usize) -> isize {
    // glibc getcwd(3) says that if buf is NULL, it will allocate a buffer
    let cwd = current_task().inner.lock().cwd.clone();
    let cwd_str = cwd.to_string();
    let copy_len = cwd_str.len() + 1;
    if copy_len > buf_size {
        log::error!("getcwd: buffer is too small");
        // buf太小返回NULL
        return 0;
    }
    let from: *const u8 = cwd_str.as_bytes().as_ptr();
    // 若出错, 打印错误信息
    // if copy_to_user(buf, from, copy_len).is_err() {
    //     log::error!("getcwd: copy_to_user failed");
    //     return 0;
    // }
    if let Err(err) = copy_to_user(buf, from, copy_len) {
        log::error!("getcwd: copy_to_user failed: {}", err);
        return 0;
    }
    // 成功返回buf指针
    buf as isize
}

pub fn sys_mkdirat(dirfd: isize, pathname: *const u8, _mode: usize) -> isize {
    let path = Path::from(c_str_to_string(pathname));
    create_dir(dirfd, &path) as isize
}

pub fn sys_chdir(pathname: *const u8) -> isize {
    let path = Path::from(c_str_to_string(pathname));
    // simply examine validity of the path
    match open_file(AT_FDCWD, &path, OpenFlags::empty()) {
        Ok(inode) => {
            current_task().inner.lock().cwd = inode.get_path();
            0
        }
        Err(_) => -1,
    }
}
