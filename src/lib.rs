#![feature(try_trait)]
#![feature(const_generics)]
#![feature(never_type)]
#![feature(box_syntax)]
#![feature(core_intrinsics)]
#![feature(impl_trait_in_bindings)]

#[macro_use]
extern crate lazy_static;


pub(crate) mod helper;

pub mod time;
pub mod version;
pub mod shade;
pub mod uuid;
pub mod io;
pub mod registry;

pub mod core;

#[cfg(feature="pkmcom")]
pub mod pkmcom;

pub mod random;