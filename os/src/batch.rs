use core::arch::asm;

use lazy_static::lazy_static;
use log::{error, info};

use crate::{sbi::shutdown, sync::UpSafeCell, syscall::get_syscall_count, trap::TrapContext};

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
            cx_ptr.as_mut().unwrap()
        }
    }
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

static USER_STACK: UserStack = UserStack {
    data: [0; KERNEL_STACK_SIZE],
};

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
    start_tick: [usize; MAX_APP_NUM + 1],
    end_tick: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}] [{:#x}]",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    fn load_app(&self, id: usize) {
        if id > self.num_app {
            info!("[kernel] All applications completed!");
            self.statistic();
            shutdown(false)
        }
        info!("[kernel] Loading app_{}", id);
        unsafe {
            // Clear app area
            core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
            let app_src = core::slice::from_raw_parts(
                self.app_start[id] as *const u8,
                self.app_start[id + 1] - self.app_start[id],
            );
            let app_dst =
                core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
            app_dst.copy_from_slice(app_src);
            // Flush instruction cache
            // See also: riscv non-priv spec chapter 3, 'Zifencei' extension.
            asm!("fence.i");
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    pub fn get_app_tick(&self, id: usize) -> usize {
        if id > self.num_app {
            error!("[kernel] app_{id} doesn't exist");
            return usize::MAX;
        }
        self.end_tick[id] - self.start_tick[id]
    }

    pub fn get_current_app_range(&self) -> (usize, usize) {
        (
            APP_BASE_ADDRESS,
            APP_BASE_ADDRESS + self.app_start[self.current_app]
                - self.app_start[self.current_app - 1],
        )
    }

    /// Print statistic info.
    fn statistic(&self) {
        const QEMU_CLOCK_FREQ: usize = 10_000_000; // 10_000_000 Hz
        // analyze the number of syscalls
        info!("SYSCALL_WRITE is called {} times", get_syscall_count(64));
        info!("SYSCALL_EXIT is called {} times", get_syscall_count(96));
        info!(
            "SYSCALL_GET_TASKINFO is called {} times",
            get_syscall_count(1024)
        );
        // analyze the completion ticks of each app
        for i in 0..self.num_app {
            let ticks = self.get_app_tick(i);
            info!("app_{i} costed {}us", ticks / (QEMU_CLOCK_FREQ / 1_000_000));
        }
    }
}

lazy_static! {
    static ref APP_MANAGER: UpSafeCell<AppManager> = unsafe {
        UpSafeCell::new({
            unsafe extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            let start_tick = [0; MAX_APP_NUM + 1];
            let end_tick = [0; MAX_APP_NUM + 1];
            AppManager {
                num_app,
                current_app: 0,
                app_start,
                start_tick,
                end_tick,
            }
        })
    };
}

/// init batch system
pub fn init() {
    print_app_info();
}

pub fn print_app_info() {
    unsafe { APP_MANAGER.exclusive_access() }.print_app_info();
}

pub fn get_current_app() -> usize {
    unsafe { APP_MANAGER.exclusive_access() }.get_current_app()
}

pub fn get_current_app_range() -> (usize, usize) {
    unsafe { APP_MANAGER.exclusive_access() }.get_current_app_range()
}

pub fn get_user_stack_range() -> (usize, usize) {
    let sp = USER_STACK.get_sp();
    (sp - USER_STACK_SIZE, sp)
}

pub fn run_next_app() -> ! {
    let app_manager = unsafe { APP_MANAGER.exclusive_access() };
    let current_app = app_manager.get_current_app();
    let current_tick = riscv::register::time::read();
    if current_app > 0 {
        app_manager.end_tick[current_app - 1] = current_tick;
    }
    app_manager.start_tick[current_app] = current_tick;
    app_manager.load_app(current_app);
    app_manager.move_to_next_app();
    unsafe extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    unreachable!("Unreachable in batch::run_current_app!")
}
