#![allow(non_camel_case_types)]

use crate::io::dataio::{BinaryIOReadable, DataInput, BinaryIOWritable};
use self::uuid::Uuid;

extern crate uuid;

#[derive(Hash,Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]
#[repr(C)]
pub struct UUID(pub(crate) u64,pub(crate) u64);

impl UUID{
    pub const nil: UUID = UUID(0,0);
}

impl From<(u64,u64)> for UUID{
    fn from(val: (u64, u64)) -> Self {
        let (x,y) = val;
        return UUID(x,y)
    }
}

impl From<u128> for UUID{
    fn from(val: u128) -> Self{
        let (x,y) = (((val>>64)&0xFFFFFFFFFFFFFFFF) as u64,(val&0xFFFFFFFFFFFFFFFF) as u64);
        return UUID(x,y)
    }
}

impl From<UUID> for u128{
    fn from(val: UUID) -> Self{
        let UUID(x,y) = val;
        return (x as u128)<<64 | y as u128;
    }
}

impl Default for UUID{
    fn default() -> Self {
        Self::nil
    }
}

impl BinaryIOReadable for UUID{
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        u64::read(din).and_then(|val|{
            u64::read(din).and_then(|val2|{
                Ok(uuid(val,val2))
            })
        })
    }
}

impl BinaryIOWritable for UUID{
    fn write(&self, out: &mut DataOutput) {
        self.0.write(out);
        self.1.write(out);
    }
}

impl From<uuid::Uuid> for UUID{
    fn from(val: Uuid) -> Self {
        return std::u128::from_be(val.as_u128()).into()
    }
}

impl From<UUID> for uuid::Uuid{
    fn from(val: Uuid) -> Self{
        return Uuid::from_u128(std::u128::to_be(val.into()))
    }
}

impl BinaryIOWritable for uuid::Uuid{
    fn write(&self, out: &mut DataOutput) {
        UUID::from(self).write(out);
    }
}

impl BinaryIOReadable for uuid::Uuid{
    fn read(din: &mut dyn DataInput) -> Result<Self, &'static str> {
        Ok(UUID::read(din)?.into())
    }
}


