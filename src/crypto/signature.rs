use crate::crypto::hash::Hash;
use crate::crypto::asymmetric::AsymmetricCipher;

pub trait Signature{
    type Hash: Hash;
    type Cipher: AsymmetricCipher;
    const SIGNATURE_SZ: usize;
    fn sign(data: &[u8],sign: &mut [u8;SIGNATURE_SZ],key: &Self::Cipher::PrivateKey);
    fn verify(data: &[u8],sign: &[u8;SIGNATURE_SZ],key: &Self::Cipher::PublicKey) -> bool;
}