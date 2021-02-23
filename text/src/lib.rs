use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TextDelay {
    seconds: u64,
    #[serde(default)]
    nanos: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextComponent {
    Text { text: String, style: Option<Style> },
    Translation { translate: String },
    Command(TextCommand),
    Group(Vec<TextComponent>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
#[serde(rename_all = "snake_case")]
pub enum TextCommand {
    Style(Style),
    Scroll(u32),
    DelayScroll(TextDelay),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    pub bold: bool,
    pub italics: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Black,
    Red,
    Green,
    Gold,
    Blue,
    Magenta,
    Cyan,
    Grey,
    DarkGrey,
    BrightRed,
    BrightGreen,
    Yellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    White,
}
