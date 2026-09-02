use core::ptr::read_volatile;

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
        while (core::ptr::read_volatile(UART_FR) & (1 << 5)) != 0 {}
        core::ptr::write_volatile(UART_BASE, c as u32);
    }
}
pub fn metin_yaz(s: &str){
    for c in s.chars(){
        yazdir(c);
    }
}
pub fn oku() -> char{
    unsafe {
        while (read_volatile(UART_FR) & (1<<4))!=0 {}
        let veri = read_volatile(UART_BASE) as u8;
        veri as char
    }
}