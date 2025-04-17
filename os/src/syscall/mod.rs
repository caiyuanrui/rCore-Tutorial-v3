use fs::sys_write;
use lazy_static::lazy_static;
use process::{sys_exit, sys_get_taskinfo};

use crate::sync::UpSafeCell;

mod fs;
mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TASKINFO: usize = 1024;
const MAX_SYSCALL_ID: usize = 1024;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    let ret = match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_GET_TASKINFO => sys_get_taskinfo(),
        _ => unimplemented!("Unsupported sys_call id: {}", id),
    };
    unsafe { SYSCALL_MONITOR.exclusive_access() }.add(id);
    ret
}

/// 扩展内核，能够统计多个应用的执行过程中系统调用编号和访问此系统调用的次数
struct SyscallMonitor {
    count: [usize; MAX_SYSCALL_ID + 1],
}

impl SyscallMonitor {
    const fn new() -> Self {
        Self {
            count: [0; MAX_SYSCALL_ID + 1],
        }
    }
    fn add(&mut self, id: usize) {
        self.count[id] += 1;
    }
    fn get(&self, id: usize) -> usize {
        self.count[id]
    }
}

lazy_static! {
    static ref SYSCALL_MONITOR: UpSafeCell<SyscallMonitor> =
        unsafe { UpSafeCell::new(SyscallMonitor::new()) };
}

pub fn get_syscall_count(id: usize) -> usize {
    unsafe { SYSCALL_MONITOR.exclusive_access() }.get(id)
}
