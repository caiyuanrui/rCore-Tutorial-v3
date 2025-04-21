use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

use super::TaskContext;

unsafe extern "C" {
    fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
