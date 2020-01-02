use crate::io::dataio::{BinaryIOWritable, DataInput, BinaryIOReadable, ReadTo};
use std::borrow::Borrow;
use crate::uuid::{uuid, UUID};

#[derive(Clone)]
pub enum NBTTag{
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Box<[u8]>),
    String(std::string::String),
    List(List),
    Compound(Compound),
    IntArray(Box<[i32]>),
    LongArray(Box<[i64]>),
    FloatArray(Box<[f32]>),
    DoubleArray(Box<[f64]>),
    UUID(UUID)
}

impl NBTTag{
    fn new_from_tag_type(tag: u8) -> Option<Self>{
        match tag{
            0 => Some(NBTTag::End),
            1 => Some(NBTTag::Byte(0)),
            2 => Some(NBTTag::Short(0)),
            3 => Some(NBTTag::Int(0)),
            4 => Some(NBTTag::Long(0)),
            5 => Some(NBTTag::Float(0f32)),
            6 => Some(NBTTag::Double(0f64)),
            7 => Some(NBTTag::ByteArray(Box::from(&[0u8;0]))),
            8 => Some(NBTTag::String("".to_string())),
            9 => Some(NBTTag::List(Default::default())),
            10 => Some(NBTTag::Compound(Default::default())),
            11 => Some(NBTTag::IntArray(Box::from(&[0u32;0]))),
            12 => Some(NBTTag::LongArray(Box::from(&[0u64;0]))),
            13 => Some(NBTTag::FloatArray(Box::from(&[0f32;0]))),
            14 => Some(NBTTag::DoubleArray(Box::from(&[0f64;0]))),
            15 => Some(NBTTag::UUID(Default::default())),
            _ => None
        }
    }

    fn get_tag_type(&self) -> u8{
        match self{
            NBTTag::End => 0,
            NBTTag::Byte(_) => 1,
            NBTTag::Short(_) => 2,
            NBTTag::Int(_) => 3,
            NBTTag::Long(_) => 4,
            NBTTag::Float(_) => 5,
            NBTTag::Double(_) => 6,
            NBTTag::ByteArray(_) => 7,
            NBTTag::String(_) => 8,
            NBTTag::List(_) => 9,
            NBTTag::Compound(_) => 10,
            NBTTag::IntArray(_) => 11,
            NBTTag::LongArray(_) => 12,
            NBTTag::FloatArray(_) => 13,
            NBTTag::DoubleArray(_) => 14,
            NBTTag::UUID(_) => 15
        }
    }

    fn byte_value(&self) -> Option<i8>{
        match
    }
}

impl Default for NBTTag{
    fn default() -> Self {
        NBTTag::End
    }
}

fn read_length_array<T: BinaryIOReadable + Copy>(arr:&mut Box<[T]>,din: &mut dyn DataInput) -> Result<&Box<[T]>,&'static str>{
    let len = i32::read(din)?;
    if len < 0{
        Err("length must not be negative")
    }
    let mut vec = Vec::<T>::with_capacity(len as usize);
    for i in 0..len{
        let t = T::read(din)?;
        vec.push(t);
    }
    *arr = vec.into_boxed_slice();
    Ok(arr)
}

impl ReadTo for NBTTag{
    fn read_to(&mut self, din: &mut dyn DataInput) -> Result<&Self, &'static str> {
        match self {
            NBTTag::End => Ok(self),
            NBTTag::Byte(val)
            | NBTTag::Short(val)
            | NBTTag::Int(val)
            | NBTTag::Long(val)
            | NBTTag::Float(val)
            | NBTTag::Double(val)
            | NBTTag::String(val)
            | NBTTag::List(val)
            | NBTTag::Compound(val)
            | NBTTag::UUID(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::ByteArray(arr)
            | NBTTag::IntArray(arr)
            | NBTTag::LongArray(arr)
            | NBTTag::FloatArray(arr)
            | NBTTag::DoubleArray(arr) => {
                read_length_array(arr,din)?;
                Ok(self)
            }
        }
    }
}

impl BinaryIOWritable for NBTTag{
    fn write(&self, out: &mut dyn DataOutput) {
        match self{
            NBTTag::End => Ok(self),
            NBTTag::Byte(val)
            | NBTTag::Short(val)
            | NBTTag::Int(val)
            | NBTTag::Long(val)
            | NBTTag::Float(val)
            | NBTTag::Double(val)
            | NBTTag::String(val)
            | NBTTag::List(val)
            | NBTTag::Compound(val)
            | NBTTag::UUID(val) => val.write(out),
            NBTTag::ByteArray(arr)
            | NBTTag::IntArray(arr)
            | NBTTag::LongArray(arr)
            | NBTTag::FloatArray(arr)
            | NBTTag::DoubleArray(arr) => {
                let sz = arr.len() as i32;
                sz.write(out);
                for a in *arr{
                    a.write(out);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct List{
    tag_type: std::mem::Discriminant<NBTTag>,
    list : std::vec::Vec<NBTTag>
}

impl List{
    fn add(&mut self,tag: &NBTTag) -> Option<&NBTTag>{
        if self.list.is_empty() {
            tag_type = std::mem::discriminant(tag);
            self.list.push(tag.clone());
            self.list.last()
        }else if tag_type == std::mem::discriminant(tag){
            self.list.push(tag.clone());
            self.list.last()
        }else {
            None
        }
    }
}

impl Default for List{
    fn default() -> Self {
        Self{tag_type: std::mem::discriminant(&NBTTag::End),list: std::vec::Vec::new()}
    }
}

impl<'a> std::iter::IntoIterator for &'a List{
    type Item = NBTTag;
    type IntoIter = std::slice::Iter<'a,Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl ReadTo for List{
    fn read_to(&mut self, din: &mut dyn DataInput) -> Result<&Self, &'static str> {
        self.list.clear();
        let tag_type = u8::read(din)?;
        let len = i32::read(din)?;
        if len < 0 {
            Err("length must not be negative")
        }
        self.list.reserve(len as usize);
        self.tag_type = std::mem::discriminant(&NBTTag::new_from_tag_type(tag_Type).ok_or("Invalid Tag Type")?);
        for i in 0..len{
            let mut tag = NBTTag::new_from_tag_type(tag_type).unwrap();
            tag.read_to(din);
            self.list.push(tag);
        }
        Ok(self)
    }
}

#[derive(Clone)]
pub struct Compound{
    underlying: std::collections::BTreeMap<std::string::String,NBTTag>
}

impl Compound{

}