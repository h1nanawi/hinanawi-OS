pub mod heap;
pub mod config;
pub mod address;
pub mod frame;
pub mod range;

pub type MemoryResult<T> = Result<T, &'static str>;

pub use {address::*, config::*, frame::FRAME_ALLOCATOR, range::Range};

/// 初始化内存相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
}
