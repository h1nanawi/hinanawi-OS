#![no_std]

#![no_main]

#![feature(llvm_asm)]

#![feature(global_asm)]

#![feature(panic_info_message)]

#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod memory;
mod interrupt;


extern crate alloc;


global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
	 // 初始化各种模块
    interrupt::init();
    memory::init();

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped by Guo Yao");

	sbi::shutdown();
}


