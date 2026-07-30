#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::arch::global_asm;

struct Uart;

impl core::fmt::Write for Uart{
    fn write_str(&mut self, s: &str) -> core::fmt::Result{
        let uart = 0x10000000 as *mut u8;
        for b in s.bytes(){
            unsafe{ *uart = b; }
        }
        Ok(())
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    let _ = write!(Uart,"{}",info);
    loop{}
}

global_asm!(
    "
    .global _start
    _start:
    la sp, __stack_top
    j _zero_bss
    "
);
global_asm!(
    "
    .global _zero_bss
    _zero_bss:
    la t0,__bss_bottom
    la t1,__bss_top
    li t2,0
    loop:
        beq t0,t1,done
        sb t2,0(t0)
        addi t0,t0,1
        j loop
    done:
    j rust_main
    "
);

#[no_mangle]
extern "C" fn rust_main() -> ! {

    let uart = 0x10000000 as *mut u8;
    let c: u8 = b'A';
    unsafe{
        *uart = c;
    }
    panic!("test panic!");
   
}

