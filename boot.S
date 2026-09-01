.section .text.boot
.global _start

_start:
    // Sadece çekirdek 0 devam etsin
    mrs     x0, mpidr_el1
    and     x0, x0, #0xFF
    cbz     x0, master

    // Diğer çekirdekler beklemede kalsın
halt:
    wfe
    b       halt

master:
    // Stack pointer'ı ayarla (linker script'te tanımlı _stack_top)
    ldr     x0, =_stack_top
    mov     sp, x0

    // BSS bölümünü temizle
    ldr     x0, =__bss_start
    ldr     x1, =__bss_end
    sub     x1, x1, x0
    bl      clear_bss

    // Kernel ana fonksiyonuna atla
    bl      kernel_main

    // Kernel dönerse (normalde dönmez) dur
    b       halt

clear_bss:
    cbz     x1, 2f
1:
    str     xzr, [x0], #8
    sub     x1, x1, #8
    cbnz    x1, 1b
2:
    ret

// Not: Burada .bss bölümü YOK!
// Stack ve BSS, linker script'te ayrı ayrı tanımlanacak.
