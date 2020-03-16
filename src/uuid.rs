#![allow(non_camel_case_types)]



use crate::io::{DataInput, ReadCopy, Writeable, DataOutput};
use uuid::Uuid;

extern crate uuid;

#[derive(Hash,Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]
#[repr(C)]
pub struct UUID(pub(crate) u64,pub(crate) u64);

impl UUID{
    pub const NIL: UUID = UUID(0,0);
}

impl From<(u64,u64)> for UUID{
    fn from(val: (u64, u64)) -> Self {
        let (x,y) = val;
        UUID(x,y)
    }
}

impl From<u128> for UUID{
    fn from(val: u128) -> Self{
        let (x,y) = (((val>>64)&0xFFFFFFFFFFFFFFFF) as u64,(val&0xFFFFFFFFFFFFFFFF) as u64);
        UUID(x,y)
    }
}

impl From<UUID> for u128{
    fn from(val: UUID) -> u128{
        let UUID(x,y) = val;
        (x as u128)<<64 | y as u128
    }
}

impl Default for UUID{
    fn default() -> Self {
        Self::NIL
    }
}

impl ReadCopy for UUID{
    fn read<S: DataInput>(din: &mut S) -> Result<Self, std::string::String> {
        u64::read(din).and_then(|val|{
            u64::read(din).and_then(|val2|{
                Ok(UUID(val,val2))
            })
        })
    }
}

impl Writeable for UUID{
    fn write<S: DataOutput>(&self, out: &mut S) {
        self.0.write(out);
        self.1.write(out);
    }
}

impl From<uuid::Uuid> for UUID{
    fn from(val: Uuid) -> Self {
        u128::from_be(val.as_u128()).into()
    }
}

impl From<UUID> for uuid::Uuid{
    fn from(val: UUID) -> Self{
        Uuid::from_u128(u128::to_be(val.into()))
    }
}

impl Writeable for uuid::Uuid{
    fn write<S: DataOutput>(&self, out: &mut S) {
        UUID::from(*self).write(out);
    }
}

impl ReadCopy for uuid::Uuid{
    fn read<S: DataInput>(din: &mut S) -> Result<Self, std::string::String> {
        Ok(UUID::read(din)?.into())
    }
}


