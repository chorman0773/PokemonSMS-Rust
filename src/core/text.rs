

use crate::registry::ResourceLocation;
use std::convert::{TryFrom, TryInto};
use json::JsonValue;
use std::time::Duration;

#[derive(Copy,Clone)]
pub enum Color{
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Grey,
    DarkGrey,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White
}

impl<S: AsRef<str>> TryFrom<&S> for Color{
    type Error = std::string::String;

    fn try_from(value: &S) -> Result<Self, Self::Error> {
        lazy_static!{
            static ref MAP: std::collection::HashMap<&str,Color> = {
                let map = std::collection::HashMap::<&str,Color>::new();
                map.insert("black",Color::Black);
                map.insert("dark_blue",Color::DarkBlue);
                map.insert("dark_green",Color::DarkGreen);
                map.insert("dark_aqua",Color::DarkAqua);
                map.insert("dark_red",Color::DarkRed);
                map.insert("dark_purple",Color::DarkPurple);
                map.insert("gold",Color::Gold);
                map.insert("grey",Color::Grey);
                map.insert("gray",Color::Grey);
                map.insert("dark_grey",Color::DarkGrey);
                map.insert("dark_gray",Color::DarkGrey);
                map.insert("blue",Color::Blue);
                map.insert("green",Color::Green);
                map.insert("aqua",Color::Aqua);
                map.insert("red",Color::Red);
                map.insert("light_purple",Color::LightPurple);
                map.insert("yellow",Color::Yellow);
                map.insert("white",Color::White);
                map
            };
        }
        if MAP.contains(value.as_ref()){
            Ok(MAP.get(value.as_ref())?)
        }else{
            Err(format!("No such color {}.",value))
        }
    }
}

#[derive(Copy,Clone)]
pub struct Style{
    pub color: Color,
    pub underscore: Option<bool>,
    pub strikethrough: Option<bool>,
    pub bold: Option<bool>,
    pub italics: Option<bool>
}

impl TryFrom<&JsonValue> for Style{
    type Error = std::string::String;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        let color: Color;
        let underscore: Option<bool>;
        let strikethrough: Option<bool>;
        let bold: Option<bool>;
        let italics: Option<bool>;
        if let JsonValue::Object(o) = value{
            if let Some(JsonValue::String(c)) = o.get("color"){
                color = Color::try_from(c)?;
            }else{
                return Err("Style must have a color field")
            }
            if let Some(&JsonValue::Boolean(val)) = o.get("underline"){
                underscore = Some(val)
            }else{
                underscore = None
            }
            if let Some(&JsonValue::Boolean(val)) = o.get("strikethrough"){
                strikethrough = Some(val)
            }else{
                strikethrough = None
            }
            if let Some(&JsonValue::Boolean(val)) = o.get("bold"){
                bold = Some(val)
            }else{
                bold = None
            }
            if let Some(&JsonValue::Boolean(val)) = o.get("italics"){
                italics = Some(val)
            }else{
                italics = None
            }
            Ok(Style{color,underscore,strikethrough,bold,italics})
        }else{
            Err("Expected an Object".to_string())
        }
    }
}

impl<'lua> TryFrom<&rlua::Value<'lua>> for Style{
    type Error = rlua::Error;

    fn try_from(value: &rlua::Value<'lua>) -> Result<Self, Self::Error> {
        let color: Color;
        let underscore: Option<bool>;
        let strikethrough: Option<bool>;
        let bold: Option<bool>;
        let italics: Option<bool>;
        if let LuaValue::Table(tab) = value{
            if let LuaValue::String(s) = tab.get("color")?{
                color = Color::try_from(&s)?;
            }else{
                return Err(rlua::Error::RuntimeError("Style must have a color field".to_string()))
            }
            if let LuaValue::Boolean(b) = tab.get("underline")?{
                underscore = Some(b)
            }else{
                underscore = None
            }
            if let LuaValue::Boolean(b) = tab.get("strikethrough")?{
                strikethrough = Some(b)
            }else{
                strikethrough = None
            }
            if let LuaValue::Boolean(b) = tab.get("bold")?{
                bold = Some(b)
            }else{
                bold = None
            }
            if let LuaValue::Boolean(b) = tab.get("italics")?{
                italics = Some(b)
            }else{
                italics = None
            }
            Ok(Style{color,underscore,strikethrough,bold,italics})
        }else{
            Err(rlua::Error::RuntimeError("Expected a table".to_string()))
        }
    }
}

