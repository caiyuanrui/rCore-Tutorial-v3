use crate::batch::{get_current_app, print_app_info, run_next_app};

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    run_next_app()
}

pub fn sys_get_taskinfo() -> isize {
    print_app_info();
    get_current_app() as isize
}
