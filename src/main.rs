#![no_std] // disable linking to the standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod power_manager;
mod serial;

use core::panic::PanicInfo;
use power_manager::{QemuExitCode, exit_qemu};

static WELCOME_TEXT: &str = "Welcome to ADZ OS!";

// new entry point, linker looks for _start by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("{}", WELCOME_TEXT);
  println!("version: 2.3");

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

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    
    for test in tests {
      test();
    }

    exit_qemu(QemuExitCode::Success);
}