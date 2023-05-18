#![no_std]
#![no_main]

mod vga_buffer;

use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    loop {}
}
