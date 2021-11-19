// All integration tests are their own executables and completely separate from
// our main.rs. This means that each test needs to define its own entry point
// function.

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(first_ross::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use first_ross::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    first_ross::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[test_case]
fn simple_fail() {
    assert_eq!(0, 0);
}
