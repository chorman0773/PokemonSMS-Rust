use std::ops::{Index, IndexMut};
use zeroize::Zeroize;
use std::borrow::{Borrow, BorrowMut};
use std::mem::take;

trait Destroyable{
    fn destroy(&mut self) -> ();
    fn is_destroyed(&self) -> bool;
}

#[derive(Zeroize,Default)]
#[zeroize(drop)]
struct DestroyableByteArray<Len: usize>{
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
                self = None;
            },
            None =>()
        }
    }

    fn is_destroyed(&self) -> bool {
        self.is_none()
    }
}

impl<Len: usize> Index<usize> for DestroyableByteArray<Len>{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        if destroyed{
            panic!("Array is Destroyed, array can't be destroyed");
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

trait SecretKey : Destroyable{
    fn key_size() -> usize;
}

trait PublicKey<Private: PrivateKey>{

}

trait PrivateKey : Destroyable{
    type PublicKey : PublicKey<Self>;
    fn derive_public(&self) -> Option<Self::PublicKey>;
}

trait SymmetricCipherParams : Destroyable + Default{}

#[derive(Default,Copy)]
struct EmptyParams{}

impl Destroyable for EmptyParams{
    fn destroy(&mut self) -> () {}

    fn is_destroyed(&self) -> bool {
        false
    }
}

impl SymmetricCipherParams for EmptyParams{

}

#[derive(Default)]
struct IvParam<Size: usize>{
    arr : DestroyableByteArray<Size>
}

impl<Size: usize> IvParam<Size>{
    fn new(arr:[u8;size]){
        Self{arr: DestroyableByteArray::from(arr)}
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

enum SymmetricCipherMode{
    Encrypt,
    Decrypt
}

pub trait SymmetricCipher{
    type Key : SecretKey;
    type Params : SymmetricCipherParams;
    fn init_encrypt(&mut self,key: Self::Key);
    fn init_decrypt(&mut self,key: Self::Key);
    fn init_params(&mut self,params: Self::Params);
    fn update(&mut self,in_buf: &[u8],out_buf: &mut [u8]);
    fn do_final(&mut self,in_buf: &[u8],out_buf: &mut [u8]);
}

pub trait AsymmetricCipher{
    type PrivateKey : PrivateKey;
    type PublicKey : PublicKey<Self::PrivateKey>;
    fn init_enc_pub(&mut self,key: Self::PublicKey);
    fn init_dec_priv(&mut self,key: Self::PrivateKey);
    fn init_enc_priv(&mut self,key: Self::PrivateKey);
    fn init_dec_pub(&mut self,key: Self::PublicKey);
    fn update(&mut self,in_buf: &[u8],out_buf: &mut [u8]);
    fn do_final(&mut self,in_buf: &[u8],out_buf: &mut [u8]);
}

