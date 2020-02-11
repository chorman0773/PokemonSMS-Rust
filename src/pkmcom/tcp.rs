use crate::pkmcom::net::Packet;
use crate::io;
use crate::pkmcom::PkmComHash;
use std::num::Wrapping;
use crate::io::{ReadCopy, Writeable};
use std::io::{Read, Error};

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


struct TcpConnection{
    sock: std::net::TcpStream,
    key: openssl::aes::AesKey,
    istream: cryptostream::read::Decryptor<&'_ std::net::TcpStream>,
    ostream: cryptostream::write::Encryptor<&'_ std::net::TcpStream>
}
