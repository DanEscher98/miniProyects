#![no_std]  // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

// this function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QuemuExitCode::Failed);
    loop {}
    // The loop is needed because the compiler doesn't know htat the
    // `isa-debug-exit` device causes a program exit.
}

#[no_mangle] // disable the name mangling
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", "Daniel");
    // panic!("FIX THE SYSTEM");
    #[cfg(test)]
    test_main();
    println!("Goodbye.");
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QuemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QuemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        // 0xf4 is the `iobase` of the `isa-debug-exit` device
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("\nRunning {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QuemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion...");
    assert_eq!(2, 1);
    serial_println!("[ok]");
}
