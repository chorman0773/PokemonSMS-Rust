//! Implementation of LCS4 IO for

pub use std::io::Error as IoError;
use std::{error::Error as StdError, io::Seek, mem::MaybeUninit, rc::Rc, sync::Arc};
use std::{
    fmt::Display,
    io::{Read, Write},
    slice,
};

///
/// The error type returned from Binary IO functions
#[derive(Debug)]
pub enum Error {
    /// An error indicating that a multibyte read was interrupted by an end of file
    EndOfFile,
    /// An error indicating that an IO operation performed during a read or write caused an error
    IoError(IoError),
    /// An error indicating that some type's validity requirements were not met
    ValidationError(Box<dyn StdError>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EndOfFile => f.write_str("Unexpected End of file"),
            Error::IoError(e) => e.fmt(f),
            Error::ValidationError(b) => b.fmt(f),
        }
    }
}

impl StdError for Error {}

impl From<IoError> for Error {
    fn from(x: IoError) -> Self {
        Self::IoError(x)
    }
}

impl Error {
    /// Indicates that some I/O operation failed
    pub fn validation_error<T: StdError + 'static>(x: T) -> Self {
        Self::ValidationError(Box::from(x))
    }
}

/// The result type produced by this library
pub type Result<T> = std::result::Result<T, Error>;

///
/// Validates the given input by running a FnOnce
/// The resulting error is converted
pub fn validate<T, U, E: StdError + 'static, F: FnOnce(T) -> std::result::Result<U, E>>(
    input: T,
    f: F,
) -> Result<U> {
    f(input).map_err(Error::validation_error)
}

/// An enumeration that stores the possible byte order modes as specified by LCS4
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ByteOrder {
    /// The Big Endian (Most Significant Byte first) Byte Order
    BigEndian,
    /// The Little Endian (Least Significant Byte first) Byte Order
    LittleEndian,
}

impl ByteOrder {
    /// Returns the Byte Order used by the host
    pub const fn native() -> Self {
        #[cfg(target_endian = "little")]
        {
            Self::LittleEndian
        }
        #[cfg(target_endian = "big")]
        {
            Self::BigEndian
        }
        #[cfg(not(any(target_endian = "little", target_endian = "big")))]
        {
            compile_error!("Unsupported Native Byte Order")
        }
    }
}

/// A Trait for types that can perform binary IO Reads
pub trait DataInput: Read {
    ///
    /// Reads exactly bytes.len() bytes into bytes.
    /// Returns an error if an End of File prevents reading the entire array.
    fn read_fully(&mut self, bytes: &mut [u8]) -> Result<()> {
        let len = <Self as Read>::read(self, bytes)?;
        if len != bytes.len() {
            Err(Error::EndOfFile)
        } else {
            Ok(())
        }
    }
    /// Reads a single byte, and returns it, or an error if a byte cannot be read
    fn read_byte(&mut self) -> Result<u8> {
        let mut ret = 0u8;
        self.read_fully(slice::from_mut(&mut ret))?;
        Ok(ret)
    }
    /// Gets the current byte order mode
    fn byte_order(&self) -> ByteOrder;
    /// Sets the current byte order mode
    fn set_byte_order(&mut self, order: ByteOrder);
}

impl<R: DataInput> DataInput for &mut R {
    fn byte_order(&self) -> ByteOrder {
        R::byte_order(self)
    }

    fn set_byte_order(&mut self, order: ByteOrder) {
        R::set_byte_order(self, order)
    }

    fn read_fully(&mut self, bytes: &mut [u8]) -> Result<()> {
        R::read_fully(self, bytes)
    }

    fn read_byte(&mut self) -> Result<u8> {
        R::read_byte(self)
    }
}

impl<R: DataInput> DataInput for Box<R> {
    fn byte_order(&self) -> ByteOrder {
        R::byte_order(self)
    }

    fn set_byte_order(&mut self, order: ByteOrder) {
        R::set_byte_order(self, order)
    }

    fn read_fully(&mut self, bytes: &mut [u8]) -> Result<()> {
        R::read_fully(self, bytes)
    }

    fn read_byte(&mut self) -> Result<u8> {
        R::read_byte(self)
    }
}

///
/// A type that can perform Binary IO Reads by passing through reads to a type that implements Read
pub struct DataInputStream<R: ?Sized> {
    order: ByteOrder,
    read: R,
}

impl<R> DataInputStream<R> {
    ///
    /// Constructs a new DataInputStream from a given stream, in the given byte order mode
    pub const fn new(read: R, order: ByteOrder) -> Self {
        Self { read, order }
    }

