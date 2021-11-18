#![no_std]  // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

// this function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // disable the name mangling
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "Daniel");
    // panic!("FIX THE SYSTEM");
    loop {}
}
