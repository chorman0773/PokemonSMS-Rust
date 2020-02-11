use crate::core::text::TextComponent;
use crate::registry::{ResourceLocation, RegistryEntry};
use crate::core::event::EventHandler;

pub enum TypeModifier{
    WEAKNESS,
    RESISTANCE,
    IMMUNITY
}

pub struct Type{
    name: TextComponent,
    loc: ResourceLocation,
    icon: std::string::String,
    type_modifiers: std::collections::HashMap<ResourceLocation,TypeModifier>,
    handler: Box<dyn EventHandler>
}

impl RegistryEntry for Type{
    fn name(&self) -> &ResourceLocation {
        &self.loc
    }
}

impl Type{
    pub fn new<E: EventHandler + 'static>(name: TextComponent,loc: ResourceLocation,icon: std::string::String,type_modifiers: std::collections::HashMap<ResourceLocation,TypeModifier>, handler: E) -> Self{
        Self{
            name,
            loc,
            icon,
            type_modifiers,
            handler: Box::new(handler)
        }
    }

    pub fn get_unlocalized_name(&self) -> &TextComponent{
        &self.name
    }

    pub fn get_icon(&self) -> &std::string::String{
        &self.icon
    }

    pub fn is_immune_to(&self,other: &Type) -> bool{
        if let Some(o) = self.type_modifiers.get(other.name()){
            o == &TypeModifier::IMMUNITY
        }else {
            false
        }
    }

    pub fn get_modifier(&self,other: &Type) -> f32{
        match self.type_modifiers.get(other.name()){
            Some(&TypeModifier::WEAKNESS) => 2f32,
            Some(&TypeModifier::RESISTANCE) => 0.5f32,
            _ => 1
        }
    }

    pub fn get_event_handler(&self) -> &dyn EventHandler{
        *self.handler
    }
}

