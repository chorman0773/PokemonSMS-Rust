

extern crate openssl;

use std::mem::MaybeUninit;

type Result<T> = std::result::Result<T,std::string::String>;

#[derive(Eq,PartialEq,Copy,Clone)]
pub enum Endianess{
    BigEndian,
    LittleEndian,
    Native
}

pub use Endianess::*;
use std::io::{Read, Error, Seek, SeekFrom, Write, Take, Bytes, Chain, IoSliceMut};
use std::borrow::BorrowMut;
use std::ops::Shr;
use self::openssl::sign::Verifier;
use self::openssl::error::ErrorStack;

impl Default for Endianess{
    fn default() -> Self {
        Endianess::Native
    }
}

pub trait DataInput{
    ///
    /// Reads enough bytes to fully fill the passed buffer, and stores them in that buffer.
    /// If not enough bytes are available (for example, because an EOF was reached), an error occurs.
    /// Note that no guarantees are made about the content of the buffer if an error occurs.
    /// This permits implementations to read directly into out, and detect the EOF after attempting to do so.
    fn read_fully(&mut self,out: &mut [u8]) -> Result<()>;
    ///
    /// Reads a single byte and returns it.
    fn read_single(&mut self) -> Result<u8>;

    ///
    /// Returns the current byte order.
    fn byte_order(&self) -> Endianess;
    fn set_byte_order(&mut self,byte_order: Endianess);

    fn read_value<T: ReadCopy>(&mut self) -> Result<T>{
        T::read(self)
    }
}

pub trait ReadCopy : Sized{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self>;
}

pub trait Readable{
    fn read_from<S: DataInput + ?Sized>(&mut self,din: &mut S) -> Result<()>;
}

default impl<T: ReadCopy> Readable for T{
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        *self = <T as ReadCopy>::read(din)?;
        Ok(())
    }
}

unsafe trait Primitive{
    fn to_order(self,order: Endianess)->Self;
    fn from_order(self,order: Endianess)->Self;
}

impl<T: Primitive> ReadCopy for T{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self>{
        let mut buf = [0u8; std::mem::size_of::<T>()];
        din.read_fully(&buf)?;
        unsafe{
            let val:T = std::mem::transmute(buf);
            Ok(val.from_order(din.byte_order()))
        }
    }
}

impl ReadCopy for u8{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        din.read_single()
    }
}

impl ReadCopy for i8{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        din.read_single().map(|v|v as i8)
    }
}

impl ReadCopy for bool{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        match u8::read(din)?{
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Invalid Boolean value".to_string())
        }
    }
}

macro_rules! impl_primitive{
    ($type:ident) =>{
        unsafe impl Primitive for $type{
            fn to_order(self,order: Endianess) -> Self{
                match order{
                    BigEndian => Self::to_be(self),
                    LittleEndian => Self::to_le(self),
                    Native => self
                }
            }

            fn from_order(self, order: Endianess) -> Self {
                match order{
                    BigEndian => Self::from_be(self),
                    LittleEndian => Self::from_le(self),
                    Native => self
                }
            }
        }
    }
}

impl_primitive!(u16);
impl_primitive!(i16);
impl_primitive!(u32);
impl_primitive!(i32);
impl_primitive!(u64);
impl_primitive!(i64);
impl_primitive!(u128);
impl_primitive!(i128);

impl ReadCopy for f32{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        Ok(f32::from_bits(u32::read(din)?))
    }
}

impl ReadCopy for f64{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        Ok(f64::from_bits(u64::read(din)?))
    }
}

impl<T: ReadCopy> ReadCopy for MaybeUninit<T>{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        T::read(din).map(|t|MaybeUninit::new(t))
    }
}

impl<T: ReadCopy,const N: usize> ReadCopy for [T;N]{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        let mut ret: MaybeUninit<[T;N]> = MaybeUninit::uninit();
        for i in 0..N{
            unsafe{ ((ret.as_mut_ptr() as *mut T).offset( i as isize) as *mut T).write(T::read(din)?)};
        }
        Ok(unsafe{ret.assume_init()})
    }
}

impl<T: ReadCopy> ReadCopy for Box<T>{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        Ok(Box::new(T::read(din)?))
    }
}

