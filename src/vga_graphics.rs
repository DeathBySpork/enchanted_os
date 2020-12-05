use alloc::string::{ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter, Text80x25, TextWriter};

const graphics: Graphics640x480x16 = Graphics640x480x16::new();

pub async fn hello_world() {
    graphics.set_mode();
    graphics.clear_screen(Color16::Black);
    for (offset, character) in "Hello World!".chars().enumerate() {
        graphics.draw_character(270 + offset * 8, 72, character, Color16::White)
    }
    graphics.draw_line((0, 0), (50, 50), Color16::Cyan);
}