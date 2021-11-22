#![no_std]  // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(first_ross::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use first_ross::println;

#[no_mangle] // disable the name mangling
pub extern "C" fn _start() -> ! {
    println!("Hello {}! ^_^/", "Daniel");
    first_ross::init();
    x86_64::instructions::interrupts::int3();
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    // panic!("FIX THE SYSTEM");
    #[cfg(test)]
    test_main();
    println!("It did not crash!\n");
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
