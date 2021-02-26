use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TextDelay {
    pub seconds: u64,
    #[serde(default)]
    pub nanos: u32,
}

impl From<TextDelay> for Duration {
    fn from(v: TextDelay) -> Duration {
        Duration::new(v.seconds, v.nanos)
    }
}

impl From<Duration> for TextDelay {
    fn from(v: Duration) -> TextDelay {
        TextDelay {
            seconds: v.as_secs(),
            nanos: v.subsec_nanos(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextComponent {
    RawText(String),
    Text { text: String, style: Option<Style> },
    Translation { translate: String },
    Command(TextCommand),
    ImplicitGroup(Vec<TextComponent>),
    Group { group: Vec<TextComponent> },
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

pub mod embed_json {
    use super::TextComponent;
    use serde::{Deserialize, Serialize};

    pub fn serialize<S>(value: &TextComponent, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let string =
            serde_json::to_string(value).map_err(|e| <S::Error as serde::ser::Error>::custom(e))?;
        string.serialize(ser)
    }

    pub fn deserialize<'de, D>(de: D) -> Result<TextComponent, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = <&str as Deserialize>::deserialize(de)?;
        serde_json::from_str(str).map_err(|e| <D::Error as serde::de::Error>::custom(e))
    }
}
