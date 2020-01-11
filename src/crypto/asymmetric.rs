use crate::crypto::{Destroyable, cipher::Mode};


pub trait PublicKey<Private: PrivateKey>{

}

pub trait PrivateKey : Destroyable{
    type PublicKey : PublicKey<Self>;
    fn derive_public(&self) -> Option<Self::PublicKey>;
}



enum AsymmetricCipherKey<Pr: PrivateKey,Pu: PublicKey<Pr>>{
    Public(Pu),
    Private(Pr)
}

pub use AsymmetricCipherKey::{Public,Private};
use crate::crypto::cipher::Cipher;


pub trait AsymmetricCipher : Cipher{
    type PrivateKey : PrivateKey;
    type PublicKey : PublicKey<Self::PrivateKey>;
    fn init(mode: Mode,key: &AsymmetricCipherKey<Self::PrivateKey,Self::PublicKey>) -> Self;
}

pub trait KeyPair: Destroyable{
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey<Self::PrivateKey>;
    fn get_public_key(&self) -> Option<&Self::PublicKey>;
    fn get_private_key(&self) -> Option<&Self::PrivateKey>;
    fn generate() -> Self;
}