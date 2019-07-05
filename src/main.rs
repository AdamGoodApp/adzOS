#![no_std] // disable linking to the standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(adz_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use adz_os::println;

static WELCOME_TEXT: &str = "Welcome to ADZ OS!";

// new entry point, linker looks for _start by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("{}", WELCOME_TEXT);
  println!("version: 2.3");

  adz_os::init();

  #[cfg(test)]
  test_main();

  loop{};
}

/// this function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  adz_os::test_panic_handler(info)
}