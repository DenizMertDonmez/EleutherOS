#![no_std]
extern crate alloc;
#![no_main]

core::arch::global_asm!(include_str!("../boot.S"));
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[cfg (feature="qemu")]
const UART_BASE: *mut u32 = 0x09000000_u64 as *mut u32;
#[cfg (not(feature = "qemu"))]
const UART_BASE: *mut u32 = 0x107D001000_usize as *mut u32;

#[cfg (feature="qemu")]
const UART_FR: *mut u32 = (0x09000000_u64+0x18) as *mut u32;
#[cfg (not(feature="qemu"))]
const UART_FR: *mut u32 = (0x107D001000_usize+0x18_usize) as *mut u32;

pub fn yazdir(c: char) {
    unsafe {
	#[cfg(not(feature="qemu"))]
        while (core::ptr::read_volatile(UART_FR) & (1 << 5)) != 0 {}
        core::ptr::write_volatile(UART_BASE, c as u32);
    }
}
pub fn metin_yaz(s: &str){
    for c in s.chars(){
        yazdir(c);
    }
}
unsafe extern "C" {
    static __heap_start: u8;
    static __heap_end: u8;
}

struct BumpAllocator {
    next: usize,
    heap_end: usize,
}

impl BumpAllocator {
    const fn new() -> Self {
        BumpAllocator {
            next: 0,
            heap_end: 0,
        }
    }
    fn init(&mut self, heap_start: usize, heap_end: usize){
        self.next = heap_start;
        self.heap_end = heap_end;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let heap_start = unsafe { &__heap_start as *const u8 as usize };
    let heap_end   = unsafe { &__heap_end   as *const u8 as usize };
    let heap_size  = heap_end - heap_start;
    metin_yaz("Merhaba Dunya!\n");
    loop{}
}