    ///
    /// Constructs a new DataInputStream from a given stream, in the native byte order mode
    pub const fn new_native(read: R) -> Self {
        Self::new(read, ByteOrder::native())
    }

    ///
    /// Returns the inner stream
    pub fn into_inner(self) -> R {
        self.read
    }
}

impl<R: Read + ?Sized> Read for DataInputStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read.read(buf)
    }
}

impl<R: Read + ?Sized> DataInput for DataInputStream<R> {
    fn byte_order(&self) -> ByteOrder {
        self.order
    }

    fn set_byte_order(&mut self, order: ByteOrder) {
        self.order = order
    }
}

impl<R: Read + Seek + ?Sized> Seek for DataInputStream<R> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.read.seek(pos)
    }
}

///
/// A trait for types which can be deserialized from a stream of bytes according to LCS 4
pub trait Deserializeable {
    /// Deserializes the bytes on the stream and stores the result in self or returns an error
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()>;
}

impl<T: Deserializeable + ?Sized> Deserializeable for &mut T {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        T::deserialize(self, input)
    }
}

impl<T: Deserializeable + ?Sized> Deserializeable for Box<T> {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        T::deserialize(self, input)
    }
}

impl<T: Deserializeable> Deserializeable for [T] {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        for r in self {
            r.deserialize(input)?;
        }
        Ok(())
    }
}

impl<T: Deserializeable, const N: usize> Deserializeable for [T; N] {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        <[T]>::deserialize(self, input)
    }
}

impl<T: Deserializeable> Deserializeable for Option<T> {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        match self {
            Some(t) => t.deserialize(input),
            None => Ok(()),
        }
    }
}

///
/// A trait for types which can be deserialized and produce a new instance
/// It's intended that this impl should be more efficient then creating a new instance, then reading into it
pub trait DeserializeCopy: Deserializeable + Sized {
    /// Deserializes the bytes on the stream and returns the resulting value or an error
    fn deserialize_copy<R: DataInput + ?Sized>(input: &mut R) -> Result<Self>;
}

impl<T: DeserializeCopy> DeserializeCopy for Box<T> {
    fn deserialize_copy<R: DataInput + ?Sized>(input: &mut R) -> Result<Self> {
        T::deserialize_copy(input).map(Box::new)
    }
}

// MCG too OP
impl<T: DeserializeCopy, const N: usize> DeserializeCopy for [T; N] {
    fn deserialize_copy<R: DataInput + ?Sized>(input: &mut R) -> Result<Self> {
        let mut uninit = MaybeUninit::<[T; N]>::uninit();
        let ptr = uninit.as_mut_ptr().cast::<T>();
        for i in 0..N {
            // SAFETY:
            // i is between 0 and N, and the array has length N
            let ptr = unsafe { ptr.add(i) };
            // SAFETY:
            // ptr is within the array uninit, and thus is valid
            unsafe { ptr.write(T::deserialize_copy(input)?) }
        }
        // SAFETY:
        // uninit is initialized by initializing each element in the above loop
        Ok(unsafe { uninit.assume_init() })
    }
}

/// A Trait for types that can perform binary IO Writes
pub trait DataOutput: Write {
    ///
    /// Writes all of `bytes` to the underlying stream or returns an error
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        Ok(self.write_all(bytes)?)
    }
    ///
    /// Writes byte to the underlying stream or returns an error
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.write_bytes(slice::from_ref(&byte))
    }
    ///
    /// Returns the byte order mode for the stream
    fn byte_order(&self) -> ByteOrder;
    ///
    /// Sets the byte order mode on the stream
    fn set_byte_order(&mut self, order: ByteOrder);
}

///
/// A type that can serialize types according to LCS4
pub struct DataOutputStream<W: ?Sized> {
    order: ByteOrder,
    write: W,
}

impl<W> DataOutputStream<W> {
    ///
    /// Constructs a new DataOutputStream from the given underlying stream and byte order mode
    pub const fn new(write: W, order: ByteOrder) -> Self {
        Self { write, order }
    }

    ///
    /// Constructs a new DataOutputStream from the given underlying stream and the native byte order mode
    pub const fn new_native(write: W) -> Self {
        Self::new(write, ByteOrder::native())
    }

    ///
    /// unwraps the DataOutputStream into the inner stream
    pub fn into_inner(self) -> W {
        self.write
    }
}

impl<W: Write + ?Sized> Write for DataOutputStream<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write.flush()
    }
}

