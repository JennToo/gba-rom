use core::fmt::Write;

use gba::bios::cpu_fast_set;
use gba::io::background;
use gba::io::display;

use crate::include_bytes_aligned;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut console = prepare_console();

    console.writeln("PANIC!");
    if let Some(loc) = info.location() {
        console.write_str("File: ").unwrap();
        console.writeln(loc.file());
        console.write_str("Line: ").unwrap();
        write!(console, "{}", loc.line()).unwrap();
        console.writeln("");
    }

    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        console.write_str("Message: ").unwrap();
        if let Some(args) = info.message() {
            core::fmt::write(&mut console, *args).unwrap();
        } else {
            console.write_str(msg).unwrap();
        }
    } else {
        console.write_str("(No message)").unwrap();
    }

    loop {}
}


const FONT: &[u8] = include_bytes_aligned!("../target/font.bin");
const FONT_PAL: &[u8] = include_bytes_aligned!("../target/font.bin.pal");
const SCREEN_WIDTH_TILES: usize = 240 / 8;
const SCREEN_HEIGHT_TILES: usize = 160 / 8;
const BG_WIDTH_TILES: usize = 256 / 8;

struct Console {
    cursor_x: usize,
    cursor_y: usize,
}

impl Console {
    fn new() -> Console {
        Console {
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn writeln(&mut self, s: &str) {
        write_str(s, self.cursor_x, self.cursor_y);
        self.cursor_y += 1;
        self.cursor_x = 0;
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_str(s, self.cursor_x, self.cursor_y);
        self.cursor_x += s.len();
        Ok(())
    }
}

fn prepare_console() -> Console {
    display::spin_until_vblank();

    let ctrl = display::DisplayControlSetting::new()
        .with_mode(display::DisplayMode::Mode0)
        .with_bg0(true);
    display::set_display_control(ctrl);

    background::BG0CNT.write(
        background::BackgroundControlSetting::new()
            .with_bg_priority(0)
            .with_char_base_block(0)
            .with_screen_base_block(8)
            .with_mosaic(false)
            .with_is_8bpp(true),
    );

    background::BG0HOFS.write(0);
    background::BG0VOFS.write(0);

    unsafe {
        let font_dest = 0x600_0000 as *mut u32;
        cpu_fast_set(
            FONT.as_ptr() as *mut u32,
            font_dest,
            (FONT.len() / 4) as u32,
            false,
        );

        let font_pal_dest = 0x500_0000 as *mut u32;
        cpu_fast_set(
            FONT_PAL.as_ptr() as *mut u32,
            font_pal_dest,
            (FONT_PAL.len() / 4) as u32,
            false,
        );
    }

    for y in 0..SCREEN_HEIGHT_TILES {
        for x in 0..SCREEN_WIDTH_TILES {
            write_str(" ", x, y);
        }
    }

    Console::new()
}

fn write_str(s: &str, x: usize, y: usize) {
    unsafe {
        let base = 0x600_4000 as *mut u16;

        let offset_start = x + y * BG_WIDTH_TILES;
        for (i, b) in s.bytes().enumerate() {
            // TODO: Handle newlines eventually
            base.offset((offset_start + i) as isize)
                .write_volatile(b as u16);
        }
    }
}