impl<'lua> FromLua<'lua> for Style{
    fn from_lua(lua_value: rlua::Value<'lua>, lua: Context<'lua>) -> Result<Self, Error> {
        Style::try_from(&lua_value)
    }
}

#[derive(Copy,Clone)]
pub enum TextCommand{
    LineBreak(u8),
    ScrollText(u8),
    DelayScrollText(std::time::Duration),
    Format(Style)
}

impl TryFrom<&JsonValue> for TextCommand{
    type Error = std::string::String;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        if let JsonValue::Object(o) = value{
            if let Some(JsonValue::String(s)) = o.get("type"){
                let val = if let Some(val) = o.get("value"){
                    val.as_f64().unwrap_or(1f64)
                }else{
                    1f64
                };
                if s=="line"{
                    if val > 5f64 || val.is_nan(){
                        return Err(format!("Cannot break more than 5 lines (Attempting to break {} lines)",val))
                    }
                    Ok(TextCommand::LineBreak(val as u8))
                }else if s=="scroll"{
                    if val > 5f64 || val.is_nan(){
                        return Err(format!("Cannot scroll more than 5 lines (Attempting to scroll {} lines)",val))
                    }
                    Ok(TextCommand::ScrollText(val as u8))
                }else if s=="delay"{
                    Ok(TextCommand::DelayScrollText(Duration::from_secs_f64(val)))
                }else if s=="format"{
                    if let Some(val) = o.get("style"){
                        Ok(TextCommand::Format(Style::try_from(val)?))
                    }else{
                        Err(format!("Format Command needs a style"))
                    }
                }else{
                    Err(format!("Unknown command {}",s))
                }
            }else{
                Err("Expected a type key".to_string())
            }
        }else{
            Err("Expected an Object".to_string())
        }
    }
}

impl<'lua> TryFrom<&rlua::Value<'lua>> for TextCommand{
    type Error = rlua::Error;

    fn try_from(value: &Value<'lua>) -> Result<Self, Self::Error> {
        if let Value::Table(tab) = value{
            if let Value::String(s) = tab.get("type")?{
                let val : f64 = if let Value::Number(n) = tab.get("value")?{
                    n
                }else{
                    1.0
                };
                if s=="line"{
                    if val > 5f64 || val.is_nan(){
                        return Err(Error::RuntimeError(format!("Cannot break more than 5 lines (Attempting to break {} lines)",val)))
                    }
                    Ok(TextCommand::LineBreak(val as u8))
                }else if s=="scroll"{
                    if val > 5f64 || val.is_nan(){
                        return Err(Error::RuntimeError(format!("Cannot scroll more than 5 lines (Attempting to scroll {} lines)",val)))
                    }
                    Ok(TextCommand::ScrollText(val as u8))
                }else if s=="delay"{
                    Ok(TextCommand::DelayScrollText(Duration::from_secs_f64(val)))
                }else if s=="format"{
                    if let Value::Table(tab) = tab.get("style")?{
                        Ok(TextCommand::Format(Style::try_from(&Value::Table(tab))?))
                    }else{
                        Err(Error::RuntimeError("Format command needs a style".to_string()))
                    }
                }else{
                    Err(Error::RuntimeError(format!("Invalid type {}",s)))
                }
            }else{
                Err(Error::RuntimeError("Expected a type key".to_string()))
            }
        }else{
            Err(Error::RuntimeError("Expected a table".to_string()))
        }
    }
}

impl<'lua> FromLua<'lua> for TextCommand{
    fn from_lua(lua_value: Value<'lua>, lua: Context<'lua>) -> Result<Self, Error> {
        TextCommand::try_from(&lua_value)
    }
}

