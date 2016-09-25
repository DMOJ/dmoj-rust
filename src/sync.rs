use std::cell::UnsafeCell;

pub struct NotThreadSafe<T> {
    value: UnsafeCell<T>
}

unsafe impl<T> Sync for NotThreadSafe<T> {}

impl<T> NotThreadSafe<T> {
    pub fn new(value: T) -> NotThreadSafe<T> {
        NotThreadSafe {
            value: UnsafeCell::new(value)
        }
    }

    pub unsafe fn get(&self) -> *mut T {
        self.value.get()
    }
}