impl ReadCopy for std::string::String{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        let sz = u16::read(din)?;
        let mut vec = Vec::<u8>::with_capacity(sz as usize);
        vec.resize(sz as usize,0u8);
        let a = vec.as_mut_slice();
        a.read_from(din)?;
        String::from_utf8(vec).map_err(|e|e.to_string())
    }
}

impl Readable for [u8]{
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        din.read_fully(self)
    }
}


impl<T: Readable> Readable for [T]{
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        for a in self.iter_mut(){
            a.read_into(din)?;
        }
        Ok(())
    }
}

impl<T: Readable + ?Sized> Readable for Box<T>{
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        self.borrow_mut().read_into(din)
    }
}

impl<A: ReadCopy> ReadCopy for (){
    fn read<S: DataInput + ?Sized>(_: &mut S) -> Result<Self>{
        Ok(())
    }
}

impl<A: ReadCopy> ReadCopy for (A,){
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        Ok((A::read(din)?,))
    }
}

impl<A: ReadCopy,B: ReadCopy> ReadCopy for (A,B){
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        Ok((A::read(din)?,B::read(din)?))
    }
}

impl<A: ReadCopy,B: ReadCopy,C: ReadCopy> ReadCopy for (A,B,C){
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        Ok((A::read(din)?,B::read(din)?,C::read(din)?))
    }
}

impl<A: ReadCopy,B: ReadCopy,C: ReadCopy,D: ReadCopy> ReadCopy for (A,B,C,D){
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self> {
        Ok((A::read(din)?,B::read(din)?,C::read(din)?,D::read(din)?))
    }
}

impl Readable for (){
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        Ok(())
    }
}

impl<A: Readable + ?Sized> Readable for (A,){
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        let (a,) = self;
        a.read_into(din)
    }
}

impl<A: Readable + ?Sized,B: Readable> Readable for (B,A){
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        let (b,a) = self;
        b.read_into(din)?;
        a.read_into(din)
    }
}

impl<A: Readable + ?Sized,B: Readable,C: Readable> Readable for (B,C,A){
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        let (b,c,a) = self;
        b.read_into(din)?;
        c.read_into(din)?;
        a.read_into(din)
    }
}

impl<A: Readable + ?Sized,B: Readable,C: Readable,D: Readable> Readable for (B,C,D,A){
    fn read_from<S: DataInput + ?Sized>(&mut self, din: &mut S) -> Result<()> {
        let (b,c,d,a) = self;
        b.read_into(din)?;
        c.read_into(din)?;
        d.read_into(din)?;
        a.read_into(din)
    }
}

pub struct DataInputStream<'a,I: Read + ?Sized + 'a>{
    read: &'a I,
    order: Endianess
}

impl<'a,I: Read + ?Sized + 'a> DataInputStream<'a,I>{
    pub fn new(read: &'a I,order: Endianess) -> Self{
        Self{read,order}
    }
}

impl<'a,I: Read + ?Sized + 'a> DataInput for DataInputStream<'a,I>{
    fn read_fully(&mut self, out: &mut [u8]) -> Result<()> {
        if self.read.read(out)? < out.len(){
            Err("Unexpected EOF in read_fully".to_string())
        }else{
            Ok(())
        }
    }

    fn read_single(&mut self) -> Result<u8> {
        let mut v = [0u8];
        self.read_fully(&mut v)?;
        Ok(v[0])
    }

    fn byte_order(&self) -> Endianess {
        self.order
    }

    fn set_byte_order(&mut self, byte_order: Endianess) {
        self.order = byte_order;
    }
}

pub use std::io::Result as IOResult;

impl<'a,I: Read +?Sized +'a> Read for DataInputStream<'a,I>{
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        self.read.read(buf)
    }
}


struct NullDevice;

impl Read for NullDevice{
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize,Error> {
        Ok(0)
    }
}

impl Write for NullDevice{
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, Error> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::result::Result<(), Error> {
        Ok(())
    }
}

struct ZeroDevice;

impl Read for ZeroDevice{
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize,Error> {
        for i in buf.iter_mut(){
            *i = 0u8; // This better be vectorized by rust
        }
        Ok(buf.len())
    }
}

impl Write for ZeroDevice{
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, Error> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::result::Result<(), Error> {
        Ok(())
    }
}

impl Seek for ZeroDevice{
    fn seek(&mut self, pos: SeekFrom) -> std::result::Result<u64,Error> {
        Ok(0)
    }
}

