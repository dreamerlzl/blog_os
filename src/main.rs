#![no_std]
#![no_main]

fn main() {}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// extern "C" -> telling compilter should use the C calling convention
// the name _start here matters, so no name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
