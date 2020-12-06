use alloc::string::{ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter, Text80x25, TextWriter};

const GRAPHICS: Graphics640x480x16 = Graphics640x480x16::new();

pub fn init_graphics() {
    GRAPHICS.set_mode();
    GRAPHICS.clear_screen(Color16::Black);
}

pub async fn hello_world() {
    write_txt("Hello, World!", 0, 0, Color16::White);
}

pub fn write_txt(s: &str, start_x: usize, start_y: usize, color: Color16) {
    for (offset, character) in s.chars().enumerate() {
        GRAPHICS.draw_character(start_x + offset * 8, start_y, character, color)
    }
}