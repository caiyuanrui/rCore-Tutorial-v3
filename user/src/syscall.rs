use core::arch::asm;

/// RISC-V 寄存器编号从 0~31 ，表示为 x0~x31 。 其中
/// x10~x17 对应 a0~a7,
/// x1 对应 ra。
///
/// 约定寄存器 a0~a6 保存系统调用的参数， a0 保存系统调用的返回值。有些许不同的是寄存器 a7 用来传递 syscall ID
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
          "ecall",
          inlateout("x10") args[0] => ret,
          in("x11") args[1],
          in("x12") args[2],
          in("x17") id,
          options(nostack)
        )
    }
    ret
}

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TASKINFO: usize = 114514;

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_get_taskinfo() -> isize {
    syscall(SYSCALL_GET_TASKINFO, [0, 0, 0])
}
