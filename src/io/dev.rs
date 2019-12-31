use crate::io::{InputStream, Status, OutputStream};
use crate::io::Status::Error;

extern crate getrandom;

#[derive(Default)]
pub struct NullDevice{

}

impl InputStream for NullDevice{
    fn read(&mut self, out: &mut [u8]) -> usize {
        0
    }

    fn readByte(&mut self) -> Option<u8> {
        Option::None
    }

    fn check_status(&self) -> Status {
        Status::Ok
    }

    fn clear_error(&mut self) -> () {}
}

impl OutputStream for NullDevice{
    fn write(&mut self, out: &[u8]) -> usize {
        out.len()
    }

    fn writeByte(&mut self, val: u8) -> () {

    }

    fn check_status(&self) -> Status {
        Status::Ok
    }

    fn clear_error(&mut self) -> () {

    }

    fn flush(&mut self) -> () {

    }
}
#[derive(Default)]
pub struct ZeroDevice{

}

impl InputStream for ZeroDevice{
    fn read(&self, out: &mut [u8]) -> usize {
        for mut b in out {
            *b = 0;
        }
        return out.len();
    }

    fn readByte(&self) -> Option<u8> {
        return Some(0)
    }

    fn check_status(&self) -> Status {
        Status::Ok
    }

    fn clear_error(&self) -> () {
        ()
    }
}

impl OutputStream for ZeroDevice{
    fn write(&self, out: &[u8]) -> usize {
        return out.len()
    }

    fn writeByte(&self, val: u8) -> () {

    }

    fn check_status(&self) -> Status {
        Ok
    }

    fn clear_error(&self) -> () {

    }

    fn flush(&self) -> () {

    }
}



pub struct RandomDevice{
    err: Option<getrandom::Error>
}

impl Default for RandomDevice{
    fn default() -> Self {
        RandomDevice{err: None}
    }
}

impl InputStream for RandomDevice{
    fn read(&mut self, out: &mut [u8]) -> usize {
        match getrandom::getrandom(out){
            Ok(()) => out.len(),
            Err(e) => {
                self.err = Some(e);
                0
            }
        }

    }

    fn readByte(&mut self) -> Option<u8> {
        let mut a = [0u8;1];
        if self.read(&mut a) != 1{
            return None
        }else{
            Some(a[0])
        }
    }

    fn check_status(&self) -> Status {
        match self.err{
            Some(e) => Status::Error(e.to_string()),
            None => Status::Ok
        }
    }

    fn clear_error(&mut self) -> () {
        self.err = None;
    }
}
