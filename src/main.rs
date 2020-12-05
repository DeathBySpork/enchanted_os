// main.rs - Entry point for FrameOS
#![no_std]
#![no_main]

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

use enchanted_os::{task::{executor::Executor, Task}, hlt_loop};
use enchanted_os::allocator;
use enchanted_os::memory::{self, BootInfoFrameAllocator};
use enchanted_os::vga_graphics::{hello_world};

entry_point!(kmain);

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn kmain(boot_info: &'static BootInfo) -> ! {

    enchanted_os::init();

    // the physical memory offset
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // used for translating virtual addresses (mapper.translate_addr(virtual address))
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // create the frame allocator
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    // initialize the heap
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("EnchantedOS Heap initialization failed.");

    let mut executor = Executor::new();

    executor.spawn(hello_world());

    executor.run();

    hlt_loop()
}