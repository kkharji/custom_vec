use std::{
    alloc, mem,
    ops::{Deref, DerefMut, Index},
    ptr,
    slice::{self, SliceIndex},
};

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

impl<T> Deref for MVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for MVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for MVec<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T> Drop for MVec<T> {
    fn drop(&mut self) {
        unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(
                mem::size_of::<T>() * self.capacity,
                mem::align_of::<T>(),
            );
            // Drop items
            ptr::drop_in_place(self.deref_mut());
            // Drop pointer
            alloc::dealloc(self.ptr.as_ptr() as _, layout);
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
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len.eq(&0) {
            None
        } else {
            unsafe {
                self.len -= 1;
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!(
                "removal index (is {index}) should be < len (is {})",
                self.len
            )
        } else {
            unsafe {
                let item;
                let ptr = self.as_mut_ptr().add(index);
                // Copy
                item = ptr::read(ptr);
                // Shift
                ptr::copy(ptr.offset(1), ptr, self.len - index - 1);
                self.len -= 1;
                item
            }
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

        assert_eq!(vec.first(), Some(&1));
        assert_eq!(vec.deref(), &[1, 2, 3, 4]);
        assert_eq!(vec[0], 1);

        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.pop(), Some(4));
        let second_item = vec.remove(2);
        assert_eq!(second_item, 3);
        assert_eq!(vec.len(), 2);
    }
}
