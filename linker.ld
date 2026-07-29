ENTRY(_start)

SECTIONS {
    . = 0x80200000;
    .text : {*(.text*)}
    
    bss : {
    __bss_bottom = .
    *(.bss .bss.*)
    *(.sbss .sbss.*)
    __bss_top = .
    }

    .rodata : {*(.rodata .rodata.*)}
    /DISCARD/ : {*(.eh_frame)}

    . = ALIGN(16);
    __stack_bottom = .;
    . += 0x00200000;
    __stack_top = .;
}
