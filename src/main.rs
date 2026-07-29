#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::arch::global_asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

global_asm!(
      ".global _start",
      "_start:",
      "la sp, __stack_top",
      "j rust_main"
);

#[no_mangle]
extern "C" fn rust_main() -> ! {
    
    let uart = 0x10000000 as *mut u8;
    let c: u8 = b'A';
    unsafe{
        *uart = c;
    }
    loop{}
}
