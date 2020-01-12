

use std::convert::AsMut;
use std::ops::{Shr, Shl, DerefMut, Deref};
use crate::version::Version;
use std::mem::{self,MaybeUninit};
use std::borrow::Borrow;
use core::panicking::panic;
use crate::io::{InputStream, Status, OutputStream};


#[derive(Copy,Clone,Eq,PartialEq)]
enum ByteOrder{
    BigEndian = 0,
    LittleEndian = 1,
    Native = 2
}

pub trait DataInput{
    fn readFully(&mut self,out:&mut [u8]) -> Result<(),std::string::String>;
    fn readSingle(&mut self) -> Result<u8,std::string::String>;
    fn byte_order(&self) -> ByteOrder;
}

pub trait BinaryIOReadable{
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self,std::string::String>;
}

pub trait ReadTo{
    fn read_to<Input: DataInput>(&mut self,din:&mut Input) -> Result<&Self,std::string::String>;
}

impl<T: BinaryIOReadable> ReadTo for T{
    fn read_to<Input: DataInput>(&mut self, din: &mut Input) -> Result<&mut Self, std::string::String> {
        match Self::read(din){
            Ok(val) =>{
                *self = val;
                Ok(self)
            },
            Err(e) => Err(e)
        }
    }
}

impl<T: BinaryIOReadable,Sz: usize> BinaryIOReadable for [T;Sz]{
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
        unsafe{
            let mut a: MaybeUninit<Self> = MaybeUninit::uninit();
            let mut ptr = a.as_mut_ptr() as *mut T;
            for i in [0..Sz]{
                std::ptr::write(ptr.offset(i),T::read(din)?);
            }
            return Ok(a.assume_init())
        } //Yes this is unsafe. I'm doing the unsafe stuff correctly though
    }
}


impl<T: ReadTo> ReadTo for [T]{
    fn read_to<Input: DataInput>(&mut self,din: &mut Input) -> Result<&Self,std::string::String>{
        for a in self{
            a.read_to(din)?;
        }
        Ok(self)
    }
}

impl<T: ReadTo> ReadTo for Box<T>{
    fn read_to<Input: DataInput>(&mut self, din: &mut Input) -> Result<&Self, std::string::String> {
        self.deref_mut().read_to(din)?;
        Ok(self)
    }
}

impl ReadTo for [u8]{
    fn read_to<Input: DataInput>(&mut self, din: &mut Input) -> Result<&Self, std::string::String> {
        din.readFully(self)?;
        Ok(self)
    }
}


impl<T:ReadTo,S:DataInput> Shr<&mut T> for &mut S {
    type Output = Self;

    fn shr(self, rhs: &mut T) -> Self::Output {
        match rhs.read_to(din){
            Ok(_) => self,
            Err(e) => panic!(e)
        }
    }
}



impl BinaryIOReadable for u8 {
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self,std::string::String>{
        din.readSingle()
    }
}
impl BinaryIOReadable for i8 {
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
        din.readSingle().map(|v| v as i8)
    }
}

impl BinaryIOReadable for bool{
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
        match din.readSingle(){
            Ok(0) => Ok(false),
            Ok(1) => Ok(true),
            Ok(_)=> Err("invalid bool value".to_string()),
            Err(e) => Err(e)
        }
    }
}

impl BinaryIOReadable for u16 {
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self,std::string::String> {
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
    fn read(din: &mut dyn DataInput) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Version) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
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
    fn read(din: &mut dyn DataInput) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
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
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self,std::string::String> {
        match u16::read(din){
            Ok(len)=>{
                let mut val = std::string::String::new();
                val.reserve(len as usize);
                let mut box_val = val.into_bytes();
                din.readFully(box_val.as_mut_slice())?;
                std::string::String::from_utf8(box_val).map_err(|err|err.to_string())
            },
            Err(e)=>Err(e)
        }
    }
}

