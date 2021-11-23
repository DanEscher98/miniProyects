#![no_std]  // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(first_ross::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use first_ross::println;

#[no_mangle] // disable the name mangling
pub extern "C" fn _start() -> ! {
    println!("Hello {}! ^_^/\n", "Daniel");
    first_ross::init();
    // x86_64::instructions::interrupts::int3();
    
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };
    
    // panic!("FIX THE SYSTEM");
    
    // PAGE FAULT
    // let ptr_err = 0xdeadbeef as *mut u32;
    // unsafe { *ptr_err = 42; }
    // let ptr_exists = 0x204060 as *mut u32;
    // let x: u32;
    // unsafe { x = *ptr_exists; }
    // println!("read from {:?} works! His value: {}\n", ptr_exists, x);
    // unsafe { *ptr_exists = 42; }
    // println!("write to {:?} works!", ptr_exists);
    
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("It did not crash!\n");
    first_ross::hlt_loop();
}

// this function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    first_ross::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    first_ross::test_panic_handler(info)
}
