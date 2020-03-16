use crate::pkmcom::net::Packet;
use crate::io;
use crate::pkmcom::PkmComHash;
use std::num::Wrapping;
use crate::io::{ReadCopy, Writeable, DataOu, DataOutputStream, Endianess};
use std::io::{Read, Error, Write};
use std::pin::Pin;
use crate::helper::Shared;
use std::mem::ManuallyDrop;
use openssl::pkey::{Private, Public};

pub struct HandshakeComplete;

impl PkmComHash for HandshakeComplete{
    fn hashcode(&self) -> Wrapping<u32> {
        255u8.hashcode()*31+0x504B4D00u32.hashcode()
    }

    fn size(&self) -> u32 {
        4
    }
}

impl Packet for HandshakeComplete{
    fn packet_id(&self) -> u8 {
        255
    }
    fn write_packet<S: io::dataio::DataOutput>(&self, out: &mut S) {
        0x504B4D00u32.write(out);
    }
    fn read_packet<S: io::dataio::DataInput>(&mut self, din: &mut S) -> Result<(), String> {
        if u32::read(dim)? != 0x504B4D00u32{
            Err("Mismatched HandshakeComplete".to_string())
        }
        Ok(())
    }
    fn create(id: u8) -> Option<Self> {
        if id==255{
            Some(Self)
        }else{
            None
        }
    }
}

pub enum TcpError{
    PkmComError(std::string::String),
    CipherError(openssl::error::ErrorStack),
    IOError(std::io::Error)
}

pub use TcpError::*;
use openssl::error::ErrorStack;
use crate::io::Endianess::BigEndian;
use cryptostream::read::Decryptor;

impl From<std::io::Error> for TcpError{
    fn from(e: Error) -> Self {
        IOError(e)
    }
}

impl From<openssl::error::ErrorStack> for TcpError{
    fn from(e: ErrorStack) -> Self {
        CipherError(e)
    }
}

impl From<std::string::String> for TcpError{
    fn from(e: String) -> Self {
        PkmComError(e)
    }
}

struct TcpConnection{
    sock: Shared<std::net::TcpStream>,
    key: Pin<Box<openssl::aes::AesKey>>,
    istream: ManuallyDrop<Box<dyn Read>>,
    ostream: ManuallyDrop<Box<dyn Write>>
}

impl Drop for TcpConnection{
    fn drop(&mut self) {
        unsafe{
            ManuallyDrop::drop(istream);
            ManuallyDrop::drop(ostream);
            Box::from_raw(self.sock.inner());
        }
    }
}

impl TcpConnection{
    fn interleve(server: &[u8;128],client: &[u8;128]) -> [u8;256]{
        let ret = unsafe { std::mem::zeroed() };
        for i in 0..128{
            ret[2*i] = server[i];
            ret[2*i+1] = client[i];
        }
        ret
    }
    pub fn server(t: std::net::TcpStream,pair: (openssl::rsa::RsaRef<Private>,openssl::rsa::RsaRef<Public>)) -> Result<Self,TcpError>{

    }
}