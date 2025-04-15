use fs::sys_write;
use process::sys_exit;

mod fs;
mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => unimplemented!("Unsupported sys_call id: {}", id),
    }
}
