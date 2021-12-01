#![no_std] // dont' link the Rust stdlib
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(first_ross::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use first_ross::println;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::Translate, VirtAddr};
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
    // use first_ross::memory::active_level_4_table;
    // use x86_64::structures::paging::PageTable;

    // let ptr_err = 0xdeadbeef as *mut u32;
    // unsafe { *ptr_err = 42; }
    // let ptr_exists = 0x204060 as *mut u32;
    // let x: u32;
    // unsafe { x = *ptr_exists; }
    // println!("read from {:?} works! His value: {}\n", ptr_exists, x);
    // unsafe { *ptr_exists = 42; }
    // println!("write to {:?} works!", ptr_exists);

    // Check Level Page
    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}",
    //     level_4_page_table.start_address());

    // ITER THROUGH PAGE LEVELS
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
    //         // get the physical address from the entry and convert it
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         // print non-empty entries of the level 3 table
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("    L3 Entry {}: {:?}", i, entry);
    //                 // let phys = entry.frame().unwrap().start_address();
    //                 // let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //                 // let ptr = VirtAddr::new(virt).as_mut_ptr();
    //                 // let l2_table: &PageTable = unsafe { &*ptr };
    //                 //
    //                 // for (i, entry) in l2_table.iter().enumerate() {
    //                 //     if !entry.is_unused() {
    //                 //         println!("        L2 Entry {}: {:?}", i, entry);
    //                 //     }
    //                 // }
    //             }
    //         }
    //     }
    // }

    // use first_ross::memory;
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mapper = unsafe { memory::init(phys_mem_offset) };
    // let addressses = [
    //     // tyhe identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset
    // ];

    // for &address in &addressses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt); // in the Traslate trait
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // CHOOSING A VIRTUAL PAGE
    // use first_ross::memory;
    // use first_ross::memory::BootInfoFrameAllocator;
    // use x86_64::structures::paging::Page;
    //
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe {
    //     BootInfoFrameAllocator::init(&boot_info.memory_map)
    // };

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
