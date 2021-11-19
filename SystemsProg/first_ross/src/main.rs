#![no_std]  // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(first_ross::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use first_ross::println;

#[no_mangle] // disable the name mangling
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "Daniel");
    // panic!("FIX THE SYSTEM");
    #[cfg(test)]
    test_main();
    println!("Goodbye.");
    loop {}
}

// this function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    first_ross::test_panic_handler(info)
}
