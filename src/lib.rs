use std::{alloc, mem, ptr};

pub struct MVec<T> {
    ptr: ptr::NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> Default for MVec<T> {
    fn default() -> Self {
        Self {
            ptr: ptr::NonNull::dangling(),
            len: Default::default(),
            capacity: Default::default(),
        }
    }
}

impl<T> MVec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, item: T) {
        let t_size = mem::size_of::<T>();

        assert!(t_size != 0);

        if self.capacity == 0 {
            self.capacity = 4;
            let layout =
                alloc::Layout::array::<T>(self.capacity).expect("Failed to create array layout");
            unsafe {
                let ptr = alloc::alloc(layout) as *mut T;
                self.ptr = ptr::NonNull::new_unchecked(ptr);
                self.ptr.as_ptr().write(item);
                self.len += 1;
            }
        } else if self.len.lt(&self.capacity) {
            let offset = self
                .len
                .checked_mul(t_size)
                .expect("Can not reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");

            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
            }

            self.len += 1;
        } else {
            let size = t_size * self.capacity;
            let align = mem::align_of::<T>();

            size.checked_add(size % align)
                .expect("Overflow size % align");

            let layout = alloc::Layout::from_size_align(size, align)
                .expect("Fail to create layout from size and align");

            let new_capacity = self
                .capacity
                .checked_mul(2)
                .expect("Multiple capacity by 2 overflow");

            let new_size = size * (new_capacity);

            unsafe {
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                self.ptr = ptr::NonNull::new_unchecked(ptr as *mut T);
                self.ptr.as_ptr().write(item);
            }

            self.capacity += 1;
            self.len += 1;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { &*self.ptr.as_ptr().add(index) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = MVec::<usize>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        assert_eq!(vec.len(), 4);
        assert_eq!(vec.capacity(), 4);

        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
    }
}
