pub use crate::core::Side::{Client, Server};


pub mod event;
pub mod text;

pub mod ability;
pub mod moves;
pub mod types;

pub mod events;

pub enum Side{
    Server,
    Client
}


pub fn get_side() -> Side{
    if cfg!(feature="client"){
        Client
    }else{
        Server
    }
}

