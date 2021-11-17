#![no_std]  // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

// this function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // disable the name mangling
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}
