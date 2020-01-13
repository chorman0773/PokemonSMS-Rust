#![feature(try_trait)]

pub mod time;
pub mod version;
pub mod crypto;
pub mod shade;
pub mod uuid;
pub mod io;
pub mod registry;

pub mod core;

#[cfg(feature="pkmcom")]
pub mod pkmcom;

pub mod random;