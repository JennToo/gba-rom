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
    display::spin_until_vblank();

    unsafe {
        let ptr = 0x600_0000 as *mut u16;

        for i in 0..FONT.len() / 2 {
            let l = FONT[2 * i] as u16;
            let h = FONT[2 * i + 1] as u16;
            let v = (h << 8) | l;
            ptr.offset(i as isize).write_volatile(v);
        }
    }
    unsafe {
        let ptr = 0x500_0000 as *mut u16;

        for i in 0..FONT_PAL.len() / 2 {
            let l = FONT_PAL[2 * i] as u16;
            let h = FONT_PAL[2 * i + 1] as u16;
            let v = (h << 8) | l;
            ptr.offset(i as isize).write_volatile(v);
        }
    }

    loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}

const FONT: &[u8] = include_bytes!("../target/font.bin");
const FONT_PAL: &[u8] = include_bytes!("../target/font-pal.bin");