#[derive(Clone)]
pub enum TextComponent{
    Text(std::string::String,Option<Style>,Option<Box<TextComponent>>),
    Argument(Option<usize>,Option<Box<TextComponent>>),
    Command(TextCommand,Option<Box<TextComponent>>),
    Translation(std::string::String,Option<Box<TextComponent>>),
    Icon(ResourceLocation,Option<Box<TextComponent>>),
    Group(std::vec::Vec<TextComponent>),
    Empty
}

impl Default for TextComponent{
    fn default() -> Self {
        Empty
    }
}

impl TextComponent{
    pub fn translate<S: ToString>(v: &S) -> Self{
        TextComponent::Translation(v.to_string(),None)
    }

    pub fn text<S: ToString>(v: &S) -> Self{
        Text(v.to_string(),None,None)
    }
    pub fn style_text<S: ToString>(v: &S,style: Style) -> Self{
        Text(v.to_string(),Some(style),None)
    }
    pub fn icon<S: TryInto<ResourceLocation>>(key: S) -> Self{
        Icon(key.try_into().ok().unwrap(),None)
    }

    pub fn concatenate(self,other: Self) -> Self{
        match self{
            Empty => Group(vec![other]),
            Group(mut v)=>{
                v.push(other);
                Group(v)
            },
            Text(text,style,Some(next)) => Text(text,style,Some(box next.concatenate(other))),
            Text(text,style,None) => Text(text,style,Some(box other)),
            val => Group(vec![val, other])
        }
    }

    ///
    /// Replaces self with a value computed from f.
    /// If f panics, then self is left as an empty text component.
    /// The value initially contained in self is moved into the function call.
    ///
    pub fn replace<F: FnOnce(Self)->Self>(&mut self, f: F){
        let val = std::mem::take(self);
        *self = f(val);
    }

}

pub use TextComponent::*;
use rlua::prelude::LuaValue;
use rlua::{FromLua, Context, Value, Error};
use std::ops::Index;

impl TryFrom<&JsonValue> for TextComponent{
    type Error = std::string::String;

    fn try_from(value: &JsonValue) -> Result<Self, Self::Error> {
        match value{
            JsonValue::String(s) => Ok(Text(s.clone(),None,None)),
            JsonValue::Array(v) => {
                let mut ret: std::vec::Vec<TextComponent> = std::vec::Vec::new();
                for i in v{
                    ret.push(i.try_into()?);
                }
                Ok(Group(ret))
            },
            JsonValue::Object(o) => {
                if let Some(JsonValue::String(text)) = o.get("text"){
                    let style;
                    if let Some(o) = o.get("style"){
                        style = Some(Style::try_from(o)?);
                    }else{
                        style = None;
                    }
                    let extra = if let Some(o) = o.get("extra"){
                        Some(Box::new(TextComponent::try_from(o)?))
                    }else{
                        None
                    };
                    Text(s,style,extra)
                }else if let Some(JsonValue::Number(n)) = o.get("argument"){
                    let extra = if let Some(o) = o.get("extra"){
                        Some(Box::new(TextComponent::try_from(o)?))
                    }else{
                        None
                    };
                    Ok(Argument(Some(n.as_fixed_point_u64(0) as usize),extra))
                }else if let Some(&JsonValue::String(s)) = o.get("translate"){
                    let extra = if let Some(o) = o.get("extra"){
                        Some(Box::new(TextComponent::try_from(o)?))
                    }else{
                        None
                    };
                    Ok(Translation(s,extra))
                }else if let Some(JsonValue::String(s)) = o.get("icon"){
                    let extra = if let Some(o) = o.get("extra"){
                        Some(Box::new(TextComponent::try_from(o)?))
                    }else{
                        None
                    };
                    Ok(Icon(ResourceLocation::try_from(s)?,extra))
                }else if let Some(JsonValue::Array(arr)) = o.get("group"){
                    let mut ret: std::vec::Vec<TextComponent> = std::vec::Vec::new();
                    for i in arr{
                        ret.push(TextComponent::try_from(i)?);
                    }
                    Ok(Group(ret))
                }else{
                    Err("Invalid or malformed Text Structure".to_string())
                }
            },
            JsonValue::Null => Ok(Empty),
            _ => Err("Expected string, array, or object".to_string())
        }
    }
}

