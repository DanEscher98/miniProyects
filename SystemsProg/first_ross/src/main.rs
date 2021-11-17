#![no_std]  // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// this function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // disable the name mangling
pub extern "C" fn _start() -> ! {
    /* The VGA text buffer is a special memory area mapped to the VGA
     * hardware that contains the contents displayed on screen. It
     * normally consists of 25 lines that each contain 80 character 
     * cells. The buffer is located at address 0xb8000. */
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // light cyan
        }
    }
    loop {}
}
