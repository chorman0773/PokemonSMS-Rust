pub use crate::core::Side::{Client, Server};

pub mod event;

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

