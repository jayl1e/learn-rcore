#![no_std]
#![no_main]

mod console;
mod lang_items;
mod loader;
mod logging;
mod sbi;
mod sync;
mod timer;
mod task;
mod syscall;
mod trap;


use core::{arch::global_asm, slice};
use log::*;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[no_mangle]
#[allow(unreachable_code)]
fn rust_main() -> ! {
    clear_bss();
    logging::init();
    loader::init();
    trap::init();
    println!("[kernel] hello going to run apps");
    trace!("start loading");
    unsafe {
        loader::load_all_apps();
    }
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    trace!("start running");
    task::run_first_task();
    sbi::shut_down(false);
    panic!("should not run here");
}

fn clear_bss() {
    extern "C" {
        // use fn because we want to access there as pointer
        // simple usize will read data there
        fn sbss();
        fn ebss();
    }
    unsafe {
        slice::from_raw_parts_mut(
            sbss as usize as *mut u8, 
            ebss as usize- sbss as usize).fill(0);
    }
}
