use crate::core::text::TextComponent;
use crate::core::event::EventHandler;
use crate::core::types::Type;
use crate::registry::RegistryObject;

pub enum Category{
    Physical,
    Special,
    Status
}

pub struct Move{
    name: TextComponent,
    desc: TextComponent,
    handler: Box<dyn EventHandler>,
    base_pp: u8,
    power: Option<u32>,
    accuracy: Option<f32>,
    category: Category,
    traits: std::collections::HashSet<std::string::String>,
    move_type: RegistryObject<'static,Type>
}