impl<W: Write + ?Sized> DataOutput for DataOutputStream<W> {
    fn byte_order(&self) -> ByteOrder {
        self.order
    }

    fn set_byte_order(&mut self, order: ByteOrder) {
        self.order = order;
    }
}

impl<W: Write + Seek + ?Sized> Seek for DataOutputStream<W> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.write.seek(pos)
    }
}

///
/// A trait for types that can be serialized as a sequence of bytes according to LCS 4
pub trait Serializeable {
    ///
    /// Serializes the type to the stream
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()>;
}

impl<T: Serializeable + ?Sized> Serializeable for &T {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        T::serialize(self, output)
    }
}
impl<T: Serializeable + ?Sized> Serializeable for &mut T {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        T::serialize(self, output)
    }
}

impl<T: Serializeable + ?Sized> Serializeable for Box<T> {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        T::serialize(self, output)
    }
}

impl<T: Serializeable + ?Sized> Serializeable for Rc<T> {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        T::serialize(self, output)
    }
}

impl<T: Serializeable + ?Sized> Serializeable for Arc<T> {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        T::serialize(self, output)
    }
}

impl<T: Serializeable> Serializeable for [T] {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        for t in self {
            t.serialize(output)?;
        }
        Ok(())
    }
}

impl<T: Serializeable, const N: usize> Serializeable for [T; N] {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        for t in self {
            t.serialize(output)?;
        }
        Ok(())
    }
}

impl<T: Serializeable> Serializeable for Option<T> {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        match self {
            Some(v) => v.serialize(output),
            None => Ok(()),
        }
    }
}

impl Deserializeable for u8 {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        input.read_fully(slice::from_mut(self))
    }
}

impl DeserializeCopy for u8 {
    fn deserialize_copy<R: DataInput + ?Sized>(input: &mut R) -> Result<Self> {
        input.read_byte()
    }
}

impl Deserializeable for i8 {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        // SAFETY:
        // the pointer is from self
        // i8 and u8 have the same size, alignment, and representation
        input.read_fully(slice::from_mut(unsafe {
            &mut *(self as *mut i8 as *mut u8)
        }))
    }
}

impl DeserializeCopy for i8 {
    fn deserialize_copy<R: DataInput + ?Sized>(input: &mut R) -> Result<Self> {
        input.read_byte().map(|u| u as i8)
    }
}

impl Serializeable for u8 {
    fn serialize<W: DataOutput + ?Sized>(&self, input: &mut W) -> Result<()> {
        input.write_byte(*self)
    }
}

impl Serializeable for i8 {
    fn serialize<W: DataOutput + ?Sized>(&self, input: &mut W) -> Result<()> {
        input.write_byte(*self as u8)
    }
}

macro_rules! impl_for_tuples{
    () => {
        impl Deserializeable for (){
            fn deserialize<S: DataInput + ?Sized>(&mut self,_: &mut S) -> Result<()>{
                Ok(())
            }
        }
        impl DeserializeCopy for (){
            fn deserialize_copy<S: DataInput + ?Sized>(_: &mut S) -> Result<()>{
                Ok(())
            }
        }
        impl Serializeable for (){
            fn serialize<S: DataOutput + ?Sized>(&self,_: &mut S) -> Result<()>{
                Ok(())
            }
        }
    };
    ($a:ident) => {
        #[allow(non_snake_case)]
        impl<$a : Deserializeable + ?Sized> Deserializeable for ($a ,){
            fn deserialize<S: DataInput + ?Sized>(&mut self,input: &mut S) -> Result<()>{
                let ($a,) = self;
                $a.deserialize(input)
            }
        }
        #[allow(non_snake_case)]
        impl<$a : DeserializeCopy> DeserializeCopy for ($a ,){
            fn deserialize_copy<S: DataInput + ?Sized>(input: &mut S) -> Result<Self>{
                Ok((<$a>::deserialize_copy(input)?,))
            }
        }
        #[allow(non_snake_case)]
        impl<$a : Serializeable + ?Sized> Serializeable for ($a ,){
            fn serialize<S: DataOutput + ?Sized>(&self,input: &mut S) -> Result<()>{
                let ($a,) = self;
                $a.serialize(input)
            }
        }
    };
    ($($leading:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($leading: Deserializeable),+ +?Sized> Deserializeable for ($($leading),+){
            fn deserialize<S: DataInput + ?Sized>(&mut self,input: &mut S) -> Result<()>{
                let ($($leading),+,) = self;
                $({$leading .deserialize(input)?})*
                Ok(())
            }
        }
        #[allow(non_snake_case)]
        impl<$($leading: DeserializeCopy),*> DeserializeCopy for ($($leading),* ){
            fn deserialize_copy<S: DataInput + ?Sized>(input: &mut S) -> Result<Self>{
                Ok(($($leading::deserialize_copy(input)?),*))
            }
        }
        #[allow(non_snake_case)]
        impl<$($leading: Serializeable),+ +?Sized> Serializeable for ($($leading),+){
            fn serialize<S: DataOutput + ?Sized>(&self,input: &mut S) -> Result<()>{
                let ($($leading),+,) = self;
                $({$leading .serialize(input)?})*
                Ok(())
            }
        }
    };
}

impl_for_tuples!();
impl_for_tuples!(A);
impl_for_tuples!(A, B);
impl_for_tuples!(A, B, C);
impl_for_tuples!(A, B, C, D);
impl_for_tuples!(A, B, C, D, E);
impl_for_tuples!(A, B, C, D, E, F);
impl_for_tuples!(A, B, C, D, E, F, G);
impl_for_tuples!(A, B, C, D, E, F, G, H);
impl_for_tuples!(A, B, C, D, E, F, G, H, I);
impl_for_tuples!(A, B, C, D, E, F, G, H, I, J);
impl_for_tuples!(A, B, C, D, E, F, G, H, I, J, K);
impl_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);

