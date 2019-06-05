#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(adz_os::test_runner)]

use core::panic::PanicInfo;
use adz_os::test_runner;
use adz_os::{println, serial_print, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  adz_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
  serial_print!("test_println... ");
  println!("test_println output");
  serial_println!("[ok]");
}