use std::pin::Pin;
use std::io::{Read, Error, Write};

#[derive(Copy,Clone)]
pub struct Shared<T: ?Sized>{
    v: *mut T
}

impl<T: ?Sized> Shared<T>{
    pub unsafe fn wrap(v: *mut T) -> Shared<T>{
        Self{v}
    }

    pub unsafe fn share(v: Box<T>) -> Shared<T>{
        Self{v: Box::into_raw(v)}
    }

    pub fn inner(&self) -> *mut T{
        return self.v
    }
}
impl<T: Read + ?Sized> Read for Shared<T>{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        unsafe{(*self.v).read(buf)}
    }
}

impl<T: Write + ?Sized> Write for Shared<T>{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        unsafe{(*self.v).write(buf)}
    }

    fn flush(&mut self) -> Result<(), Error> {
       unsafe{(*self.v).flush()}
    }

}