impl<'lua> TryFrom<&rlua::Value<'lua>> for TextComponent{
    type Error = rlua::Error;

    fn try_from(value: &Value<'lua>) -> Result<Self, Self::Error> {
        match value{
            Value::String(s) => Ok(Translation(s.to_str()?.to_string(),None)),
            Value::Table(tab) => {
                return if let Value::String(text) = tab.get("text")?{
                    let style;
                    if let Value::Table(tab) = tab.get("style")?{
                        style = Some(Style::try_from(&Value::Table(tab))?);
                    }else{
                        style = None
                    }
                    let extra = match tab.get("extra")?{
                        &LuaValue::Nil => None,
                        val => Some(Box::new(TextComponent::try_from(val)?))
                    };
                    Ok(Text(text.to_str()?.to_string(),style,extra))
                }else if let Value::String(key) = tab.get("translate")?{
                    let extra = match tab.get("extra")?{
                        &LuaValue::Nil => None,
                        val => Some(Box::new(TextComponent::try_from(val)?))
                    };
                    Ok(Translation(key.to_str()?.to_string(),extra))
                }else if let Value::String(res) = tab.get("icon")?{
                    let extra = match tab.get("extra")?{
                        &LuaValue::Nil => None,
                        val => Some(Box::new(TextComponent::try_from(val)?))
                    };
                    Ok(Icon(ResourceLocation::try_from(res.to_str()?)?,extra))
                }else if let Value::Integer(n) = tab.get("argument")?{
                    let extra = match tab.get("extra")?{
                        &LuaValue::Nil => None,
                        val => Some(Box::new(TextComponent::try_from(val)?))
                    };
                    Ok(Argument(if n < 0{
                      None
                    }else{
                        Some(n as usize)
                    },extra))
                }else{
                    Err(Error::RuntimeError("Invalid or malformed Text Structure".to_string()))
                }
            },
            Value::Nil => Ok(Empty),
            _ => Err(Error::RuntimeError("Expected string or table".to_string()))
        }
    }
}

impl<'lua> FromLua<'lua> for TextComponent{
    fn from_lua(lua_value: Value<'lua>, lua: Context<'lua>) -> Result<Self, Error> {
        TextComponent::try_from(&lua_value)
    }
}

#[derive(Default)]
pub struct I18N{
    tree: std::collections::HashMap<std::string::String,TextComponent>
}

impl I18N{
    pub fn translate_copy(&self,text: TextComponent) -> TextComponent{
        {
            match text{
                Text(s,style,Some(mut next)) => {
                    self.translate(next.as_mut());
                    Text(s,style,Some(next))
                },
                Translation(s,mut next) => {
                    let mut ret = self.tree.get(&s).map(|v|v.clone())
                        .unwrap_or_else(||Text(s,None,None));
                    if let Some(mut n) = next{
                        self.translate(n.as_mut());
                        ret = ret.concatenate(*n)
                    }
                    ret
                },
                Argument(n,Some(mut next)) =>{
                    self.translate(next.as_mut());
                    Argument(n,Some(next))
                },
                Command(cmd,Some(mut next)) => {
                    self.translate(next.as_mut());
                    Command(cmd,Some(next))
                },
                Icon(res,Some(mut next)) => {
                    self.translate(next.as_mut());
                    Icon(res,Some(next))
                },
                Group(mut g) =>{
                    for t in g.iter_mut(){
                        self.translate(t)
                    }
                    Group(g)
                },
                v => v
            }
        }
    }
    pub fn translate(&self,component: &mut TextComponent){
        component.replace(|text|self.translate_copy(text));
    }
}

impl TryFrom<&JsonValue> for I18N{
    type Error = std::string::String;

    fn try_from(v: &JsonValue) -> Result<Self,Self::Error> {
        let mut tree = Default::default();
        if let JsonValue::Object(o) = v{
            for (k,v) in o.iter(){
                tree.insert(k.to_string(),TextComponent::try_from(v)?)
            }
            Ok(Self{tree})
        }else{
            Err("An object is required".to_string())
        }
    }
}

