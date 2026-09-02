use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

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

pub fn init_heap(heap_start: usize, heap_end: usize){
    let mut guard = ALLOCATOR.lock();
    guard.init(heap_start, heap_end);
}