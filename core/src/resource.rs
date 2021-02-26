use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
pub struct ResourceLocation {
    domain: String,
    path: String,
}

#[derive(Clone, Debug)]
pub enum FromStrError {
    InvalidDomain(String),
    InvalidPath(String),
    NotEnoughComponents(String),
}

impl Display for FromStrError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidDomain(text) => f.write_fmt(format_args!(
                "Domain {} is not a valid resource domain",
                text
            )),
            Self::InvalidPath(text) => {
                f.write_fmt(format_args!("Path {} is not a valid path", text))
            }
            Self::NotEnoughComponents(text) => {
                f.write_fmt(format_args!("{} is not a resource location", text))
            }
        }
    }
}

impl Error for FromStrError {}

lazy_static! {
    static ref DOMAIN_REGEX: Regex = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap();
    static ref PATH_REGEX: Regex =
        Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*(/[A-Za-z_][A-Za-z0-9_]*)*$").unwrap();
}

impl FromStr for ResourceLocation {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(':');
        let domain = components
            .next()
            .ok_or_else(|| FromStrError::NotEnoughComponents(s.to_string()))?;
        if !DOMAIN_REGEX.is_match(domain) {
            return Err(FromStrError::InvalidDomain(domain.to_string()));
        }
        let path = components
            .next()
            .ok_or_else(|| FromStrError::NotEnoughComponents(s.to_string()))?;
        if !PATH_REGEX.is_match(path) {
            return Err(FromStrError::InvalidPath(path.to_string()));
        }
        Ok(Self {
            domain: domain.to_string(),
            path: path.to_string(),
        })
    }
}

impl<'a> TryFrom<&'a str> for ResourceLocation {
    type Error = FromStrError;
    fn try_from(s: &'a str) -> Result<ResourceLocation, FromStrError> {
        s.parse()
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.domain, self.path))
    }
}

impl From<ResourceLocation> for String {
    fn from(r: ResourceLocation) -> Self {
        r.to_string()
    }
}

impl ResourceLocation {
    pub fn new(domain: String, path: String) -> Self {
        if !DOMAIN_REGEX.is_match(&domain) {
            panic!("{:?}", FromStrError::InvalidDomain(domain))
        }
        if !PATH_REGEX.is_match(&path) {
            panic!("{:?}", FromStrError::InvalidPath(path.to_string()))
        }

        Self { domain, path }
    }
    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
