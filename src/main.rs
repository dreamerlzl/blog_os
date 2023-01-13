#![no_std]
#![no_main] // disable emitting main for an executable binary
#![feature(custom_test_frameworks)] // allow the use of #[test_case] and #![test_runner]
#![test_runner(blog_os::test_runner)] //
#![reexport_test_harness_main = "test_main"] // rename custom test harness "main"

use core::panic::PanicInfo;

use blog_os::println;

// extern "C" -> telling compiler should use the C calling convention
// the name _start here matters(marks the function as lang_start), so no name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
    loop {} // still needed cause compiler doesn't know the last statement would exit the program
}
