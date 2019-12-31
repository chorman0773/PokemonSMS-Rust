
use std::borrow::BorrowMut;
use std::ops::{DerefMut, Shr};


pub enum Status{
    Ok,
    Error(std::string::String),
    Eof
}
pub trait InputStream{
    fn read(&mut self,out:&mut [u8]) -> usize;
    fn readByte(&mut self) -> Option<u8>;
    fn check_status(&self) -> Status;
    fn clear_error(&mut self) -> ();
}

pub trait OutputStream{
    fn write(&mut self,out:&[u8]) -> usize;
    fn writeByte(&mut self,val:u8) -> ();
    fn check_status(&self) -> Status;
    fn clear_error(&mut self) -> ();
    fn flush(&mut self) -> ();
}



pub mod dataio;
pub mod dev;
pub mod cipher;