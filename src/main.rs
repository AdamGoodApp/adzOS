#![no_std] // disable linking to the standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

static WELCOME_TEXT: &str = "Welcome to ADZ OS!";

// new entry point, linker looks for _start by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("{}", WELCOME_TEXT);
  println!("version: 2.3");

  loop{};
}

/// this function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}