macro_rules! impl_for_primitives{
    [$($ty:ty),+] => {
        $(
            impl Deserializeable for $ty{
                fn deserialize<R: DataInput + ?Sized>(&mut self,input: &mut R) -> Result<()>{
                    let mut bytes = [0u8;std::mem::size_of::<$ty>()];
                    input.read_fully(&mut bytes)?;
                    *self = match input.byte_order(){
                        ByteOrder::BigEndian => <$ty>::from_be_bytes(bytes),
                        ByteOrder::LittleEndian => <$ty>::from_le_bytes(bytes)
                    };
                    Ok(())
                }
            }
            impl DeserializeCopy for $ty{
                fn deserialize_copy<R: DataInput + ?Sized>(input: &mut R) -> Result<Self>{
                    let mut bytes = [0u8;std::mem::size_of::<$ty>()];
                    input.read_fully(&mut bytes)?;
                    Ok(match input.byte_order(){
                        ByteOrder::BigEndian => <$ty>::from_be_bytes(bytes),
                        ByteOrder::LittleEndian => <$ty>::from_le_bytes(bytes)
                    })
                }
            }
            impl Serializeable for $ty{
                fn serialize<W: DataOutput + ?Sized>(&self,output: &mut W) -> Result<()>{
                    let bytes = match output.byte_order(){
                        ByteOrder::BigEndian => <$ty>::to_be_bytes(*self),
                        ByteOrder::LittleEndian => <$ty>::to_le_bytes(*self)
                    };
                    output.write_bytes(&bytes)
                }
            }
        )+
    }
}

impl_for_primitives![i16, u16, i32, u32, i64, u64, i128, u128, f32, f64];

impl Deserializeable for String {
    fn deserialize<R: DataInput + ?Sized>(&mut self, input: &mut R) -> Result<()> {
        let size = u16::deserialize_copy(input)? as usize;
        let mut vec = vec![0u8; size];
        input.read_fully(&mut vec)?;
        *self = validate(vec, String::from_utf8)?;
        Ok(())
    }
}

impl DeserializeCopy for String {
    fn deserialize_copy<R: DataInput + ?Sized>(input: &mut R) -> Result<Self> {
        let size = u16::deserialize_copy(input)? as usize;
        let mut vec = vec![0u8; size];
        input.read_fully(&mut vec)?;
        validate(vec, String::from_utf8)
    }
}

///
/// An error type that indicates a particular value is outside of a range imposed for {de,}serialization
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct OutOfRange<T>(pub T);

impl<T: Display> Display for OutOfRange<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} is not in range for this operation",
            &self.0
        ))
    }
}

impl<T> StdError for OutOfRange<T> where Self: std::fmt::Debug + Display {}

impl Serializeable for String {
    fn serialize<W: DataOutput + ?Sized>(&self, output: &mut W) -> Result<()> {
        let size = self.len();
        if size > u16::MAX as usize {
            Err(Error::validation_error(OutOfRange(size)))
        } else {
            (size as u16).serialize(output)?;
            output.write_bytes(self.as_bytes())
        }
    }
}
