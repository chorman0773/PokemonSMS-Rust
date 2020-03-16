use std::cell::Cell;
use rlua::{UserData, UserDataMethods, MetaMethod};
use std::any::Any;
use std::hash::{Hash, Hasher};
use crate::core::{get_side, Side};

pub trait EventKey : Copy + Eq + Hash {}

#[derive(Copy,Clone)]
pub struct EventKeyWrapper<'a>(pub &'a dyn EventKey);

impl<'a> Hash for EventKeyWrapper<'a>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.type_id().hash(state);
        self.0.hash();
    }
}

impl<'a> PartialEq for EventKeyWrapper<'a>{
    fn eq(&self, other: &Self) -> bool {
        if self.0.type_id() == other.0.type_id(){
            self.0 == other.0
        }else{
            false
        }
    }
}

impl<'a> Eq for EventKeyWrapper<'a>{}

impl<'a,'b: 'a> From<EventKeyWrapper<'a>> for EventKeyWrapper<'b>{
    fn from(val: EventKeyWrapper<'a>) -> Self {
        Self(val.0)
    }
}

impl<'a,'b: 'a> From<&EventKeyWrapper<'a>> for EventKeyWrapper<'b>{
    fn from(val: &EventKeyWrapper<'a>) -> Self {
        Self(val.0)
    }
}

impl<'a> UserData for EventKeyWrapper<'a>{
    fn add_methods<'lua,M: UserDataMethods<'lua, Self>>(methods: &mut M){
        methods.add_meta_function(MetaMethod::Eq,|ctx,args: (EventKeyWrapper<'a>,EventKeyWrapper<'a>)|{
            let (a,b) = args;
            if a.0.type_id() != b.0.type_id(){
                return false
            }else{
                return a.0 == b.0
            }
        })
    }
}

pub trait Event {
    type Key: EventKey;
    type Params : rlua::ToLuaMulti;
    fn get_key(&self) -> &'static Self::Key;
    fn get_params(&self) -> &Self::Params;

}

pub trait EventHandler{
    fn handle(&mut self,event: &dyn Event);
}

impl<E: Event,EH: EventHandler> FnMut(&E)->() for EH{
    fn call_mut(&mut self, args: (&E)) -> Self::Output {
        let (event) = args;
        self.handle(event)
    }
}

pub struct LuaHandler<'lua>(pub rlua::Function<'lua>,pub rlua::Function<'lua>);
impl<'lua> UserData for LuaHandler<'lua>{}

pub struct LuaEventBus<'a>{
    ctx: &'a rlua::Lua,
    events: Option<rlua::RegistryKey>
}

impl Drop for LuaEventBus{
    fn drop(&mut self) {
        if let Some(rkey) = self.events.as_mut_ref(){
            self.ctx.context(|ctx|{ctx.remove_registry_value(core::mem::take(rkey))});
        }
    }
}

impl<'a> LuaEventBus<'a>{
    pub fn new(ctx: &'a rlua::Lua) -> Self{
        Self{ctx,events: None}
    }
    pub fn init(&mut self) -> rlua::Result<&rlua::RegistryKey>{
        if let None = self.events{
            self.ctx.context(|ctx|{
                let mut tab = self.ctx.create_table()?;
                self.events = Some(ctx.create_registry_value(tab)?);
            })?;
        }
        Ok(self.events.as_ref().unwrap())
    }
    fn register<'lua>(ctx: rlua::Context<'lua>,bus: &mut Self,vals: (EventKeyWrapper<'static>, rlua::Function<'lua>, rlua::Function<'lua>)) -> rlua::Result<&Self>{
        let (key,pred,handler) = vals;
        let predreg = ctx.create_registry_value(pred)?;
        let handlerreg = ctx.create_registry_value(handler)?;
        bus.ctx.context(|ctx| {
            let wrapped = LuaHandler(ctx.registry_value(&predreg)?, ctx.registry_value(&handlerreg)?);
            ctx.remove_registry_value(predreg)?;
            ctx.remove_registry_value(handlerreg)?;
            let rkey = bus.init()?;
            let mut tab: rlua::Table = bus.ctx.registry_value(rkey)?;
            tab.raw_set(key, wrapped)?;
        })?;
        Ok(bus)
    }
    fn register_noop<'lua>(ctx: rlua::Context<'lua>,bus: &mut Self,vals: (EventKeyWrapper<'static>, lua::Function<'lua>, lua::Function<'lua>)) -> rlua::Result<&Self>{
        Ok(bus)
    }
}

impl UserData for LuaEventBus{
    fn add_methods<'lua,M: UserDataMethods<'lua, Self>>(methods: &mut M){
        methods.add_function_mut(r"register",Self::register);
        if get_side() == Side::Client {
            methods.add_function_mut(r"registerClient", Self::register);
            methods.add_function_mut(r"registerServer",Self::register_noop);
        }else{
            methods.add_function_mut(r"registerServer", Self::register);
            methods.add_function_mut(r"registerClient",Self::register_noop);
        }
    }
}

impl EventHandler for LuaEventBus{
    fn handle(&mut self, event: &dyn Event) {
        if let Some(rkey) = self.events.as_ref(){
            self.ctx.context(|ctx|{
                let tab: rlua::Table = ctx.registry_value(rkey)?;
                let key = EventKeyWrapper(event.get_key());
                let LuaHandler(pred,handler) = tab.get(key)?;
                let result = pred.call::<_,bool>(event.get_params())?;
                if result{
                    handler.call(get_params())?;
                }
                Ok(())
            });
        }
    }
}

pub struct NullEventBus;

impl EventHandler for NullEventBus{
    fn handle(&mut self, event: &dyn Event) {}
}




