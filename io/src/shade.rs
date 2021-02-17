//!
//! Structures for serializing and deserializing files in the ShadeNBT format

use std::{
    io::ErrorKind,
    ops::{Deref, DerefMut},
};

use crate::{
    data::{ByteOrder, DeserializeCopy, Deserializeable, Serializeable},
    nbt::compound::NbtCompound,
    version::Version,
};

#[derive(Clone, Debug)]
/// The header of a ShadeNBT File
pub struct ShadeFile {
    magic: [u8; 4],
    version: Version,
    flags: u8,
    compound: NbtCompound,
}

pub mod consts {
    //! Constant values for ShadeNBT

    use crate::version::Version;

    /// The current version of the Shade file format
    pub const SHADE_VERSION: Version = Version::from_encoded(0x0004);

    pub(crate) const SHADE_FLAGS_VERSION: Version = Version::from_encoded(0x0002);

    pub(crate) const SHADE_FLAGS_MASK: u8 = 0xA0;

    pub(crate) const SHADE_FLAGS_ACCEPTED_MASK: u8 = 0xE0;

    ///
    /// The magic number for a ShadeNBT file: "\xADNBT" or [AD 4E 42 54]
    pub const SHADE_MAGIC: [u8; 4] = [0xAD, 0x4E, 0x42, 0x54];
}

impl ShadeFile {
    ///
    /// Creates a new Shade File with the current version, big endian byte order mode, and an empty compound
    pub fn new() -> Self {
        Self {
            magic: consts::SHADE_MAGIC,
            version: consts::SHADE_VERSION,
            flags: 0x00,
            compound: NbtCompound::new(),
        }
    }

    ///
    /// Creates a new Shade File with the current version and the specified Byte Order Mode.
    pub fn with_byte_order(order: ByteOrder) -> Self {
        Self {
            magic: consts::SHADE_MAGIC,
            version: consts::SHADE_VERSION,
            flags: if order == ByteOrder::LittleEndian {
                0x80
            } else {
                0x00
            },
            compound: NbtCompound::new(),
        }
    }

    ///
    /// Creates a new Shade File with the given version in Big Endian Byte Order Mode
    /// Panics if the Version is unsupported.
    pub fn with_version(version: Version) -> Self {
        if consts::SHADE_VERSION < version {
            panic!("Shade Version {} is not implemented", version)
        } else {
            Self {
                magic: consts::SHADE_MAGIC,
                version,
                flags: 0x00,
                compound: NbtCompound::new(),
            }
        }
    }

    ///
    /// Creates a new Shade File with the given version in given Byte Order Mode
    /// Panics if the Version is unsupported, or if Little Endian byte order mode is specified, and a version before 1.2 is specified
    pub fn with_version_and_byte_order(version: Version, byte_order: ByteOrder) -> Self {
        if consts::SHADE_VERSION < version {
            panic!("Shade Version {} is not implemented", version)
        } else if byte_order == ByteOrder::LittleEndian && version < consts::SHADE_FLAGS_VERSION {
            panic!("Shade Version {} does not support Little Endian", version)
        } else {
            Self {
                magic: consts::SHADE_MAGIC,
                version: version,
                flags: if byte_order == ByteOrder::LittleEndian {
                    0x80
                } else {
                    0x00
                },
                compound: NbtCompound::new(),
            }
        }
    }

    ///
    /// Returns the version of the Shade File
    pub fn version(&self) -> Version {
        self.version
    }

    ///
    /// Returns the Byte Order mode of the file. For Shade files versions less than 1.2, this always returns ByteOrder::BigEndian
    pub fn byte_order(&self) -> ByteOrder {
        if (self.flags & 0x80) != 0 {
            ByteOrder::LittleEndian
        } else {
            ByteOrder::BigEndian
        }
    }
}

impl Deref for ShadeFile {
    type Target = NbtCompound;

    fn deref(&self) -> &Self::Target {
        &self.compound
    }
}

impl DerefMut for ShadeFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.compound
    }
}

impl Serializeable for ShadeFile {
    fn serialize<W: crate::data::DataOutput + ?Sized>(
        &self,
        output: &mut W,
    ) -> std::io::Result<()> {
        self.magic.serialize(output)?;
        self.version.serialize(output)?;
        if consts::SHADE_FLAGS_VERSION < self.version {
            self.flags.serialize(output)?;
        }
        if self.flags & 0x80 == 0 {
            output.set_byte_order(ByteOrder::LittleEndian)
        } else {
            output.set_byte_order(ByteOrder::BigEndian)
        }
        self.compound.serialize(output)
    }
}

impl Deserializeable for ShadeFile {
    fn deserialize<W: crate::data::DataInput + ?Sized>(
        &mut self,
        output: &mut W,
    ) -> std::io::Result<()> {
        self.magic.deserialize(output)?;
        if self.magic != consts::SHADE_MAGIC {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid magic (not a shade file)",
            ));
        }
        self.version.deserialize(output)?;
        if consts::SHADE_VERSION < self.version {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Version {} is not implemetented", self.version),
            ));
        }
        if consts::SHADE_FLAGS_VERSION < self.version {
            self.flags.deserialize(output)?;
            if (self.flags & !consts::SHADE_FLAGS_ACCEPTED_MASK) != 0 {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "Invalid flags in mask",
                ));
            }
            self.flags &= consts::SHADE_FLAGS_MASK;
        }
        if self.flags & 0x80 == 0 {
            output.set_byte_order(ByteOrder::LittleEndian)
        } else {
            output.set_byte_order(ByteOrder::BigEndian)
        }
        self.compound.deserialize(output)
    }
}

impl DeserializeCopy for ShadeFile {
    fn deserialize_copy<R: crate::data::DataInput + ?Sized>(
        input: &mut R,
    ) -> std::io::Result<Self> {
        let magic = <[u8; 4]>::deserialize_copy(input)?;
        if magic != consts::SHADE_MAGIC {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid magic (not a shade file)",
            ));
        }
        let version = Version::deserialize_copy(input)?;
        if consts::SHADE_VERSION < version {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Version {} is not implemetented", version),
            ));
        }
        let mut flags;
        if consts::SHADE_FLAGS_VERSION < version {
            flags = u8::deserialize_copy(input)?;
            if (flags & !consts::SHADE_FLAGS_ACCEPTED_MASK) != 0 {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "Invalid flags in mask",
                ));
            }
            flags &= consts::SHADE_FLAGS_MASK;
        } else {
            flags = 0;
        }

        if flags & 0x80 == 0 {
            input.set_byte_order(ByteOrder::LittleEndian)
        } else {
            input.set_byte_order(ByteOrder::BigEndian)
        }

        let compound = NbtCompound::deserialize_copy(input)?;

        Ok(Self {
            magic,
            version,
            flags,
            compound,
        })
    }
}
