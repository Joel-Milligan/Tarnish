# Tarnish
A toy operating system built in Rust, following this [website](https://os.phil-opp.com/).

## Modules
The following section briefly describes the various components of the kernel.

### GDT
Initialising the [Global Descriptor Table](https://wiki.osdev.org/Global_Descriptor_Table).

### Interrupts
Initialises the [IDT](https://wiki.osdev.org/Interrupt_Descriptor_Table) and interrupt handlers.

Only supports [8259 PIC](https://wiki.osdev.org/PIC), and thus interrupts only work when booting with BIOS.
Will need to upgrade to [APIC](https://wiki.osdev.org/APIC) to support UEFI.

### Logger
A logger for use with the `log` crate. Supports logging out to both the serial port and framebuffer.

#### Framebuffer
Used for drawing to the screen. Only supports logging text.

#### Serial
Used for writing out to a serial port.

### Memory
Builds a frame allocator from the passed in boot info.

#### Allocator
Three different implementations of allocators to use. Fixed size block is the primary one to use, with linked list used as a fall-back.

1. Fixed Size Block
1. Linked List 
1. Bump
