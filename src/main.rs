#![no_std]
#![feature(start)]

use gba::io::display;
use gba::vram::bitmap;
use gba::Color;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let ctrl = display::DisplayControlSetting::new()
        .with_mode(display::DisplayMode::Mode3)
        .with_bg2(true);
    display::set_display_control(ctrl);
    bitmap::Mode3::write_pixel(120, 80, Color::from_rgb(31, 0, 0));
    bitmap::Mode3::write_pixel(136, 80, Color::from_rgb(0, 31, 0));
    bitmap::Mode3::write_pixel(120, 96, Color::from_rgb(0, 0, 31));

    loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}
