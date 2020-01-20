#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::convert::{TryFrom, TryInto};
use std::ops::{Index, Deref};
use std::cell::Cell;
use std::sync::RwLock;

#[derive(Clone,PartialEq,PartialOrd,Eq,Ord,Hash)]
pub struct ResourceLocation{
    domain: std::string::String,
    path: std::string::String
}


lazy_static!{
    static ref DOMAIN_PATTERN: regex::Regex = {
        regex::Regex::new("[a-z_][a-z0-9_]*").unwrap()
    };
}
lazy_static!{
    static ref PATH_PATTERN: regex::Regex = {
        regex::Regex::new(r"[a-z_][a-z0-9_]*(\\[a-z_][a-z0-9_]*)*").unwrap()
    };
}

lazy_static!{
    static ref PATTERN: regex::Regex = {
        regex::Regex::new(r"[a-z_][a-z0-9_]*:[a-z_][a-z0-9_]*(\\[a-z_][a-z0-9_]*)*").unwrap()
    };
}

impl ResourceLocation{
    pub const fn new(domain: std::string::String,path: std::string::String) -> Result<ResourceLocation,std::string::String>{
        if !DOMAIN_PATTERN.is_match(&domain) || !PATH_PATTERN.is_match(&path){
            Err(r"Resource Locations must match: [a-z_][a-z0-9_]*:[a-z_][a-z0-9_]*(\\[a-z_][a-z0-9_]*)*".to_string())
        }else{
            Ok(ResourceLocation{domain,path})
        }
    }
}

impl<S: AsRef<str>> TryFrom<&S> for ResourceLocation{
    type Error = std::string::String;

    fn try_from(value: &S) -> Result<Self, Self::Error> {
        if !PATTERN.is_match(value){
            Err(r"Resource Locations must match: [a-z_][a-z0-9_]*:[a-z_][a-z0-9_]*(\\[a-z_][a-z0-9_]*)*".to_string())
        }else{
            let mut split = value.split(':');
            let domain = split.next().unwrap().to_string();
            let path = split.next().unwrap().to_string();
            Ok(ResourceLocation{domain,path})
        }
    }
}

impl ToString for ResourceLocation{
    fn to_string(&self) -> String {
        let mut ret = self.domain.clone();
        ret.push(':');
        ret.push_str(&self.path);
        ret
    }
}


pub trait RegistryEntry{
    fn name(&self) -> &ResourceLocation;
}

pub struct Registry<E: RegistryEntry>{
    underlying: std::collections::BTreeMap<ResourceLocation,E>
}

impl<E: RegistryEntry> Registry<E>{
    pub fn new() -> Registry<E>{
        Self{underlying: std::collections::BTreeMap::new()}
    }
    pub fn register(&mut self,val: E) -> Result<&mut E,std::String::String>{
        let name = val.name().clone();
        if self.underlying.contains_key(&name){
            Err("Object with this name already exists".to_string())
        }else{
            self.underlying.insert(name.clone(),val);
            Ok(self.underlying.get_mut(&name).unwrap())
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&E>{
        self.underlying.iter().map(|(_,o)|o)
    }
}



impl<E: RegistryEntry,Q: TryInto<ResourceLocation>> Index<&Q> for Registry<E>{
    type Output = E;

    fn index(&self, index: &Q) -> &Self::Output {
        &self.underlying[&index.try_into().unwrap()]
    }
}

pub struct RegistryObject<'a, E: RegistryEntry>{
    registry: &'a Registry<E>,
    name: ResourceLocation,
    obj: RwLock<Option<&'a E>>
}

impl<'a, E: RegistryEntry> RegistryObject<'a,E>{
    pub fn new(registry: &'a Registry<E>,name: ResourceLocation) -> Self{
        Self{registry,name,obj: RwLock::new(None)}
    }
}

impl<'a,E: RegistryEntry> Deref for RegistryObject<'a,E>{
    type Target = E;

    fn deref(&self) -> &'a Self::Target {
        if let Some(o) = *self.obj.read()? {
            o
        }else{
            *self.obj.write()? = Some(&self.registry[&self.name])
            *self.obj.read()?.unwrap()
        }
    }
}

