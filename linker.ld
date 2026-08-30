ENTRY(start)

SECTIONS {
    . = 1M;

    .boot : {
        KEEP(*(.multiboot2_header))
    }

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }
}
