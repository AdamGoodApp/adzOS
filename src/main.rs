#![no_std] // disable linking to the standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// this function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

// new entry point, linker looks for _start by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  loop {}
}
