use x86_64::structures::idt::InterruptStackFrame;

use super::{interrupt_index::InterruptIndex, pic::PICS};

pub extern "x86-interrupt" fn handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
