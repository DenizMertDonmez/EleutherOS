ENTRY(_start)

SECTIONS
{
    /* QEMU virt için yükleme adresi */
    . = 0x40000000;

    /* Çekirdek kodu - .text.boot en başta olmalı */
    .text : {
        KEEP(*(.text.boot))
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    /* BSS bölümü - burada __bss_start ve __bss_end tanımlanır */
    .bss : {
        __bss_start = .;
        *(.bss)
        *(COMMON)
        __bss_end = .;

        __heap_start = .;
        . = . + 0x100000;
        __heap_end = .;
    }

    /* Stack alanı - BSS'in hemen sonrasına yerleştirilir */
    . = ALIGN(16);
    . = . + 0x10000;          /* 64 KB stack alanı */
    _stack_top = .;
}
