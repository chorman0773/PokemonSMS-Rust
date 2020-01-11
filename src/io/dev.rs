
use crate::io::{InputStream, Status, OutputStream};
use Status::{Error, Eof};
extern crate getrandom;

#[derive(Default)]
pub struct NullDevice{
    wsz: usize
}

impl InputStream for NullDevice{
    fn read(&mut self, out: &mut [u8]) -> Status {
        Eof
    }

    fn readByte(&mut self) -> Option<u8> {
        Option::None
    }

    fn last_error(&self) -> Status {
        Status::Eof
    }

    fn clear_error(&mut self) -> () {}
}

impl OutputStream for NullDevice{
    fn write(&mut self, out: &[u8]) -> usize {
        wsz = out.len();
        out.len()
    }

    fn writeByte(&mut self, val: u8) -> () {
        wsz = 1;
    }

    fn last_error(&self) -> Status {
        Status::Ok(wsz)
    }

    fn clear_error(&mut self) -> () {

    }

    fn flush(&mut self) -> () {

    }
}

unsafe impl Send for NullDevice{}
unsafe impl Sync for NullDevice{}

#[derive(Default)]
pub struct ZeroDevice{
    sz: usize
}

impl InputStream for ZeroDevice{
    fn read(&self, out: &mut [u8]) -> Status {
        for mut b in out {
            *b = 0;
        }
        sz = out.len();
        return Status::Ok(sz);
    }

    fn readByte(&self) -> Option<u8> {
        sz = 1;
        return Some(0)
    }

    fn last_error(&self) -> Status {
        Status::Ok(sz)
    }

    fn clear_error(&self) -> () {
        ()
    }
}

impl OutputStream for ZeroDevice{
    fn write(&self, out: &[u8]) -> Status {
        sz = out.len();
        return Status::Ok(sz)
    }

    fn writeByte(&self, val: u8) -> () {
        sz = 1;
    }

    fn last_error(&self) -> Status {
        Status::Ok(sz)
    }

    fn clear_error(&self) -> () {
        ()
    }

    fn flush(&self) -> () {
        ()
    }
}

unsafe impl Send for ZeroDevice{}
unsafe impl Sync for ZeroDevice{}


pub struct RandomDevice{
    err: Status
}

impl Default for RandomDevice{
    fn default() -> Self {
        RandomDevice{err: Status::Ok(0)}
    }
}

impl InputStream for RandomDevice{
    fn read(&mut self, out: &mut [u8]) -> Status {
        match getrandom::getrandom(out){
            Ok(()) =>{
                err = Status::Ok(out.len());
                err
            },
            Err(e) => {
                self.err = Error(e.to_string());
                err
            }
        }

    }

    fn readByte(&mut self) -> Option<u8> {
        let mut a = [0u8;1];
        match self.read(&mut a){
            Ok(_) => Some(a[0]),
            _ => None
        }
    }

    fn last_error(&self) -> Status {
        err.clone()
    }

    fn clear_error(&mut self) -> () {
        self.err = Status::Ok(0);
    }
}

impl !Sync for RandomDevice{}
