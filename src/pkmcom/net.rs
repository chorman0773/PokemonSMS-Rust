use crate::io;
use crate::pkmcom::PkmComHash;
use std::num::Wrapping;
use crate::io::{InputStream, OutputStream};
use crate::io::dataio::{DataOutputStream, BinaryIOWritable, DataInputStream, BinaryIOReadable};

#[derive(PartialEq,Eq)]
pub enum Side{
    Client,
    Server,
    Hub
}

type Result<T> = core::result::Result<T,std::string::String>;


pub trait Packet : PkmComHash{
    fn packet_id(&self) -> u8;
    fn write_packet<S: io::dataio::DataOutput>(&self,out: &mut S);
    fn read_packet<S: io::dataio::DataInput>(&mut self,din: &mut S) -> Result<()>;
    fn create(id: u8) -> Option<Self>;
}

pub enum Tie<A: Packet,B: Packet>{
    First(A),
    Second(B)
}

impl<A: Packet,B: Packet> PkmComHash for Tie<A,B>{
    fn hashcode(&self) -> Wrapping<u32>{
        match self{
            Tie::First(a) => a.hashcode(),
            Tie::Second(b) => b.hashcode()
        }
    }

    fn size(&self) -> u32 {
        match self{
            Tie::First(a) => a.size(),
            Tie::Second(b) => b.size()
        }
    }
}

impl<A: Packet,B: Packet> Packet for Tie<A,B>{
    fn packet_id(&self) -> u8 {
        match self{
            Tie::First(a) => a.packet_id(),
            Tie::Second(b) => b.packet_id()
        }
    }

    fn write_packet<S: io::dataio::DataOutput>(&self, out: &mut S) {
        match self{
            Tie::First(a) => a.write_packet(out),
            Tie::Second(b) => b.write_packet(out)
        }
    }

    fn read_packet<S: io::dataio::DataInput>(&mut self, din: &mut S) -> Result<()> {
        match self{
            Tie::First(a) => a.read_packet(out),
            Tie::Second(b) => b.read_packet(out)
        }
    }

    fn create(id: u8) -> Option<Self> {
        if let Some(a) = A::create(id){
            Some(Tie::First(a))
        }else if let Some(b) = B::create(id){
            Some(Tie::Second(b))
        }else{
            None
        }
    }
}


pub unsafe trait NetHandler{
    type Addr;
    fn send_packet<P: Packet>(&mut self,packet: P);
    fn recv_packet<P: Packet>(&mut self) -> Result<P>;
    fn is_remote(&self) -> bool;
    fn close(&mut self);
    fn connect(addr: Addr) -> Result<Self>;
}

pub unsafe trait NetController{
    type Handler: NetHandler;
    fn accept(&mut self) -> Option<&mut Self::Handler>;
    fn listen(addr: Addr) -> Result<Self>;
}

pub trait ProtocolErrorHandler{
    fn handle_protocol_error<Handler: NetHandler>(&self,handler:&mut Handler,s: &std::string::String);
}

pub trait Service{
    type Addr;
    type InputStream : InputStream;
    type OutputStream: OutputStream;
    fn get_name() -> &'static str;
    fn addr_from_pair(addr: std::string::String,port: u16) -> Addr;
    fn add_to_pair(addr: Addr) -> (std::string::String,u16);
    fn listen(addr: Addr) -> Result<Self>;
    fn connect(addr: Addr) -> Result<Self>;
    fn input_stream(&mut self) -> Option<&mut Self::InputStream>;
    fn output_stream(&mut self) -> Option<&mut Self::OutputStream>;
    fn accept(&mut self) -> Option<Self>;
    fn close(&mut self);
}

pub struct Handler<Serv: Service>{
    service: Serv,
    remote: bool
}

impl<Serv: Service> Handler<Serv>{
    pub fn new(srv: Serv,remote: bool) -> Self{
        Self{service: srv,remote}
    }
}

unsafe impl<Serv: Service> NetHandler for Handler<Serv>{
    type Addr = Serv::Addr;

    fn send_packet<P: Packet>(&mut self, packet: P) {
        if let Some(out) = self.service.output_stream(){
            let mut dout = DataOutputStream::new(out,ByteOrder::BigEndian);
            packet.packet_id().write(&mut dout);
            packet.hashcode().0.write(&mut dout);
            packet.write_packet(&mut dout);
        }

    }

    fn recv_packet<P: Packet>(&mut self) -> Result<P> {
        return if let Some(ins) = self.service.input_stream() {
            let mut din = DataInputStream::new(ins, ByteOrder::BigEndian);
            let pid = u8::read(&mut din)?;
            let hash = u32::read(&mutdin)?;
            let size = i32::read(&mut din)?;
            if let Some(mut packet) = P::create(pid) {
                packet.read_packet(&mut din)?;
                let Wrapping(phash) = packet.hashcode();
                let len = packet.length();
                if phash != hash {
                    Err(format!("Packet Hash mismatch: got {:x} expected {:x}", hash, phash))
                } else if len != size {
                    Err(format!("Packet Size mismatch: got {:x} expected {:x}", size, len))
                }else {
                    Ok(packet)
                }
            } else {
                Err(format!("Unexpected Packet with id {}", pid))
            }
        } else {
            Err("Invalid Service bound, must have a bound InputStream".to_string())
        }
    }

    fn is_remote(&self) -> bool {
        self.remote
    }

    fn close(&mut self) {
        self.service.close()
    }

    fn connect(addr: Self::Addr) -> Result<Self> {
        let service = Serv::connect(addr)?;
        Ok(Self{service,remote: false})
    }
}

impl<Serv: Service> Drop for Handler<Serv>{
    fn drop(&mut self) {
        self.close()
    }
}

struct Controller<Serv: Service>{
    service: Serv,
    inner: Vec<Handler<Serv>>
}

unsafe impl<Serv: Service> NetController for Controller<Serv>{
    type Handler = Handler<Serv>;

    fn accept(&mut self) -> Option<&mut Self::Handler> {
        let handler = self.service.accept()?;
        self.inner.push(Handler::new(handler,true));
        self.inner.last_mut()
    }

    fn listen(addr: Self::Addr) -> Result<Self> {
        Ok(Self{service: Serv::listen(addr)?,inner: Vec::new()})
    }
}