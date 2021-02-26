use fused_lock::FusedRwLock;
use rlua::{prelude::*, Context, Value};

use std::{
    collections::HashMap,
    ops::{Deref, Index},
};

use crate::resource::ResourceLocation;

pub trait RegistryEntry {
    fn registry_name(&self) -> &ResourceLocation;
}

pub struct Registry<E> {
    underlying: FusedRwLock<HashMap<ResourceLocation, E>>,
}

impl<E> Default for Registry<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Registry<E> {
    pub fn new() -> Self {
        Self {
            underlying: FusedRwLock::new(HashMap::new()),
        }
    }

    pub fn lock(&self) {
        self.underlying.lock()
    }

    pub fn get(&self, name: &ResourceLocation) -> Option<&E> {
        self.underlying.try_read().map(|e| e.get(name)).flatten()
    }

    pub fn iter(&self) -> Iter<E> {
        Iter(self.underlying.try_read().map(HashMap::values))
    }
}

impl<E> Index<&ResourceLocation> for Registry<E> {
    type Output = E;
    fn index(&self, k: &ResourceLocation) -> &E {
        &self.underlying.read()[k]
    }
}

pub struct Iter<'a, E>(Option<std::collections::hash_map::Values<'a, ResourceLocation, E>>);

impl<'a, E> Iterator for Iter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<&'a E> {
        self.0.as_mut().map(Iterator::next).flatten()
    }
}

impl<'a, E> IntoIterator for &'a Registry<E> {
    type Item = &'a E;
    type IntoIter = Iter<'a, E>;
    fn into_iter(self) -> Iter<'a, E> {
        self.iter()
    }
}

impl<E: RegistryEntry> Registry<E> {
    pub fn register(&self, val: E) -> Option<E> {
        if self.underlying.is_locked() {
            return Some(val);
        }
        if let Some(mut guard) = self.underlying.try_write() {
            guard.insert(val.registry_name().clone(), val)
        } else {
            Some(val)
        }
    }

    pub fn get_object(&self, key: ResourceLocation) -> RegistryObject<E> {
        RegistryObject {
            registry: self,
            key,
        }
    }
}

pub struct RegistryObject<'a, E> {
    registry: &'a Registry<E>,
    key: ResourceLocation,
}

impl<'a, E: RegistryEntry> RegistryObject<'a, E> {
    pub fn get(&self) -> Option<&E> {
        self.registry.get(&self.key)
    }
}

impl<'a, E: RegistryEntry> Deref for RegistryObject<'a, E> {
    type Target = E;
    fn deref(&self) -> &E {
        &self.registry[&self.key]
    }
}

impl<E: RegistryEntry + for<'lua> FromLua<'lua>> Registry<E> {
    pub fn load_from_chunk<'lua>(
        &self,
        eval: rlua::Function<'lua>,
        _ctx: Context<'lua>,
    ) -> rlua::Result<bool> {
        let out: Value<'lua> = eval.call(())?;
        if self.underlying.is_locked() {
            return Ok(false);
        }
        if let Value::Table(tbl) = out {
            for v in 1..=tbl.raw_len() {
                let e = tbl.get(v)?;
                if self.register(e).is_some() {
                    return Ok(false);
                }
            }
            Ok(true)
        } else {
            Err(rlua::Error::FromLuaConversionError {
                from: "Value",
                to: "Registry",
                message: Some("Registry files must return a table".to_string()),
            })
        }
    }
}