pub struct TextArguments<'a>{
    arg_pos: usize,
    components: &'a [TextComponent]
}

impl<'a> Iterator for TextArguments<'a>{
    type Item = &'a TextComponent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.arg_pos >= self.components.len() {
            None
        }else{
            let ret = &self.components[self.arg_pos];
            self.arg_pos += 1;
            Some(ret)
        }
    }
}

impl<'a> Index<usize> for TextArguments<'a>{
    type Output = TextComponent;

    fn index(&self, index: usize) -> &Self::Output {
        components[index]
    }
}

pub trait TextDisplay{
    ///
    /// Pushes the current style onto the TextDisplay's Style Stack.
    /// This does not update the current style
    fn push_style(&mut self) -> &mut Self;
    ///
    /// Sets the current style to the given one.
    /// This overwrites the existing style, if its desired to return to the existing style,
    ///  self.push_style() may be called before this method.
    fn set_style(&mut self,style: Style) -> &mut Self;
    fn pop_style(&mut self) -> &mut Self;
    fn execute_command(&mut self,c: TextCommand) -> &mut Self;
    fn write_string(&mut self,s: &std::string::String) -> &mut Self;
    fn draw_icon(&mut self,r: &ResourceLocation) -> &mut Self;
    fn get_translation(&self) -> Option<&I18N>;

    fn draw_text(&mut self,text: &TextComponent,args: &mut TextArguments) -> &mut Self{
        match text{
            Empty => {},
            Group(v) => {
                self.push_style();
                for t in v.iter(){
                    self.draw_text(t,args)
                }
                self.pop_style();
            },
            Text(t,style,next) => {
                self.push_style();
                if let Some(s) = style{
                    self.set_style(*s);
                }
                self.write_string(t);
                if let Some(n) = next{
                    self.draw_text(n.as_ref(),args)
                }
                self.pop_style();
            },
            Translation(key,next)=>{
                self.push_style();
                if let Some(i) = self.get_translation(){
                    self.draw_text(i.tree.get(key).unwrap_or_else(||&Text(key.clone(),None,None)))
                }else{
                    self.draw_text(&Text(key.clone(),None,None))
                }
                if let Some(n) = next{
                    self.draw_text(&n,args)
                }
                self.pop_style()
            },
            Command(TextCommand::Format(style),next) => {
                self.set_style(*style);
                if let Some(n) = next{
                    self.draw_text(&n,args)
                }
            },
            Command(cmd,next) => {
                self.execute_command(*cmd);
                if let Some(n) = next{
                    self.draw_text(&n,args)
                }
            },
            Argument(Some(n),next) => {
                self.push_style();
                self.draw_text(&args[*n],args);
                if let Some(n) = next{
                    self.draw_text(&n,args)
                }
                self.pop_style();
            },
            Argument(None,next) =>{
                self.push_style();
                self.draw_text(args.next().unwrap(),args);
                if let Some(n) = next{
                    self.draw_text(&n,args)
                }
                self.pop_style();
            }
            Icon(key, next) => {
                self.draw_icon(key);
                if let Some(n) = next {
                    self.push_style();
                    self.draw_text(&n,args);
                    self.pop_style();
                }
            }
        }
        self
    }
}

pub struct NullDisplay;

impl TextDisplay for NullDisplay{
    fn push_style(&mut self) -> &mut Self {
        self
    }

    fn set_style(&mut self, style: Style) -> &mut Self {
        self
    }

    fn pop_style(&mut self) -> &mut Self {
        self
    }

    fn execute_command(&mut self, c: TextCommand) -> &mut Self {
        self
    }

    fn write_string(&mut self, s: &String) -> &mut Self {
        self
    }

    fn draw_icon(&mut self, r: &ResourceLocation) -> &mut Self {
        self
    }

    fn get_translation(&self) -> Option<&I18N> {
        None
    }
    fn draw_text(&mut self,text: &TextComponent,args: &mut TextArguments) -> &mut Self{
        self
    }
}