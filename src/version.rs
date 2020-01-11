use std::fmt::{Debug, Formatter, Error};

#[derive(Default,Eq,PartialEq,Copy,Clone,Ord,PartialOrd,Hash)]
pub struct Version{
    major: u8,
    minor: u8
}

#[derive(Eq,PartialEq,Copy,Clone,Hash)]
pub struct VersionRange{
    lower: Version,
    upper: Version
}

impl Debug for Version{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        (major as u16 + 1).fmt(f).and_then(||{
            f.write_str(".");
            self.minor.fmt(f)
        })
    }
}

impl VersionRange{
    const ALL_VERSIONS: VersionRange = VersionRange(Version(0,0),Version(255,255));
    const NO_VERSIONS: VersionRange = VersionRange(Version(0,1),Version(0,0));
    pub const fn in_range(&self,v:Version) -> bool{
        return lower <= v && v<=upper;
    }
    pub const fn check_range(&self,v:Version) -> Option<Version>{
        if in_range(v) {
            return Some(v)
        }else {
            return None
        }
    }
    pub const fn clamp(&self,v:Version) -> Version{
        if lower > v{
            return lower
        }else if upper < v{
            return upper
        }else {
            return v
        }
    }
}

impl Version{
    pub const v1_0: Version = Version(0,0);
    pub const fn new(major:u16,minor:u8) -> Option<Version>{
        if major > 256{
            None
        }else{
            Some(unsafe { Version::new_unchecked(major, minor) })
        }
    }
    pub unsafe fn new_unchecked(major:u16,minor:u8)->Version{
        Version{major: (major-1)as u8,minor}
    }
    pub fn from_serial(val:u16) -> Version{
        return Version{major: (val>>8) as u8,minor: (val&0xff) as u8}
    }

    pub fn to_serial(self) -> u16{
        return (self.major as u16) << 8 | self.minor as u16;
    }
    pub const fn origin(self) -> Version{
        Version{major,minor:0}
    }

    pub const fn same_origin(self) -> VersionRange{
        return VersionRange{ lower: origin(), upper: self }
    }

}

impl From<Version> for VersionRange{
    const fn from(v: Version) -> Self {
        VersionRange{lower:v,upper:v}
    }
}

