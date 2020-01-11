use crate::version::{Version, VersionRange};

const MAGIC: [u8;4] = [0xAD, 0x4E, 0x42,0x54];
const CRYPTO_MAGIC: [u8;4] = [0xEC, 0x4E,0x42, 0x54];

const CURRENT_VERSION: Version = Version(0,2);
const ACCEPTABLE_VERSIONS: VersionRange = CURRENT_VERSION.same_origin();
