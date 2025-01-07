//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

use fs::{sys_chdir, sys_getcwd, sys_mkdirat, sys_read, sys_write};
use mm::sys_brk;

use crate::task::{
    sys_clone, sys_exec, sys_execve, sys_exit, sys_fork, sys_get_time, sys_getpid, sys_waitpid,
    sys_yield,
};

mod fs;
mod mm;

const SYSCALL_GETCWD: usize = 17;
const SYSCALL_MKDIRAT: usize = 34;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GITPID: usize = 172;
const SYSCALL_BRK: usize = 214;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAIT4: usize = 260;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall_perv(syscall_id: usize, args: [usize; 6]) -> isize {
    match syscall_id {
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

const CARELESS_SYSCALLS: [usize; 4] = [63, 64, 124, 260];

#[no_mangle]
pub fn syscall(
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    syscall_id: usize,
) -> isize {
    if !CARELESS_SYSCALLS.contains(&syscall_id) {
        log::info!("syscall_id: {}", syscall_id);
    }
    // if syscall_id == SYSCALL_WAIT4 {
    // log::warn!("syscall_id: {}", syscall_id);
    // }
    match syscall_id {
        SYSCALL_GETCWD => sys_getcwd(a0 as *mut u8, a1),
        SYSCALL_MKDIRAT => sys_mkdirat(a0 as isize, a1 as *const u8, a2),
        SYSCALL_CHDIR => sys_chdir(a0 as *const u8),
        SYSCALL_READ => sys_read(a0, a1 as *mut u8, a2),
        SYSCALL_WRITE => sys_write(a0, a1 as *const u8, a2),
        SYSCALL_EXIT => sys_exit(a0 as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_GITPID => sys_getpid(),
        SYSCALL_BRK => sys_brk(a0),
        SYSCALL_FORK => sys_clone(a0 as u32, a1, a2, a3, a4),
        SYSCALL_EXEC => sys_execve(a0 as *mut u8, a1 as *const usize, a2 as *const usize),
        SYSCALL_WAIT4 => sys_waitpid(a0 as isize, a1 as *mut i32),
        // SYSCALL_WAIT4 => sys_waitpid(a0 as isize, a1 as *mut i32, a2 as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
