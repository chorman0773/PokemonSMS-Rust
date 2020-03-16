#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::convert::{TryFrom, TryInto};
use std::ops::{Index, Deref, DerefMut};
use std::cell::Cell;
use std::sync::{RwLock, LockResult, RwLockReadGuard, RwLockWriteGuard, PoisonError};

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
    pub fn new(domain: std::string::String,path: std::string::String) -> Result<ResourceLocation,std::string::String>{
        if !DOMAIN_PATTERN.is_match(&domain) || !PATH_PATTERN.is_match(&path){
            Err(r"Resource Locations must match: [a-z_][a-z0-9_]*:[a-z_][a-z0-9_]*(\\[a-z_][a-z0-9_]*)*".to_string())
        }else{
            Ok(ResourceLocation{domain,path})
        }
    }

	pub fn create<S: AsRef<str>>(value: &S) -> Self{
		value.try_into().unwrap()
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

pub struct Registry<E>{
    underlying: RwLock<std::collections::BTreeMap<ResourceLocation,E>>,
    locked: RwLock<bool>
}

unsafe impl<E: RegistryEntry + Sync> Sync for Registry<E>{}

impl<E> !Send for Registry<E>{}

impl<E: RegistryEntry> Registry<E>{
    pub fn new() -> Registry<E>{
        Self{underlying: RwLock::new(std::collections::BTreeMap::new()),..Default::default()}
    }
    pub fn register(&self,val: E) -> Result<(),RegistryError>{
        let name = val.name().clone();
        let lock = self.underlying.write()?;
        if *self.locked.read()?{
            Err(RegistryError::Locked)
        }else if lock.contains_key(&name){
            Err(RegistryError::AlreadyExists)
        }else{
            lock.insert(name.clone(),val);
            Ok(())
        }
    }

    pub fn lock(&self) -> Result<(),RegistryError>{
        let lock = self.underlying.read()?;
		if *self.locked.read()?{
			Err(RegistryError::Locked)
		}else{
			*self.locked.write()? = true;
			Ok(())
		}
    }

    pub fn iter(&self) -> Result<impl Iterator<Item=&E>,RegistryError>{
        let lock = self.underlying.read()?;
        Ok(ItemIter{map_lock: lock,iter: lock.iter()})
    }

    pub fn get_delayed<Q: TryInto<ResourceLocation>>(&self,key: &Q) -> RegistryObject<E>{
        RegistryObject::new(self,key.try_into()?)
    }
}

struct ItemIter<'a,E: RegistryEntry>{
    map_lock: RwLockReadGuard<'a,std::collections::BTreeMap<ResourceLocation,E>>,
    iter: std::collections::btree_map::Iter<'a,ResourceLocation,E>
}

impl<'a,E: RegistryEntry> Iterator for ItemIter<'a,E>{
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_,v)|v)
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

pub enum RegistryError{
    Poisoned,
    Locked,
    AlreadyExists
}

impl<T> From<PoisonError<T>> for RegistryError{
    fn from(_: PoisonError<T>) -> Self {
       RegistryError::Poisoned
    }
}


