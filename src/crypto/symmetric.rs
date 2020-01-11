
use crate::crypto::{Destroyable, DestroyableByteArray};
use crate::crypto::cipher::{Mode,Cipher};

trait SecretKey : Destroyable{
    fn key_size() -> usize;
}

pub trait SymmetricCipherParams : Destroyable + Default{}

#[derive(Default,Copy)]
pub struct EmptyParams{}

impl Destroyable for EmptyParams{
    fn destroy(&mut self) -> () {}
    fn is_destroyed(&self) -> bool {
        false
    }
}

impl SymmetricCipherParams for EmptyParams{}

#[derive(Default)]
pub struct IvParam<Size: usize>{
    arr : DestroyableByteArray<Size>
}

impl<Size: usize> IvParam<Size>{
    pub fn new(arr:[u8;Size]) -> Self{
        Self{arr: DestroyableByteArray::from(arr)}
    }
    pub fn copy(arr: &DestroyableByteArray<Size>) -> Self{
        Self{arr: Clone::clone(arr) }
    }
}

impl<Size: usize> Destroyable for IvParam<Size>{
    fn destroy(&mut self) -> () {
        arr.destroy()
    }

    fn is_destroyed(&self) -> bool {
        arr.is_destroyed()
    }
}

impl<Size: usize> SymmetricCipherParams for IvParam<Size>{}

pub trait SymmetricCipher : Cipher{
    type Key : SecretKey;
    type Params : SymmetricCipherParams;
    fn init(mode: Mode,key: &Self::Key) -> Self;
    fn init_params(&mut self,params: Self::Params);
}
