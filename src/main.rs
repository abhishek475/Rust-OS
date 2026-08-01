#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::arch::global_asm;
use core::fmt::Write;

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
    let _ = write!(Uart,"{}",info);
    loop{}
}

global_asm!(
    "
    .global _start
    _start:
    la sp, __stack_top
    j _zero_bss

    .global _zero_bss
    _zero_bss:
    la t0, __bss_bottom
    la t1, __bss_top
    li t2, 0
    bss_loop:
        beq t0, t1, bss_done
        sb t2, 0(t0)
        addi t0, t0, 1
        j bss_loop
    bss_done:
    j rust_main

    .global trap_entry
    trap_entry:
        j trap_entry
    "
);

#[unsafe(no_mangle)]
extern "C" fn trap_handler(){
    let mut uart = Uart;
    let _ = writeln!(uart,"trap occured");
    loop{}
}
unsafe extern "C" {
    fn trap_entry();
}
#[unsafe(no_mangle)]
extern "C" fn rust_main() -> ! {

    let uart = 0x10000000 as *mut u8;
    let c: u8 = b'A';
    unsafe{
        *uart = c;
    }
    unsafe{
        core::arch::asm!("csrw stvec, {}", in(reg) trap_entry as usize);
    }
    unsafe {
        core::ptr::read_volatile(0xdeadbeef as *const u8);
    }
    loop{}
}

