#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use first_ross::vga_buffer::{Green, Red};
use first_ross::{
    QemuExitCode, exit_qemu,
    serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{}", Green("[ok]"));
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test_main();
    should_fail();
    serial_println!("{}", Red("[test did not panic]"));
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("\nRunning {} tests", tests.len());
//     for test in tests {
//         test();
//         serial_println!("[test did not panic]");
//         exit_qemu(QemuExitCode::Failed);
//     }
//     exit_qemu(QemuExitCode::Success);
// }
