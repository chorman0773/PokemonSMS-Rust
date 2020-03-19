pub use crate::core::Side::{Client, Server};
use std::sync::{RwLock, Mutex};
use std::collections::BTreeMap;


pub mod event;
pub mod text;

pub mod ability;
pub mod moves;
pub mod types;

pub mod events;
pub mod species;

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

pub struct CachingFn<K,V>{
    cache: Mutex<BTreeMap<K,V>>,
    actual: Box<dyn Fn(&K)->V>
}

impl<K: PartialOrd,V> CachingFn<K,V>{
    pub fn new<F: Fn(&K)->V+'static>(f: F) -> Self{
        Self{cache: Mutex::default(),actual: Box::new(f)}
    }
}

impl<K: PartialOrd,V: Clone> Fn(&K)->V for CachingFn<K,V>{
    fn call(&self, (key): (K)) -> Self::Output {
        if let Ok(mut cache) = self.cache.lock(){
            if let Some(v) = cache.get(key){
                return v.clone()
            }else{
                cache.insert(key,self.actual(&key));
                cache.get(key).unwrap().clone()
            }
        }else{
            self.actual(key)
        }
    }
}
