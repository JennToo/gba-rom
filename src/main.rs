#![no_std]
#![feature(start)]

use gba::io::display;
use gba::palram;
use gba::vram::bitmap;
use gba::Color;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let ctrl = display::DisplayControlSetting::new()
        .with_mode(display::DisplayMode::Mode0)
        .with_bg0(true);
    display::set_display_control(ctrl);

    unsafe {
        let ptr = 0x600_0000 as *mut u8;

        for (i, b) in FONT.iter().enumerate() {
            ptr.offset(i as isize).write_volatile(*b);
        }
    }

    for i in 0..255 {
        palram::index_palram_bg_8bpp(i).write(Color::from_rgb(
            i as u16,
            (255 - i) as u16,
            i as u16 / 2,
        ));
    }

    loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}

const FONT: &[u8] = include_bytes!("../target/font.bin");
