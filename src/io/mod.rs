
use std::borrow::BorrowMut;
use std::ops::{DerefMut, Shr};
use std::io::Read;


#[derive(Clone,Eq,PartialEq)]
pub enum Status{
    Ok(usize),
    Error(std::string::String),
    Eof
}

impl Default for Status{
    fn default() -> Self {
        Ok(0)
    }
}

impl Status{
    pub fn is_ok(&self) -> bool{
        match self{
            Status::Ok(_) => true,
            _ => false
        }
    }
    pub fn as_size(&self) -> Option<usize>{
        match self{
            Status::Ok(sz) => Some(*sz),
            _ => None
        }
    }
}

impl From<&Status> for Result<usize,std::string::String>{
    fn from(s: &Status) -> Self {
        match s{
            Status::Ok(sz) => Result::Ok(*sz),
            Status::Error(e) => Result::Err(e.clone()),
            Status::Eof => Result::Err("Eof on stream".to_string())
        }
    }
}

pub trait InputStream{
    fn read(&mut self,out:&mut [u8]) -> Status;
    fn readByte(&mut self) -> Option<u8>;
    fn last_error(&self) -> Status;
    fn clear_error(&mut self) -> ();
}

pub trait OutputStream{
    fn write(&mut self,out:&[u8]) -> Status;
    fn writeByte(&mut self,val:u8) -> Option<()>;
    fn last_error(&self) -> Status;
    fn clear_error(&mut self) -> ();
    fn flush(&mut self) -> ();
}

pub mod dataio;
pub mod dev;
mod cipher;
