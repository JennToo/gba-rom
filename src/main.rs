#![no_std]
#![feature(start)]
#![feature(panic_info_message)]

mod panic;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    panic!("Hello there!");
    loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}

#[repr(align(4))]
pub struct AlignmentWrapper<T>(pub T);

#[macro_export]
macro_rules! include_bytes_aligned {
    ($path:expr) => {
        &crate::AlignmentWrapper(*include_bytes!($path)).0
    }
}
