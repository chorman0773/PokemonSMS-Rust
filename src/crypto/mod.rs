use std::ops::{Index, IndexMut};
use zeroize::Zeroize;
use std::borrow::{Borrow, BorrowMut};
use std::mem::take;
use core::panicking::panic;

pub mod symmetric;
pub mod asymmetric;
pub mod hash;
pub mod signature;
pub mod cipher;
pub mod sha;



pub trait Destroyable{
    fn destroy(&mut self) -> ();
    fn is_destroyed(&self) -> bool;
}

#[derive(Zeroize,Default,Clone)]
#[zeroize(drop)]
pub struct DestroyableByteArray<Len: usize>{
    bytes: [u8;Len],
    destroyed: bool
}

impl<Len: usize> From<[u8;Len]> for DestroyableByteArray<Len>{
    fn from(arr:[u8;Len]) -> DestroyableByteArray<Len>{
        return Self{bytes: arr,destroyed: false}
    }
}

impl<Len: usize> Destroyable for DestroyableByteArray<Len>{
    fn destroy(&mut self) -> () {
        self.zeroize();
        self.destroyed = true
    }

    fn is_destroyed(&self) -> bool {
        self.destroyed
    }
}

impl<T: Destroyable> Destroyable for Option<T>{
    fn destroy(&mut self) -> () {
        match self{
            Some(d) => {
                d.destroy();
                *self = None;
            },
            None =>()
        }
    }

    fn is_destroyed(&self) -> bool {
        match self{
            Some(d) => d.is_destroyed(),
            None => true
        }
    }
}

impl<Len: usize> Index<usize> for DestroyableByteArray<Len>{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if destroyed{
            panic!("Array is Destroyed, array can't be indexed");
        }else if index >= Len {
            panic!("Out of range");
        }
        return &bytes[index];
    }
}

impl<Len: usize> IndexMut<usize> for DestroyableByteArray<Len>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if destroyed{
            panic!("Array is Destroyed and cannot be indexed");
        }else if index >= Len {
            panic!("Out of range");
        }
        return &mut bytes[index];
    }
}

impl<Len: usize> Borrow<[u8]> for DestroyableByteArray<Len>{
    fn borrow(&self) -> &[u8] {
        if destroyed{
            panic!("Array is Destroyed")
        }
        return bytes
    }
}

impl<Len: usize> BorrowMut<[u8]> for DestroyableByteArray<Len>{
    fn borrow_mut(&mut self) -> &mut [u8] {
        if destroyed{
            panic!("Array is Destroyed")
        }
        return bytes
    }
}

pub trait BlockCipher{
    const BLK_SIZE: usize;
}

