use crate::registry::{ResourceLocation, RegistryObject};
use crate::core::types::Type;
use crate::core::text::TextComponent;

use crate::core::moves::Move;
use std::collections::{BTreeMap, BTreeSet, LinkedList};
use crate::core::ability::Ability;
use crate::core::CachingFn;


pub struct Form{
    pub type1: RegistryObject<'static,Type>,
    pub type2: Optional<RegistryObject<'static,Type>>,
    pub ability: RegistryObject<'static,Ability>,
    pub ability2: Optional<RegistryObject<'static,Ability>>,
    pub hidden: Optional<RegistryObject<'static,Ability>>,
    pub base_stats: [u16;5],
    pub event_bus: Box<dyn EventBus + 'static>
}


pub struct Species{
    loc: ResourceLocation,
    name: TextComponent,
    base_form: std::string::String,
    forms: CachingFn<K,V>,
    base_hp: u16,
    machine_moves: BTreeSet<ResourceLocation>,
    event_bus: Box<dyn EventBus + 'static>
}

