use crate::version::{Version, VersionRange};
use std::ops::RangeBounds;

pub const MAGIC: [u8;4] = [0xAD, 0x4E, 0x42,0x54];
pub const CRYPTO_MAGIC: [u8;4] = [0xEC, 0x4E,0x42, 0x54];

pub const CURRENT_VERSION: Version = Version(0,3);
pub const ACCEPTABLE_VERSIONS: impl RangeBounds<Version> = CURRENT_VERSION.same_origin();
