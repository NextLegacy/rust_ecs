use std::{alloc::GlobalAlloc, ops::Index, ptr::NonNull};

pub struct TypeErasedVec {
    data: NonNull<u8>,
    layout: std::alloc::Layout,
    len: usize,
    capacity: usize, // in items -> total bytes = layout.size() * capacity
}

impl TypeErasedVec {
    pub fn new<T>() -> Self {
        let layout = std::alloc::Layout::new::<T>();
        let data = unsafe { std::alloc::System.alloc(layout) };
        let data = NonNull::new(data).expect("Allocation failed");

        Self {
            data,
            layout,
            len: 0,
            capacity: 0,
        }
    }

    pub fn reserve_typed<T>(&mut self, additional: usize) {
        let layout = std::alloc::Layout::new::<T>();
        self.reserve(additional * layout.size() / self.layout.size());
    }

    pub fn reserve(&mut self, additional: usize) {
        let new_len = self.len + additional;
        if new_len > self.capacity {
            let new_capacity = new_len.next_power_of_two();
            let new_layout = std::alloc::Layout::from_size_align(self.layout.size() * new_capacity, self.layout.align()).unwrap();
            let new_data = unsafe { std::alloc::System.alloc(new_layout) };
            let new_data = NonNull::new(new_data).expect("Allocation failed");

            unsafe {
                std::ptr::copy_nonoverlapping(self.data.as_ptr(), new_data.as_ptr(), self.len * self.layout.size());
                std::alloc::System.dealloc(self.data.as_ptr(), self.layout);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }

    pub fn emplace(&mut self) {
        if self.len == self.capacity {
            self.reserve(self.capacity + 1);
        }
        unsafe { self.data.as_ptr().add(self.len * self.layout.size()) };
        self.len += 1;
    }

    pub fn emplace_typed<T>(&mut self) {
        self.reserve_typed::<T>(1);
        self.emplace();
    }

    pub fn push<T>(&mut self, value: T) {
        self.emplace_typed::<T>();
        *self.get_typed_mut::<T>(self.len - 1) = value;
    }

    pub fn get_typed<T>(&self, index: usize) -> &T {
        unsafe { &*(self.data.as_ptr().add(index * self.layout.size()) as *const T) }
    }

    pub fn get_typed_mut<T>(&mut self, index: usize) -> &mut T {
        assert!(index < self.len);
        let layout = std::alloc::Layout::new::<T>();
        unsafe {
            &mut *(self.data.as_ptr().add(index * layout.size()) as *mut T)
        }
    }

    pub fn remove_swap_with_last(&mut self, index: usize) {
        assert!(index < self.len);
        self.len -= 1;
        if index < self.len {
            unsafe {
                let last_ptr = self.as_ptr().add(self.len * self.layout.size());
                let ptr = self.as_mut_ptr().add(index * self.layout.size());
                std::ptr::copy_nonoverlapping(last_ptr, ptr, self.layout.size());
            }
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len * self.layout.size()) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr(), self.len * self.layout.size()) }
    }

    pub fn as_typed_slice<T>(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    pub fn as_typed_slice_mut<T>(&mut self) -> &mut [T] {
        let layout = std::alloc::Layout::new::<T>();
        let number_of_elements = (self.capacity * self.layout.size()) / layout.size();
        unsafe { std::slice::from_raw_parts_mut(self.data.as_ptr() as *mut T, number_of_elements) }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_ptr()
    }

    pub fn as_typed_ptr<T>(&self) -> *const T {
        self.data.as_ptr() as *const T
    }

    pub fn as_typed_mut_ptr<T>(&mut self) -> *mut T {
        self.data.as_ptr() as *mut T
    }

    pub fn len     (&self) -> usize              { self.len      }
    pub fn is_empty(&self) -> bool               { self.len == 0 }
    pub fn capacity(&self) -> usize              { self.capacity }
    pub fn layout  (&self) -> std::alloc::Layout { self.layout   }

    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, u8> {
        self.as_slice_mut().iter_mut()
    }

    pub fn iter_typed<T>(&self) -> std::slice::Iter<'_, T> {
        self.as_typed_slice::<T>().iter()
    }

    pub fn iter_typed_mut<T>(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_typed_slice_mut::<T>().iter_mut()
    }
}

impl Drop for TypeErasedVec {
    fn drop(&mut self) {
        unsafe {
            std::alloc::System.dealloc(self.data.as_ptr(), self.layout);
        }
    }
}