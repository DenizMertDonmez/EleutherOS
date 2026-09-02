#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

core::arch::global_asm!(include_str!("../boot.S"));
use core::panic::PanicInfo;
use crate::drivers::uart::metin_yaz;

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

    
    metin_yaz("Merhaba Dunya!\n");

    let mut v = alloc::vec::Vec::new();
    v.push(42);
    v.push(7);
    metin_yaz("Vec olusturuldu!\n");

    loop{}
}