impl<A: Readable + ?Sized,S: DataInput + ?Sized> Shr<&mut A> for &mut S{
    type Output = Self;

    fn shr(self, rhs: &mut A) -> Self::Output{
        if let Err(s) = rhs.read_from(self){
            panic!(s);
        }
        self
    }
}

pub struct VerifyingReader<'a,'b,I: Read + 'a + ?Sized>{
    reader: &'a mut I,
    verifier: Verifier<'b>
}

impl<'a,'b,I: Read + 'a + ?Sized> VerifyingReader<'a,'b,I>{
    pub fn new(reader: &'a mut I,verifier: Verifier<'b>) -> Self{
        Self{reader,verifier}
    }
    pub fn verify(&self,signature: &[u8]) -> std::result::Result<bool,ErrorStack>{
        self.verifier.verify(signature)
    }
}
impl<'a,'b,I: Read + 'a + ?Sized> Read for VerifyingReader<'a,'b,I>{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let sz = self.reader.read(buf)?;
        self.verifier.update(&buf[..sz])?;
        Ok(sz)
    }
}


pub trait DataOutput{
    fn write_bytes(&mut self,bytes: &[u8]) -> Result<()>;
    fn write_single(&mut self,byte: u8) -> Result<()>;
    fn byte_order(&self) -> Endianess;
}

pub trait Writeable{
    fn write<S: DataOutput>(&self,out: &mut S) -> Result<()>;
    fn write_consume<S: DataOutput>(self,out:&mut S)-> Result<()>
        where Self: Sized{
        self.write(out)
    }
}

impl<P: Primitive> Writeable for P{
    fn write<S: DataOutput>(&self, out: &mut S) -> Result<()> {
        let bytes = unsafe{ std::mem::transmute::<_,[u8;std::mem::size_of::<P>()]>(self.to_order(out.byte_order()))};
        out.write_bytes(&bytes)
    }
}

impl Writeable for f32{
    fn write<S: DataOutput>(&self, out: &mut S) -> Result<()> {
        self.to_bits().write(out)
    }
}

impl Writeable for f64{
    fn write<S: DataOutput>(&self, out: &mut S) -> Result<()> {
        self.to_bits().write(out)
    }
}

impl<T: Writeable,const N: usize> Writeable for [T;N]{
    fn write<S: DataOutput>(&self, out: &mut S) -> Result<()> {
        self.iter().try_for_each(|s|s.write())
    }

    fn write_consume<S: DataOutput>(self,out: &mut S) -> Result<()>{
        Ok(for t in self{
            t.write_consume(out)?;
        })
    }
}

impl<T: Writeable> Writeable for [T]{
    fn write<S: DataOutput>(&self, out: &mut S) -> Result<()> {
        self.iter().try_for_each(|s|s.write())
    }
}

impl Writeable for [u8]{
    fn write<S: DataOutput>(&self, out: &mut S) -> Result<()> {
        out.write_bytes(self)
    }
}

impl<St: AsRef<str> + ?Sized> Writeable for St{
    fn write<S: DataOutput>(&self, out: &mut S) -> Result<()> {
        let s = self.as_ref();
        let len = usize::max(s.len(),u16::max_value() as usize) as u16;
        len.write(out)?;
        out.write_bytes(s.as_bytes()[..len])
    }
}


pub struct DataOutputStream<'a,I: Write + ?Sized + 'a>{
    write: &'a mut I,
    order: Endianess
}

impl<'a,I: Write +?Sized +'a> DataOutputStream<'a,I>{
    pub fn new(write: &'a mut I,order: Endianness) -> Self{
        Self{write,order}
    }
    pub fn set_byte_order(&mut self,order: Endianness){
        self.order = order
    }
}

impl<'a,I: Write +?Sized +'a> DataOutput for DataOutputStream<'a,I>{
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.write.write(bytes).map_err(|e|e.to_string())
            .and_then(|s|if s==bytes.len(){return Ok(())}else{Err("Length Error".to_string())})
    }

    fn write_single(&mut self, byte: u8) -> Result<()> {
        self.write_bytes(std::slice::from_ref(&byte))
    }

    fn byte_order(&self) -> Endianess {
        self.order
    }
}

impl<'a,I: Write + ?Sized + 'a> Write for DataOutputStream<'a,I>{
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.flush()
    }
}