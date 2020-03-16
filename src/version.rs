use std::fmt::{Formatter, Error, Display, Debug};
use std::ops::RangeBounds;

#[derive(Default,Eq,PartialEq,Copy,Clone,Ord,PartialOrd,Hash)]
pub struct Version(pub u8,pub u8);


impl Display for Version{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("{}",self.0 as u16 + 1))?;
		f.write_fmt(format_args!("{}",self.1));
		Ok(())
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


