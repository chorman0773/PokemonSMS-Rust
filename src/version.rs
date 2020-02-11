use std::fmt::{Formatter, Error, Display, Debug};
use std::ops::RangeBounds;

#[derive(Default,Eq,PartialEq,Copy,Clone,Ord,PartialOrd,Hash,Display)]
pub struct Version(pub u8,pub u8);


impl Display for Version{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        (major as u16 + 1).fmt(f).and_then(||{
            f.write_str(".");
            self.minor.fmt(f)
        })
    }
}


impl Version{
    pub const fn origin(self) -> Version
    {
        Version(self.0,0)
    }

    pub const fn same_origin(self) -> impl RangeBounds<Version>{
        return self.origin()..=self
    }
}


