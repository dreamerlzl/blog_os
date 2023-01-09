#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// extern "C" -> telling compiler should use the C calling convention
// the name _start here matters(marks the function as lang_start), so no name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case] // this attribute is only used for custom test
fn trivial() {
    println!("ok");
}
