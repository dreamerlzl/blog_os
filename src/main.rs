#![no_std]
#![no_main]

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
    panic!("fuck");
    loop {}
}
