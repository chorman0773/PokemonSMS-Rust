//! Support for the LCS4 version structure type
//!

use std::{fmt::Display, num::NonZeroU16, ops::RangeBounds};

use crate::data::{DeserializeCopy, Deserializeable, OutOfRange, Serializeable};

///
/// A two component Version which can be encoded according to LCS4
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    fields: [u8; 2],
}

impl Version {
    ///
    /// Obtains the version from the given pair
    pub const fn from_pair_nonzero(
        major: NonZeroU16,
        minor: u8,
    ) -> Result<Self, OutOfRange<NonZeroU16>> {
        let maj = major.get();
        if maj > 256 {
            Err(OutOfRange(major))
        } else {
            Ok(Self {
                fields: [(maj - 1) as u8, minor],
            })
        }
    }

    ///
    /// Obtains the version from the given pair
    pub const fn from_pair(maj: u16, minor: u8) -> Result<Self, OutOfRange<u16>> {
        if maj > 256 || maj == 0 {
            Err(OutOfRange(maj))
        } else {
            Ok(Self {
                fields: [(maj - 1) as u8, minor],
            })
        }
    }

    ///
    /// Decodes the given version into the fields of the verison, according to LCS 4
    pub const fn from_encoded(v: u16) -> Self {
        Self {
            fields: v.to_be_bytes(),
        }
    }

    ///
    /// Encodes the version into a u16, according to LCS 4.
    pub const fn into_encoded(self) -> u16 {
        u16::from_be_bytes(self.fields)
    }

    ///
    /// Obtains the version with the same major component but a 0 minor component
    pub const fn origin(mut self) -> Version {
        self.fields[0] = 0;
        self
    }

    /// The version 1.0, or the smallest possible version
    pub const V1_0: Version = Version::from_encoded(0);
    /// The version 256.255, or the largest possible version
    pub const V256_255: Version = Version::from_encoded(!0);

    /// Returns a Range of versions that include all version from the origin to the current (inclusive)
    pub fn same_origin(self) -> impl RangeBounds<Version> {
        let origin = self.origin();
        origin..=self
    }

    ///
    /// Obtains the minor field, between 0 and 255 inclusive
    pub const fn minor(self) -> u8 {
        self.fields[1]
    }

    ///
    /// Obtains the major field, between 1 and 256 inclusive
    pub const fn major(self) -> NonZeroU16 {
        // SAFETY:
        // 0<=self.fields[0]<256
        // thereof 1<=self.fields[0]+1<257
        unsafe { NonZeroU16::new_unchecked((self.fields[0] as u16) + 1) }
    }

    ///
    /// Obtains the encoded major field, which is self.major()-1
    pub const fn major_encoded(self) -> u8 {
        self.fields[0]
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.major(), self.minor()))
    }
}

impl Serializeable for Version {
    fn serialize<W: crate::data::DataOutput + ?Sized>(
        &self,
        output: &mut W,
    ) -> std::io::Result<()> {
        self.fields.serialize(output)
    }
}

impl Deserializeable for Version {
    fn deserialize<R: crate::data::DataInput + ?Sized>(
        &mut self,
        input: &mut R,
    ) -> std::io::Result<()> {
        self.fields.deserialize(input)
    }
}

impl DeserializeCopy for Version {
    fn deserialize_copy<R: crate::data::DataInput + ?Sized>(
        input: &mut R,
    ) -> std::io::Result<Self> {
        Ok(Self {
            fields: <[u8; 2]>::deserialize_copy(input)?,
        })
    }
}
