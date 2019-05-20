#![no_std] // disable linking to the standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

static WELCOME_TEXT: &[u8] = b"Welcome to ADZ OS!";

// new entry point, linker looks for _start by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in WELCOME_TEXT.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }

  loop {}
}

/// this function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
