#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

core::arch::global_asm!(include_str!("../boot.S"));
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

fn alloc_error(layout: core::alloc::Layout) -> !{
    panic!("Bellek ayirma hatasi: {:?}", layout)
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

struct SpinLock<T>{
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

struct SpinLockGuard<'a, T>{
    lock: &'a SpinLock<T>,
}

impl<T> core::ops::Deref for SpinLockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {&*self.lock.data.get()}
    }
}

impl<T> core::ops::DerefMut for SpinLockGuard<'_, T>{
    fn deref_mut(&mut self) -> &mut T {
        unsafe {&mut *self.lock.data.get()}
    }
}

impl<T> Drop for SpinLockGuard<'_, T>{
    fn drop(&mut self){
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> SpinLock<T> {
    const fn new(data: T) -> Self {
        SpinLock{
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
    fn lock(&self) -> SpinLockGuard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        SpinLockGuard{lock: self}
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

unsafe impl core::alloc::GlobalAlloc for SpinLock<BumpAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut guard= self.lock();

        let align = layout.align();
        let start = (guard.next + align-1) & !(align-1);
        let size= layout.size();

        if start + size <= guard.heap_end {
            guard.next = start+size;
            start as *mut u8
        } else {
            core::ptr::null_mut()
        }

    }
    unsafe fn dealloc(&self, _ptr:*mut u8, _layout: core::alloc::Layout){}
}

#[global_allocator]
static ALLOCATOR: SpinLock<BumpAllocator> = SpinLock::new(BumpAllocator::new());

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let heap_start = unsafe { &__heap_start as *const u8 as usize };
    let heap_end   = unsafe { &__heap_end   as *const u8 as usize };

    {
        let mut guard = ALLOCATOR.lock();
        guard.init(heap_start, heap_end);
    }
    metin_yaz("Merhaba Dunya!\n");
    let mut v = alloc::vec::Vec::new();
    v.push(42);
    v.push(7);
    metin_yaz("Vec olusturuldu!\n");
    loop{}
}
