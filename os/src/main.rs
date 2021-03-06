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

	// 动态内存分配测试
	use alloc::boxed::Box;
	use alloc::vec::Vec;
	let v = Box::new(5);
	assert_eq!(*v, 5);
	core::mem::drop(v);

	let mut vec = Vec::new();
	for i in 0..10000 {
		vec.push(i);
	}
	assert_eq!(vec.len(), 10000);
	for (i, value) in vec.into_iter().enumerate() {
		assert_eq!(value, i);
	}
	println!("heap test passed by Guo Yao");
	println!("{}", *memory::config::KERNEL_END_ADDRESS);
	for _ in 0..2 {
		let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
			Result::Ok(frame_tracker) => frame_tracker,
			Result::Err(err) => panic!("{}", err)
		};
		let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
			Result::Ok(frame_tracker) => frame_tracker,
			Result::Err(err) => panic!("{}", err)
		};
		println!("{} and {}", frame_0.address(), frame_1.address());
	}
	sbi::shutdown();
}


