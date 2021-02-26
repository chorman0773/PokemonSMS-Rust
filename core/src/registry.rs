use lazy_static::__Deref;
use rlua::{prelude::*, Context, Value};

use std::{
    cell::UnsafeCell,
    collections::HashMap,
    ops::Index,
    sync::{
        atomic::{AtomicBool, Ordering},
        RwLock,
    },
};

use crate::resource::ResourceLocation;

pub trait RegistryEntry {
    fn registry_name(&self) -> &ResourceLocation;
}

pub struct Registry<E> {
    lock: RwLock<()>,
    underlying: UnsafeCell<HashMap<ResourceLocation, E>>,
    locked: AtomicBool,
}

impl<E> Default for Registry<E> {
    fn default() -> Self {
        Self {
            lock: Default::default(),
            underlying: Default::default(),
            locked: Default::default(),
        }
    }
}

impl<E> Registry<E> {
    pub fn new() -> Self {
        Default::default()
    }

    fn get_if_locked(&self) -> Option<&HashMap<ResourceLocation, E>> {
        if self.locked.load(Ordering::Acquire) {
            // SAFETY:
            // Because self.locked is set, the lock is never mutably borrowed.
            Some(unsafe { &*(self.underlying.get() as *const HashMap<_, _>) })
        } else {
            None
        }
    }

    pub fn lock(&self) {
        let _guard = self.lock.read().unwrap();
        self.locked.store(true, Ordering::Release);
    }

    pub fn get(&self, name: &ResourceLocation) -> Option<&E> {
        self.get_if_locked().map(|m| m.get(name)).flatten()
    }

    pub fn iter(&self) -> Iter<E> {
        if self.locked.load(Ordering::Acquire) {
            Iter(self.get_if_locked().map(HashMap::values))
        } else {
            Iter(None)
        }
    }
}

impl<E> Index<&ResourceLocation> for Registry<E> {
    type Output = E;
    fn index(&self, k: &ResourceLocation) -> &E {
        if self.locked.load(Ordering::Acquire) {
            self.get(k).unwrap()
        } else {
            panic!(
                "Cannot access a Registry that's not locked. Use Registry::create_object instead"
            )
        }
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
        if self.locked.load(Ordering::Relaxed) {
            Some(val)
        } else {
            let _guard = self.lock.write().unwrap();
            if self.locked.load(Ordering::Relaxed) {
                // SAFETY:
                // Because self.lock is locked exclusively, it's safe to mutably borrow the interior.
                unsafe { &mut *self.underlying.get() }.insert(val.registry_name().clone(), val)
            } else {
                Some(val)
            }
        }
    }
}

impl<E: RegistryEntry + for<'lua> FromLua<'lua>> Registry<E> {
    pub fn load_from_chunk<'lua>(
        &self,
        eval: rlua::Function<'lua>,
        _ctx: Context<'lua>,
    ) -> rlua::Result<()> {
        let out: Value<'lua> = eval.call(())?;

        if let Value::Table(tbl) = out {
            for v in 1..=tbl.raw_len() {
                let e = tbl.get(v)?;
                self.register(e);
            }
            Ok(())
        } else {
            Err(rlua::Error::FromLuaConversionError {
                from: "Value",
                to: "Registry",
                message: Some("Registry files must return a table".to_string()),
            })
        }
    }
}
