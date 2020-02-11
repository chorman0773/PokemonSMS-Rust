use crate::registry::{ResourceLocation, RegistryEntry};
use crate::core::text::*;
use crate::core::event::{EventHandler, LuaEventBus,NullEventBus};
use rlua::{FromLua, Context, Value, Error, Table};
use std::convert::TryFrom;

#[macro_use]
extern crate lazy_static;

pub struct Ability{
    loc: ResourceLocation,
    name: TextComponent,
    desc: TextComponent,
    bus: Box<dyn EventHandler>
}

impl Ability{
    pub fn new<EH: EventHandler + 'static>(loc: ResourceLocation,name: TextComponent,desc: TextComponent,bus: EH) -> Ability{
        Ability{loc,name,desc,bus: Box::new(bus)}
    }
    pub fn get_name(&self) -> &TextComponent{
        &self.name
    }
    pub fn get_description(&self) -> &TextComponent{
        &self.desc
    }
    pub fn get_event_bus(&self) -> &dyn EventHandler{
        self.bus.as_ref()
    }
    lazy_static!{
        pub static ref NULL_ABILITY: Ability = {
            Ability::new(ResourceLocation::new("system","abilities/null")?,Text("null".to_string(),None),Empty,NullEventBus)
        };
    }
}

impl RegistryEntry for Ability{
    fn name(&self) -> &ResourceLocation {
        &self.loc
    }
}



