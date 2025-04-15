mod context;
pub use context::TrapContext;
use log::error;

use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Trap},
    stval, stvec,
    utvec::TrapMode,
};

use crate::syscall::syscall;

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, kernel killed it");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, kernel killed it");
            run_next_app();
        }

        _ => {
            panic!("Unsupported trap: {:?} {:?}", scause.cause(), stval)
        }
    }
    cx
}

fn run_next_app() {
    unimplemented!()
}
