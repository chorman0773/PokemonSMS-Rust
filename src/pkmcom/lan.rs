use crate::pkmcom::PkmComHash;
use std::num::Wrapping;

pub struct BaseProtocol{
    pub service: std::string::String,
    pub addr: std::string::String,
    pub port: u16
}

impl PkmComHash for BaseProtocol{
    fn hashcode(&self) -> Wrapping<u32> {
        (&self.service,&self.addr,&self.port).hashcode()
    }

    fn size(&self) -> u32 {
        (&self.service,&self.addr,&self.port).size()
    }
}

pub enum LanDiscovery{
    ServiceAdvertisement(BaseProtocol,u8,Box<[std::string::String]>),
    ListeningForServices,
    NoLongerAvailable(BaseProtocol)
}

pub use LanDiscovery::*;
use crate::pkmcom::net::Packet;
use crate::io;

impl PkmComHash for LanDiscovery{
    fn hashcode(&self) -> Wrapping<u32> {
        match self{
            ServiceAdvertisement(base,flags,arr)=>(0u8,base,flags,arr).hashcode(),
            ListeningForServices => 1u8.hashcode(),
            NoLongerAvailable(base) => (2u8,base).hashcode()
        }
    }

    fn size(&self) -> u32 {
        match self{
            ServiceAdvertisement(base,flags,arr) => (base,flags,arr).size(),
            ListeningForServices => 0,
            NoLongerAvailable(base) => base.size()
        }
    }
}

impl Packet for LanDiscovery{
    fn packet_id(&self) -> u8 {
        match self{
            ServiceAdvertisement(_,_,_) => 0,
            ListeningForServices => 1,
            NoLongerAvailable(_) => 2
        }
    }

    fn write_packet<S: io::dataio::DataOutput>(&self, out: &mut S) {
        unimplemented!()
    }

    fn read_packet<S: io::dataio::DataInput>(&mut self, din: &mut S) -> Result<(), String> {
        unimplemented!()
    }

    fn create(id: u8) -> Option<Self> {
        unimplemented!()
    }

}
