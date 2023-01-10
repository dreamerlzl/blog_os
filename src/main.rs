#![no_std]
#![no_main] // disable emitting main for an executable binary
#![feature(custom_test_frameworks)] // allow the use of #[test_case] and #![test_runner]
#![test_runner(crate::test_runner)] // specify which function is called by custom test harness's main
#![reexport_test_harness_main = "test_main"] // rename custom test harness "main"

mod serial;
mod test;
mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {} // still needed cause compiler doesn't know the last statement would exit the program
}

// extern "C" -> telling compiler should use the C calling convention
// the name _start here matters(marks the function as lang_start), so no name mangling
#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn test::Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case] // this attribute is only used for custom test
fn trivial() {
    assert!(true);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
