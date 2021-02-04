//! Module for the Named Binary Tag, and ShadeNBT format
//! This module deals with representing the NBT format as a structure,
//!  and binary serialization. It does not perform Shade version checking, nor does it parse the Shade or CryptoShade header
//!

pub mod array {
    //!
    //! Types for NBT_Tag*Array
    use std::{
        mem::ManuallyDrop,
        ops::{Deref, DerefMut, Index, IndexMut},
        ptr,
        slice::{self, SliceIndex},
    };
    ///
    /// A type which can store a dynamic, fixed-size array of T
    #[derive(Clone, Debug)]
    pub struct NbtArray<T> {
        inner: Box<[T]>,
    }

    impl<T> Deref for NbtArray<T> {
        type Target = [T];

        #[inline]
        fn deref(&self) -> &Self::Target {
            self.inner.deref()
        }
    }

    impl<T> DerefMut for NbtArray<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.inner.deref_mut()
        }
    }

    impl<T> From<Vec<T>> for NbtArray<T> {
        #[inline]
        fn from(v: Vec<T>) -> Self {
            Self {
                inner: v.into_boxed_slice(),
            }
        }
    }

    impl<T> From<Box<[T]>> for NbtArray<T> {
        #[inline]
        fn from(v: Box<[T]>) -> Self {
            Self { inner: v }
        }
    }

    impl<T, const N: usize> From<[T; N]> for NbtArray<T> {
        #[inline]
        fn from(v: [T; N]) -> Self {
            Self { inner: Box::new(v) }
        }
    }

    impl<T, I: SliceIndex<[T]>> Index<I> for NbtArray<T> {
        type Output = I::Output;

        #[inline]
        fn index(&self, index: I) -> &Self::Output {
            self.inner.index(index)
        }
    }

    impl<T, I: SliceIndex<[T]>> IndexMut<I> for NbtArray<T> {
        #[inline]
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            self.inner.index_mut(index)
        }
    }

    ///
    /// Iterator for NbtArray<T>
    pub struct IntoIter<T> {
        inner: Box<[ManuallyDrop<T>]>,
        position: usize,
    }

    impl<T> Drop for IntoIter<T> {
        fn drop(&mut self) {
            for i in self.position..self.inner.len() {
                // SAFETY:
                // from position..len has not been taken. position is incremented first
                unsafe { ManuallyDrop::drop(&mut self.inner[i]) }
            }
        }
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.position = self.position.checked_add(1).unwrap();
            // SAFETY:
            // position is incremented first, so any item taken here will never be visited again
            self.inner
                .get_mut(self.position - 1)
                .map(|m| unsafe { ManuallyDrop::take(m) })
        }
    }

    impl<T> IntoIterator for NbtArray<T> {
        type Item = T;

        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            let len = self.inner.len();
            let ptr = Box::into_raw(self.inner).cast::<T>();
            // SAFETY: the slice is from self.inner: Box<[T]>. ManuallyDrop<T> is transparent over T
            let inner = unsafe {
                Box::from_raw(ptr::slice_from_raw_parts_mut(
                    ptr.cast::<ManuallyDrop<T>>(),
                    len,
                ))
            };
            IntoIter { inner, position: 0 }
        }
    }

    ///
    /// Iterator over references to elements of an NbtArray
    pub struct Iter<'a, T: 'a>(slice::Iter<'a, T>);

    impl<'a, T: 'a> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<&'a T> {
            self.0.next()
        }
    }

    ///
    /// Iterator over mutable references to elements of an NbtArray
    pub struct IterMut<'a, T: 'a>(slice::IterMut<'a, T>);

    impl<'a, T: 'a> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<&'a mut T> {
            self.0.next()
        }
    }

    impl<T> NbtArray<T> {
        ///
        /// Returns an iterator of references to the array elements
        pub fn iter(&self) -> Iter<T> {
            Iter(self.inner.iter())
        }
        ///
        /// Returns an iterator of mut references to the array elements
        pub fn iter_mut(&mut self) -> IterMut<T> {
            IterMut(self.inner.iter_mut())
        }
    }

    impl<'a, T> IntoIterator for &'a NbtArray<T> {
        type Item = &'a T;

        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut NbtArray<T> {
        type Item = &'a mut T;

        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
}
