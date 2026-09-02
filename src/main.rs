#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

core::arch::global_asm!(include_str!("../boot.S"));
use core::panic::PanicInfo;
use crate::drivers::uart::{self, metin_yaz};

mod drivers {
    pub mod uart;
}

mod memory {
    pub mod allocator;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> !{
    panic!("Bellek ayirma hatasi: {:?}", layout)
}

unsafe extern "C" {
    static __heap_start: u8;
    static __heap_end: u8;
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let heap_start = unsafe { &__heap_start as *const u8 as usize };
    let heap_end   = unsafe { &__heap_end   as *const u8 as usize };

    
    metin_yaz("> ");
    // Döngü dışında bir değişken: son karakter '\r' miydi?
    let mut last_was_cr = false;

    loop {
        let c = uart::oku();

        if c == '\r' {
            // Enter tuşu: yeni satıra geç, prompt yaz
            uart::yazdir('\n');
            uart::metin_yaz("> ");
        } else if c == '\n' {
            // \n gelirse yok say (çünkü Enter genellikle \r gönderir)
            // burada hiçbir şey yapma
        } else {
            // normal karakteri geri yazdır
            uart::yazdir(c);
        }
    }
}

