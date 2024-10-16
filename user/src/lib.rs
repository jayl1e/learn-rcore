#![no_std]
#![feature(linkage)]

use syscall::{sys_get_time, sys_yield};

mod syscall;
pub mod console;
mod lang_items;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start()->!{
    clear_bss();
    exit(main());
    unreachable!("should exit after main")
}

fn clear_bss(){
    extern "C"{
        // use fn because we want to access there as pointer
        // simple usize will read data there
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as *mut u8,
            ebss as usize - sbss as usize )
            .fill(0);
    }
}

#[linkage = "weak"]
#[no_mangle]
fn main()->i32{
    panic!("can not find main")
}

pub fn write(fd: usize, buf: &[u8]) -> isize { syscall::sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { syscall::sys_exit(exit_code) }
pub fn get_task_info(name_buf: &mut [u8]) -> Option<&str> {
    let l = syscall::sys_get_task_info(name_buf);
    if l<0{
        return None
    }
    let name = core::str::from_utf8(&name_buf[..(l as usize)]).unwrap();
    Some(name)
}

pub fn yield_()->isize{sys_yield()}

pub fn get_time()->isize{
    sys_get_time()
}