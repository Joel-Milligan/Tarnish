use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::interrupts::interrupt_index::InterruptIndex;
use crate::interrupts::{
    breakpoint_interrupt, double_fault, keyboard_interrupt, page_fault, timer_interrupt,
};

pub fn init_idt() {
    IDT.load();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_interrupt::handler);
        idt.page_fault.set_handler_fn(page_fault::handler);
        idt.double_fault.set_handler_fn(double_fault::handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt::handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt::handler);
        idt
    };
}
