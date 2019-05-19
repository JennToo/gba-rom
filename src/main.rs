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
