extern crate uuid;
use crate::io::*;
use crate::time::Duration as local_duration;
use std::convert::AsMut;
use std::ops::{Shr, Shl};
use crate::version::Version;

enum ByteOrder{
    BigEndian = 0,
    LittleEndian = 1,
    Native = 2
}

pub trait DataInput{
    fn readFully(&mut self,out:&mut [u8]) -> Result<(),&'static str>;
    fn readSingle(&mut self) -> Result<u8,&'static str>;
    fn byte_order(&self) -> ByteOrder;
}

pub trait BinaryIOReadable{
    fn read(din: &mut dyn DataInput) -> Result<Self,&'static str>;
}

impl<T:BinaryIOReadable,S:DataInput> Shr<&mut T> for &mut S {
    type Output = Self;

    fn shr(self, rhs: &mut T) -> Self::Output {
        match T::read(self){
            Ok(t) =>{
                *rhs = t;
                self
            },
            Err(msg) => panic!(msg)
        }
    }
}

impl<S: DataInput> Shr<&mut[u8]> for &mut S{
    type Output = Self;

    fn shr(self, rhs: &mut [u8]) -> Self::Output {
        match self.readFully(rhs){
            Ok(_) => self,
            Err(msg) => panic!(msg)
        }
    }
}

impl<T: BinaryIOReadable,S:DataInput> Shr<&mut[T]> for &mut S{
    type Output = Self;
    fn shr(self,rhs:&mut [T]) -> Self::Output{
        for mut t in rhs{
            match T::read(self){
                Ok(val) => *t = val,
                Err(e) => panic!(e)
            }
        }
        self
    }
}

impl BinaryIOReadable for u8 {
    fn read(din: &mut dyn DataInput) -> Result<Self,&'static str>{
        din.readSingle()
    }
}
impl BinaryIOReadable for i8 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        din.readSingle().map(|v| v as i8)
    }
}

impl BinaryIOReadable for bool{
    fn read(din: &mut DataInput) -> Result<Self, &'static str> {
        din.readSingle().and_then(|v| {
            if v == 0{
                return Ok(false)
            }else if v==1{
                return Ok(true)
            }else{
                Err("Invalid bool value")
            }
        })
    }
}

impl BinaryIOReadable for u16 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;2];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: u16 = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(u16::from_be(val)),
                        LittleEndian=> Ok(u16::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for i16 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;2];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for Version{
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes: [u8;2];
        match din.readFully(bytes.as_mut()){
            Ok(_) =>{
                return Ok(Version::from_serial(unsafe{ std::mem::transmute::<[u8;2],u16>(bytes)}.from_be()))
            }
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for u32 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;4];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for i32 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;4];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for f32 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;4];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for u64 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;8];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for i64 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;8];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for f64 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;8];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for u128 {
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;16];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}
impl BinaryIOReadable for i128 {
    fn read(din: &mut DataInput) -> Result<Self, &'static str> {
        let mut bytes = [0 as u8;16];
        match din.readFully(bytes.as_mut()){
            Ok(_) => {
                unsafe{
                    let val: Self = std::mem::transmute(bytes);
                    match din.byte_order(){
                        BigEndian=> Ok(Self::from_be(val)),
                        LittleEndian=> Ok(Self::from_le(val)),
                        Native=> Ok(val)
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for std::string::String{
    fn read(din: &mut dyn DataInput) -> Result<Self,&'static str> {
        match u16::read(din){
            Ok(len)=>{
                let mut val = std::string::String::new();
                val.reserve(len as usize);
                unsafe {
                    match din.readFully(val.as_bytes_mut()) {
                        Ok(_)=>Ok(val),
                        Err(e)=>Err(e)
                    }
                }
            },
            Err(e)=>Err(e)
        }
    }
}

impl BinaryIOReadable for local_duration{
    fn read(din: &mut DataInput) -> Result<Self, &'static str> {
        match i64::read(din){
            Ok(seconds)=>{
                match u32::read(din){
                    Ok(nanos) => if nanos<1000000000{
                        Ok(local_duration{seconds,nanos})
                    }else {
                        Err("Nanos value out of range")
                    },
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }
}


impl<T: BinaryIOReadable> BinaryIOReadable for Option<T>{
    fn read(din: &mut DataInput) -> Result<Self, &'static str> {
        T::read(din).map(|val|{Some(val)})
    }
}


pub struct DataInputStream<'a>{
    wrapped: &'a mut dyn InputStream,
    endianness: ByteOrder
}

impl DataInputStream{
    fn new<'a>(wrapped: &'a mut dyn InputStream,endianness: ByteOrder) -> std::Box<DataInputStream<'a>>{
        Box(DataInputStream{wrapped,endianness})
    }
}

impl InputStream for DataInputStream{
    fn read(&mut self, out: &mut [u8]) -> usize {
        self.wrapped.read(out)
    }

    fn readByte(&mut self) -> Option<u8> {
        self.wrapped.readByte()
    }

    fn check_status(&self) -> Status {
        self.wrapped.check_status()
    }

    fn clear_error(&mut self) -> () {
        self.wrapped.clear_error()
    }
}

impl DataInput for DataInputStream{
    fn readFully(&mut self, out: &mut [u8]) -> Result<(), &'static str> {
        if self.read(out) != out.len() {
            Err("Unfulfilled read")
        }else {
            Ok(())
        }
    }

    fn readSingle(&mut self) -> Result<u8,&'static str> {
        if Some(t)=self.wrapped.readByte(){
            Ok(t)
        }else{
            Err("Unexpected EOF")
        }
    }

