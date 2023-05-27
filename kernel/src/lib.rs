#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

extern crate alloc;

pub mod gdt;
pub mod interrupts;
pub mod logger;
pub mod memory;

use bootloader_api::info::FrameBufferInfo;
use bootloader_api::BootInfo;
use log::LevelFilter;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init_logger(
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    frame_buffer_logger_status: bool,
    serial_logger_status: bool,
) {
    let logger = logger::LOGGER.get_or_init(move || {
        logger::LockedLogger::new(
            framebuffer,
            info,
            frame_buffer_logger_status,
            serial_logger_status,
        )
    });
    log::set_logger(logger).expect("logger already set");
    log::set_max_level(LevelFilter::Trace);
    log::info!("Framebuffer info: {info:?}");
}

pub fn init(boot_info: &'static mut BootInfo) {
    logger::init(&mut boot_info.framebuffer);
    gdt::init();
    interrupts::init();
    memory::allocator::init_heap(boot_info.physical_memory_offset, &boot_info.memory_regions)
        .expect("heap initialization failed");
}
