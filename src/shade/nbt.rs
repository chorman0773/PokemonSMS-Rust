
use crate::uuid::UUID;
use crate::io::{Readable, ReadCopy, DataInput, Writeable, DataOutput};

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

const TAG_ANY_NUMERIC: u8 = 99;

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
}

impl Default for NBTTag{
    fn default() -> Self {
        NBTTag::End
    }
}

fn read_length_array<'a,T: ReadCopy,S: DataInput>(arr:&'a mut Box<[T]>,din: &mut S) -> Result<&'a Box<[T]>,&'static str>{
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

impl Readable for NBTTag{
    fn read_from<Input: DataInput>(&mut self, din: &mut Input) -> Result<&Self, &'static str> {
        match self {
            NBTTag::End => Ok(self),
            NBTTag::Byte(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::Short(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::Int(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::Long(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::Float(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::Double(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::String(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::List(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::Compound(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::UUID(val) => {
                val.read_to(din)?;
                Ok(self)
            },
            NBTTag::ByteArray(arr) => {
                read_length_array(arr,din)?;
                Ok(self)
            }
            NBTTag::IntArray(arr) => {
                read_length_array(arr,din)?;
                Ok(self)
            }
            NBTTag::LongArray(arr) => {
                read_length_array(arr,din)?;
                Ok(self)
            }
            NBTTag::FloatArray(arr) => {
                read_length_array(arr,din)?;
                Ok(self)
            }
            NBTTag::DoubleArray(arr) => {
                read_length_array(arr,din)?;
                Ok(self)
            }
        }
    }
}

impl ReadCopy for NBTTag{
    fn read<S: DataInput + ?Sized>(din: &mut S) -> Result<Self,std::string::String> {
        let compound = din.read_value()?;
        return Ok(NBTTag::Compound(compound))
    }
}

impl Writeable for NBTTag{
    fn write<S: DataOutput>(&self, out: &mut S) {
        match self{
            NBTTag::End => Ok(self),
            NBTTag::Byte(val) => val.write(out),
            NBTTag::Short(val)  => val.write(out),
            NBTTag::Int(val)  => val.write(out),
            NBTTag::Long(val)  => val.write(out),
            NBTTag::Float(val)  => val.write(out),
            NBTTag::Double(val) => val.write(out),
            NBTTag::String(val)=> val.write(out),
            NBTTag::List(val)=> val.write(out),
            NBTTag::Compound(val) => val.write(out),
            NBTTag::UUID(val) => val.write(out),
            NBTTag::ByteArray(arr) => {
                let sz = arr.len() as i32;
                sz.write(out);
                for a in arr.iter(){
                    a.write(out);
                }
            }
            NBTTag::IntArray(arr) => {
                let sz = arr.len() as i32;
                sz.write(out);
                for a in arr.iter(){
                a.write(out);
                }
            }
            NBTTag::LongArray(arr) => {
                let sz = arr.len() as i32;
                sz.write(out);
                for a in arr.iter(){
                    a.write(out);
                }
            }
            NBTTag::FloatArray(arr) => {
            let sz = arr.len() as i32;
            sz.write(out);
            for a in arr.iter(){
            a.write(out);
            }
            }
            NBTTag::DoubleArray(arr) => {
                let sz = arr.len() as i32;
                sz.write(out);
                for a in arr.iter(){
                    a.write(out);
                }
            }
        };
    }
}

#[derive(Clone)]
pub struct List{
    tag_type: u8,
    list : std::vec::Vec<NBTTag>
}

impl List{
    fn add(&mut self,tag: &NBTTag) -> Option<&NBTTag>{
        if self.list.is_empty() {
            self.tag_type = tag.get_tag_type();
            self.list.push(tag.clone());
            self.list.last()
        }else if self.tag_type == tag.get_tag_type(){
            self.list.push(tag.clone());
            self.list.last()
        }else {
            None
        }
    }

    fn iter(&self) -> impl Iterator<Item=&NBTTag>{
        self.list.iter()
    }
}

impl Default for List{
    fn default() -> Self {
        Self{tag_type: 0,list: std::vec::Vec::new()}
    }
}


impl Readable for List{
    fn read_from<S: DataInput>(&mut self, din: &mut S) -> Result<&Self, std::string::String> {
        self.list.clear();
        let tag_type = u8::read(din)?;
        let len = i32::read(din)?;
        if len < 0 {
            return Err("length must not be negative".to_string())
        }
        self.list.reserve(len as usize);
        self.tag_type = tag_type;
        for i in 0..len{
            let mut tag = NBTTag::new_from_tag_type(tag_type).ok_or_else(||"Invalid Tag Type".to_string())?;
            tag.read_from(din)?;
            self.list.push(tag);
        }
        Ok(self)
    }
}

impl Writeable for List{
    fn write<S: DataOutput + ?Sized>(&self, out: &mut S) {
        self.tag_type.write(out);
        self.len.write(out);
        for tag in &self.list{
            tag.write(out);
        }
    }
}

#[derive(Clone)]
pub struct Compound{
    underlying: std::collections::BTreeMap<std::string::String,NBTTag>
}

impl Compound{
    pub fn put(&mut self,name: std::string::String,tag: NBTTag){
        self.underlying.insert(name,tag);
    }
    pub fn get(&self,name: &std::string::String) -> Option<&NBTTag>{
        self.underlying.get(name)
    }
    pub fn get_mut(&mut self,name: &std::string::String) -> Option<&mut NBTTag>{
        self.underlying.get_mut(name)
    }
    pub fn get_or<'a,'b,'c: 'a+'b>(&'a self,name: &std::string::String,other:&'b NBTTag) ->&'c NBTTag{
        if let Some(tag) = self.get(name){
            tag
        }else{
            other
        }
    }
}

impl Readable for Compound{
    fn read_from<Input: DataInput>(&mut self, din: &mut Input) -> Result<&Self, String> {
        self.underlying.clear();
        loop{
            let tag_type = u8::read(din)?;
            if tag_type == 0{
                break;
            }
            let name = std::string::String::read(din)?;
            let mut tag = NBTTag::new_from_tag_type(tag_type).ok_or_else(||"Invalid Tag Type".to_string())?;
            tag.read_to(din)?;
            self.underlying.insert(name,tag);
        };
        Ok(self)
    }
}

impl Writeable for Compound{
    fn write<S: DataOutput>(&self, out: &mut S) {
        for (key,value) in &self.underlying{
            value.get_tag_type().write(out);
            key.write(out);
            value.write(out);
        }
    }
}