    fn byte_order(&self) -> ByteOrder {
        endianness
    }
}

trait DataOutput{
    fn writeBytes(&mut self,bytes:&[u8]);
    fn writeByte(&mut self,byte:u8);
    fn byte_order(&self) -> ByteOrder;
}

trait BinaryIOWritable{
    fn write(&self,out:&mut dyn DataOutput);
}

impl<T: BinaryIOWritable,S: DataOutput> Shl<T> for &mut S{
    type Output = Self;

    fn shl(self, rhs: T) -> Self::Output {
        rhs.write(self);
        self
    }
}

impl<S: DataOutput> Shl<&[u8]> for &mut S{
    type Output = Self;

    fn shl(self,rhs:&[u8]) -> Self::Output{
        self.writeBytes(rhs);
        self
    }
}

impl<T: BinaryIOWritable,S: DataOutput> Shl<&[T]> for &mut S{
    type Output = Self;

    fn shl(self,rhs:&[T]) -> Self::Output{
        for val in rhs{
            val.write(self)
        }
        self
    }
}

impl BinaryIOWritable for u8{
    fn write(&self, out: &mut dyn DataOutput) {
        out.writeByte(*self)
    }
}

impl BinaryIOWritable for i8{
    fn write(&self,out:&mut dyn DataOutput){
        out.writeByte(*self as u8);
    }
}

impl BinaryIOWritable for bool{
    fn write(&self,out:&mut dyn DataOutput){
        (self as u8).write(out);
    }
}

impl BinaryIOWritable for u16{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;2] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for i16{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;2] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for Version{
    fn write(&self, out: &mut dyn DataOutput) {
        let bytes: [u8;2] = unsafe{ std::mem::transmute(self.to_serial().from_be())};
        out.writeBytes(&bytes);
    }
}

impl BinaryIOWritable for u32{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;4] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for i32{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;4] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for f32{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;4] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for u64{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;8] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for i64{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;8] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for u128{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;16] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}
impl BinaryIOWritable for i128{
    fn write(&self, out: &mut dyn DataOutput) {
        unsafe{
            let bytes: [u8;16] = std::mem::transmute(match out.byte_order(){
                BigEndian => Self::to_be(*self),
                LittleEndian => Self::to_le(*self),
                Native => *self
            });
            out.writeBytes(&bytes);
        }
    }
}

impl BinaryIOWritable for local_duration{
    fn write(&self, out: &mut DataOutput) {
        self.get_seconds().write(out);
        self.get_nanos().write(out);
    }
}




