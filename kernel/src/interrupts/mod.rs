mod breakpoint_interrupt;
mod double_fault;
pub mod idt;
mod interrupt_index;
mod keyboard_interrupt;
mod page_fault;
mod pic;
mod timer_interrupt;

use idt::init_idt;
use pic::PICS;

pub fn init() {
    init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
