#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(const_in_array_repeat_expressions)]
#![feature(wake_trait)]
#![feature(asm)]
#![feature(c_variadic)]

pub mod interrupts;
pub mod allocator;
pub mod gdt;
pub mod task;
pub mod memory;
pub mod vga_graphics;

extern crate alloc;
extern crate rlibc;

use core::panic::PanicInfo;

use x86_64::instructions::port::Port;

// ================= HEAP ALLOCATION

pub const HEAP_START: usize = 0x_4444_4444_0000; // TODO: change the location to find available space
pub const HEAP_SIZE_KB: usize = 1000;

pub const HEAP_SIZE: usize = HEAP_SIZE_KB * 1024;

// ================= INITIALIZATION

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    vga_graphics::init_graphics();
}

// ================= ALLOCATION ERROR HANDLING

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

// ================= Port IO

/// Unsafe due to call to Port.write(...)
/// Unsafe due to writing to port
pub unsafe fn outb(port: u16, data: u16) {
    let mut p = Port::new(port);
    p.write(data);
}

/// Unsafe due to call to Port.read(...)
/// Unsafe due to writing to port
pub unsafe fn inb(port: u16) -> u16 {
    let mut p = Port::new(port);
    p.read()
}

// ================= CUSTOM PANIC IMPLIMENTATION

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // TODO: Timestamps?
    // println!("&4{}", _info);

    hlt_loop(); // halt the os
}

// ================= HLT LOOP

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}