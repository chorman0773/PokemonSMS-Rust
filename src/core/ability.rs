use crate::registry::{ResourceLocation, RegistryEntry};
use crate::core::text::*;
use crate::core::event::{EventHandler, LuaEventBus,NullEventBus};
use rlua::{FromLua, Context, Value, Error, Table};
use std::convert::{TryFrom, TryInto};


pub struct Ability{
    loc: ResourceLocation,
    name: TextComponent,
    desc: TextComponent,
    bus: Box<dyn EventHandler + 'static>
}

impl Ability{
    pub fn new<R: TryInto<ResourceLocation>,EH: EventHandler + 'static>(loc: R,name: TextComponent,desc: TextComponent,bus: EH) -> Result<Ability,<R as TryInfo<ResourceLocation>>::Err>{
        Ok(Ability{loc: loc.try_into()?,name,desc,bus: Box::new(bus)})
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

}

lazy_static!{
    pub static ref null: Ability = {
        Ability::new("system:abilities/null",Text("null".to_string(),None),Empty,NullEventBus).unwrap()
    };
    pub static ref REGISTRY: Registry<Ability> = {
        let reg = Registry::new();
        reg.register(*null);
        reg
    };
}


impl RegistryEntry for Ability{
    fn name(&self) -> &ResourceLocation {
        &self.loc
    }
}