impl BinaryIOReadable for local_duration{
    fn read<Input: DataInput>(din: &mut Input) -> Result<Self, std::string::String> {
        match i64::read(din){
            Ok(seconds)=>{
                match u32::read(din){
                    Ok(nanos) => if nanos<1000000000{
                        Ok(local_duration{seconds,nanos})
                    }else {
                        Err("Nanos value out of range".to_string())
                    },
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }
}


impl<T: BinaryIOReadable> BinaryIOReadable for Option<T>{
    fn read(din: &mut DataInput) -> Result<Self, std::string::String> {
        T::read(din).map(|val|{Some(val)})
    }
}


pub struct DataInputStream<'a,Stream: InputStream>{
    wrapped: &'a mut Stream,
    endianness: ByteOrder
}

impl<'a,Stream: InputStream> DataInputStream<'a,Stream>{
    pub(crate) fn new(wrapped: &'a mut Stream, endianness: ByteOrder) -> Self{
        DataInputStream{wrapped,endianness}
    }
}

impl<'a,Stream: InputStream> InputStream for DataInputStream<'a,Stream>{
    fn read(&mut self, out: &mut [u8]) -> Status {
        self.wrapped.read(out)
    }

    fn readByte(&mut self) -> Option<u8> {
        self.wrapped.readByte()
    }

    fn last_error(&self) -> Status {
        self.wrapped.check_status()
    }

    fn clear_error(&mut self) -> () {
        self.wrapped.clear_error()
    }
}

impl<'a,Stream: InputStream> DataInput for DataInputStream<'a,Stream>{
    fn readFully(&mut self, out: &mut [u8]) -> Result<(), std::string::String> {
        match self.read(out){
            Status::Ok(sz) =>{
                if sz==out.len(){
                    Ok(())
                }else{
                    Err("Read not fufilled".to_string())
                }
            },
            Status::Error(e) => Err(e),
            Status::Eof => Err("Eof on Stream".to_string())
        }
    }

    fn readSingle(&mut self) -> Result<u8,std::string::String> {
        if Some(t)=self.wrapped.readByte(){
            Ok(t)
        }else{
            Err("Unexpected EOF".to_string())
        }
    }

    fn byte_order(&self) -> ByteOrder {
        endianness
    }
}

impl<'a,'b: 'a,S: InputStream> From<DataInputStream<'a,S>> for DataInputStream<'b,S>{
    fn from(val: DataInputStream<'a, S>) -> Self {
        Self::new(val.wrapped,val.endianness)
    }
}

pub trait DataOutput{
    fn writeBytes(&mut self,bytes:&[u8]);
    fn writeByte(&mut self,byte:u8);
    fn byte_order(&self) -> ByteOrder;
}

pub trait BinaryIOWritable{
    fn write<S: DataOutput>(&self,out:&mut S);
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
    fn write<S: DataOutput>(&self, out: &mut S) {
        out.writeByte(*self)
    }
}

impl BinaryIOWritable for i8{
    fn write<S: DataOutput>(&self,out:&mut S){
        out.writeByte(*self as u8);
    }
}

impl BinaryIOWritable for bool{
    fn write<S: DataOutput>(&self,out:&mut S){
        (self as u8).write(out);
    }
}

impl BinaryIOWritable for u16{
    fn write<S: DataOutput>(&self, out: &mut S) {
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
    fn write<S: DataOutput>(&self, out: &mut S) {
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
    fn write<S: DataOutput>(&self, out: &mut S) {
        let bytes: [u8;2] = unsafe{ std::mem::transmute(self.to_serial().from_be())};
        out.writeBytes(&bytes);
    }
}

impl BinaryIOWritable for u32{
    fn write<S: DataOutput>(&self, out: &mut S) {
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
    fn write<S: DataOutput>(&self, out: &mut S) {
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
    fn write<S: DataOutput>(&self, out: &mut S) {
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
    fn write<S: DataOutput>(&self, out: &mut S) {
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
    fn write<S: DataOutput>(&self, out: &mut S) {
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
    fn write<S: DataOutput>(&self, out: &mut S) {
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


impl<T: BinaryIOWritable> BinaryIOWritable for Box<T>{
    fn write<S: DataOutput>(&self, out: &mut S) {
        self.borrow().write(out)
    }
}

pub struct DataOutputStream<'a,S: OutputStream>{
    wrapped: &'a mut S,
    endianness: ByteOrder
}

impl<'a,S: OutputStream> DataOutputStream<'a,S>{
    pub fn new(wrapped: &'a S,endianness: ByteOrder) -> Self{
        Self{wrapped,endianness}
    }
}

impl<'a,S: OutputStream> OutputStream for DataOutputStream<'a,S>{
    fn write(&mut self, out: &[u8]) -> Status {
        self.wrapped.write(out)
    }

    fn writeByte(&mut self, val: u8) -> Option<()> {
        self.wrapped.writeByte(val)
    }

    fn last_error(&self) -> Status {
        self.wrapped.last_error()
    }

    fn clear_error(&mut self) -> () {
        self.wrapped.clear_error()
    }

    fn flush(&mut self) -> () {
        self.wrapped.flush()
    }
}

impl<'a,S: OutputStream> DataOutput for DataOutputStream<'a,S>{
    fn writeBytes(&mut self, bytes: &[u8]) {
        self.wrapped.write(bytes);
    }

    fn writeByte(&mut self, byte: u8) {
        self.wrapped.writeByte(byte);
    }

    fn byte_order(&self) -> ByteOrder {
        self.endianness
    }
}

impl<'a,'b: 'a,S: OutputStream> From<DataOutputStream<'a,S>> for DataOutputStream<'b,S>{
    fn from(val: DataOutputStream<'a, S>) -> Self {
        Self::new(val.wrapped,val.endianness)
    }
}


impl BinaryIOWritable for !{
    fn write<S: DataOutput>(self, out: &mut S) {
        self
    }
}

impl ReadTo for !{
    fn read_to<Input: DataInput>(self, din: &mut Input) -> Result<&Self, String> {
        self
    }
}

