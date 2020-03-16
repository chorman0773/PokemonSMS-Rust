use crate::version::Version;
use crate::io::{ReadCopy, DataInput, DataInputStream};

use super::constants;
use std::io::Read;
use crate::shade::nbt::NBTTag;
use crate::shade::shade::CryptoCompound::Encrypted;
use core::ptr;
use std::mem::ManuallyDrop;
use openssl::sha::Sha256;
use zeroize::Zeroizing;

pub struct ShadeHeader{
    version: Version,
    flags: Option<u8>
}

pub struct CryptoHead{
    block_sz: u16,
    salt: [u8;32],
    iv: [u8;16],
    check_hash: [u8;32]
}


pub enum CryptoCompound{
    Encrypted(Box<[u8]>),
    Decrypted(NBTTag)
}

impl CryptoCompound{
    pub fn read<I: Read +?Sized>(block_sz: u16,read: &mut I) -> std::io::Result<EncryptedCompound>{
        let mut blocks = Vec::with_capacity(block_size*16);
        sz.resize(block_size*16,0u8);
        read.read(&mut blocks);
        Ok(Self{blocks:blocks.into_boxed_slice()})
    }
    pub fn decrypt(&mut self,head: &CryptoHead,order: Endianness,passwd: &str) -> Result<(),std::string::String>{
        Ok(if let Encrypted(b_blocks) = self{
            let mut pvec = Vec::from(passwd);
            pvec.extend_from_slice(&head.iv[..8]);
            let mut digest = Sha256::new();
            digest.update(&pvec);
            if head.check_hash!=digest.finish(){
                return Err("Decryption Failed, bad password".to_string())
            }
            unsafe{
                pvec.set_len(passwd.len());
                // Sound because u8 is Copy
                pvec.extend_from_slice(&head.salt);
                digest = Sha256::new();
                digest.update(&pvec);
                let key = Zeroizing::new(digest.finish());
                let blocks = ManuallyDrop::new(ptr::read(b_blocks));
                let mut input = cryptostream::bufread::Decryptor::new(&**blocks,openssl::symm::Cipher::aes_256_cbc(),&*key,&head.iv)?;
                let mut din = DataInputStream::new(&mut input,order);
                core::mem::forget(b_blocks);
                ptr::write(self,CryptoCompound::Decrypted(din.read_value()?));
                core::mem::drop(ManuallyDrop::into_inner(blocks))
            }
        }else{

        })

    }
}

pub enum ShadeFile{

}


