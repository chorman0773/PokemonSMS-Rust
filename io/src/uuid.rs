//! Support for the LCS4 uuid structure type
//!

use std::fmt::{Display, LowerHex, UpperHex};

#[cfg(feature = "uuid_v1")]
use std::time::SystemTime;
#[cfg(feature = "uuid_v1")]
use uuid::Uuid as V1Generator;

use crate::data::{DeserializeCopy, Deserializeable, Serializeable};

///
/// A universally Unique Identifier,
///  in a format which can be serialized according to LCS 4
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UUID {
    low: u64,
    high: u64,
}

impl UUID {
    ///
    /// Constructs a new UUID from the given components
    pub const fn new(low: u64, high: u64) -> Self {
        Self { low, high }
    }

    ///
    /// Extracts the components of self into a tuple of (low,high)
    pub const fn into_fields(self) -> (u64, u64) {
        (self.low, self.high)
    }

    ///
    /// The NIL UUID, with both components set to 0
    pub const NIL: UUID = UUID::new(0, 0);

    ///
    /// Generates a random (v4) UUID
    #[cfg(feature = "random_uuid")]
    pub fn random() -> Self {
        use rand::prelude::*;
        let (mut high, mut low) = thread_rng().gen();
        high = (high & !0xF000) | 0x4000;
        low = low & !0x8000000000000000;
        UUID { low, high }
    }

    ///
    /// Returns a new Version 1 UUID
    #[cfg(feature = "uuid_v1")]
    pub fn time_based() -> Self {
        use rand::prelude::*;
        use uuid::v1::{Context, Timestamp};
        static SEQ: Context = Context::new(0);
        let bytes = thread_rng().gen::<[u8; 6]>();
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let ts = Timestamp::from_unix(&SEQ, time.as_secs(), time.subsec_nanos());
        let other = V1Generator::new_v1(ts, &bytes).unwrap();
        let fields = other.as_fields();
        let high = ((fields.0 as u64) << 32) | ((fields.1 as u64) << 16) | (fields.2 as u64);
        let low = u64::from_be_bytes(*fields.3);
        UUID { high, low }
    }
}

impl From<u128> for UUID {
    fn from(v: u128) -> Self {
        Self::new(v as u64, (v >> 64) as u64)
    }
}

impl From<UUID> for u128 {
    fn from(u: UUID) -> Self {
        ((u.high as u128) << 64) | (u.low as u128)
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h0 = (self.high >> 32) as u32;
        let h1 = (self.high & 0xffff0000 >> 16) as u16;
        let h2 = (self.high & 0xffff) as u16;
        let l0 = (self.low >> 48) as u16;
        let l1 = self.low & 0xffffffffffff;
        f.write_fmt(format_args!(
            "{:08x}-{:08x}-{:08x}-{:08x}-{:08x}",
            h0, h1, h2, l0, l1
        ))
    }
}

// Just do the same thing as display, that's the default
impl LowerHex for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h0 = (self.high >> 32) as u32;
        let h1 = (self.high & 0xffff0000 >> 16) as u16;
        let h2 = (self.high & 0xffff) as u16;
        let l0 = (self.low >> 48) as u16;
        let l1 = self.low & 0xffffffffffff;
        f.write_fmt(format_args!(
            "{:08x}-{:08x}-{:08x}-{:08x}-{:08x}",
            h0, h1, h2, l0, l1
        ))
    }
}

// Use `{:X}` for the format of the inner fields
impl UpperHex for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h0 = (self.high >> 32) as u32;
        let h1 = (self.high & 0xffff0000 >> 16) as u16;
        let h2 = (self.high & 0xffff) as u16;
        let l0 = (self.low >> 48) as u16;
        let l1 = self.low & 0xffffffffffff;
        f.write_fmt(format_args!(
            "{:08X}-{:08X}-{:08X}-{:08X}-{:08X}",
            h0, h1, h2, l0, l1
        ))
    }
}

impl Serializeable for UUID {
    fn serialize<W: crate::data::DataOutput + ?Sized>(
        &self,
        output: &mut W,
    ) -> crate::data::Result<()> {
        self.high.serialize(output)?;
        self.low.serialize(output)
    }
}

impl Deserializeable for UUID {
    fn deserialize<R: crate::data::DataInput + ?Sized>(
        &mut self,
        input: &mut R,
    ) -> crate::data::Result<()> {
        self.high.deserialize(input)?;
        self.low.deserialize(input)
    }
}

impl DeserializeCopy for UUID {
    fn deserialize_copy<R: crate::data::DataInput + ?Sized>(
        input: &mut R,
    ) -> crate::data::Result<Self> {
        Ok(Self {
            high: u64::deserialize_copy(input)?,
            low: u64::deserialize_copy(input)?,
        })
    }
}
