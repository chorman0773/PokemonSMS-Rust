#![deny(warnings, missing_docs)]

//!
//! Package for performing IO in the format specified by LCS 4,
//! as well as serialization according to the ShadeNBT specification
//!

#[macro_use]
extern crate fake_enum;

pub mod data;

pub mod uuid;

pub mod version;

#[cfg(feature = "nbt")]
pub mod nbt;

#[cfg(feature = "shade")]
pub mod shade;
