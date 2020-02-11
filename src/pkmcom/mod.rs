use std::num::Wrapping;
use std::ops::Deref;

pub mod net;
pub mod irc;

#[cfg(feature="pkmcom_tcp")]
pub mod tcp;

#[cfg(feature="lan")]
pub mod lan;

pub trait PkmComHash{
    fn hashcode(&self) -> Wrapping<u32>;
    fn size(&self) -> u32;
}

impl PkmComHash for bool{
    fn hashcode(&self) -> Wrapping<u32> {
        if self==true{
            Wrapping(1337u32)
        }else{
            Wrapping(1331u32)
        }
    }
    fn size(&self) -> u32{
        1
    }
}

impl PkmComHash for i8{
    fn hashcode(&self) -> Wrapping<u32> {
        Wrapping(self as u32)
    }
    fn size(&self) -> u32{
        1
    }
}

impl PkmComHash for u8{
    fn hashcode(&self) -> Wrapping<u32> {
        Wrapping(self as u32)
    }
    fn size(&self) -> u32{
        1
    }
}

impl PkmComHash for i16{
    fn hashcode(&self) -> Wrapping<u32> {
        Wrapping(self as u32)
    }
    fn size(&self) -> u32{
        2
    }
}

impl PkmComHash for u16{
    fn hashcode(&self) -> Wrapping<u32> {
        Wrapping(self as u32)
    }
    fn size(&self) -> u32{
        2
    }
}

impl PkmComHash for i32{
    fn hashcode(&self) -> Wrapping<u32> {
        Wrapping(self as u32)
    }
    fn size(&self) -> u32{
        4
    }
}

impl PkmComHash for u32{
    fn hashcode(&self) -> Wrapping<u32> {
        Wrapping(self as u32)
    }
    fn size(&self) -> u32{
        4
    }
}

impl PkmComHash for i64{
    fn hashcode(&self) -> Wrapping<u32> {
        Wrapping((self as u32) ^ (self>>32) as u32)
    }
    fn size(&self) -> u32{
        8
    }
}

impl PkmComHash for f32{
    fn hashcode(&self) -> Wrapping<u32>{
        return unsafe{ std::mem::transmute::<f32,u32>(*self)}.hashcode()
    }
    fn size(&self) -> u32{
        4
    }
}

impl PkmComHash for f64{
    fn hashcode(&self) -> Wrapping<u32>{
        return unsafe{std::mem::transmute::<f64,i64>(*self)}.hashcode()
    }
    fn size(&self) -> u32{
        8
    }
}

impl<T: PkmComHash> PkmComHash for [T]{
    fn hashcode(&self) -> Wrapping<u32> {
        let mut hash = Wrapping(0u32);
        for a in self{
            hash *= 31;
            hash += a.hashcode();
        }
        hash
    }
    fn size(&self) -> u32{
        self.len() as u32
    }
}

impl<T: PkmComHash,Size: u32> PkmComHash for [T;Size]{
    fn hashcode(&self) -> Wrapping<u32> {
        let mut hash = Wrapping(0u32);
        for a in self{
            hash *= 31;
            hash += a.hashcode();
        }
        hash
    }
    fn size(&self) -> u32{
        Size
    }
}

impl PkmComHash for std::string::String{
    fn hashcode(&self) -> Wrapping<u32> {
        self.as_bytes().hashcode()
    }
    fn size(&self) -> u32{
        (2 + self.len()) as u32
    }
}

impl<T: PkmComHash,P: Deref<T>> PkmComHash for P{
    fn hashcode(&self) -> Wrapping<u32> {
        self.deref().hashcode()
    }
    fn size(&self) -> u32{
        self.deref().size()
    }
}

impl PkmComHash for !{
    fn hashcode(self) -> Wrapping<u32>{
        self
    }
    fn size(self) -> u32{
        self
    }
}

impl<A: PkmComHash + ?Sized> PkmComHash for(A,){
    fn hashcode(&self) -> Wrapping<u32> {
        self.0.hashcode()
    }
    fn size(&self) -> u32{
        self.0.size()
    }
}

impl<B: PkmComHash,A: PkmComHash + ?Sized> PkmComHash for (B,A){
    fn hashcode(&self) -> Wrapping<u32>{
        self.0.hashcode()*31+self.1.hashcode()
    }

    fn size(&self) -> u32 {
        self.0.size()+self.1.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,A: PkmComHash + ?Sized> PkmComHash for (B,C,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,a) = self;
        (b,c).hashcode()*31+a.hashcode()
    }

    fn size(&self) -> u32 {
        let (b,c,a) = self;
        b.size()+c.size()+a.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,A: PkmComHash + ?Sized> PkmComHash for (B,C,D,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,a) = self;
        (b,c,d).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,a) = self;
        b.size()+c.size()+d.size()+a.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,A: PkmComHash + ?Sized> PkmComHash for (B,C,D,E,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,a) = self;
        (b,c,d,e).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,a) = self;
        b.size()+c.size()+d.size()+e.size()+a.size()
    }
}

impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,F: PkmComHash,A: PkmComHash + ?Sized>
    PkmComHash for (B,C,D,E,F,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,f,a) = self;
        (b,c,d,e,f).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,f,a) = self;
        b.size()+c.size()+d.size()+e.size()+f.size()+a.size()
    }
}

impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,F: PkmComHash,G: PkmComHash,A: PkmComHash + ?Sized>
PkmComHash for (B,C,D,E,F,G,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,f,g,a) = self;
        (b,c,d,e,f,g).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,f,g,a) = self;
        b.size()+c.size()+d.size()+e.size()+f.size()+g.size()+a.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,F: PkmComHash,G: PkmComHash,H: PkmComHash,A: PkmComHash + ?Sized>
PkmComHash for (B,C,D,E,F,G,H,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,f,g,h,a) = self;
        (b,c,d,e,f,g,h).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,f,g,h,a) = self;
        b.size()+c.size()+d.size()+e.size()+f.size()+g.size()+h.size()+a.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,F: PkmComHash,G: PkmComHash,H: PkmComHash,I: PkmComHash,A: PkmComHash + ?Sized>
PkmComHash for (B,C,D,E,F,G,H,I,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,f,g,h,i,a) = self;
        (b,c,d,e,f,g,i,h).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,f,g,h,i,a) = self;
        b.size()+c.size()+d.size()+e.size()+f.size()+g.size()+h.size()+i.size()+a.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,F: PkmComHash,G: PkmComHash,H: PkmComHash,I: PkmComHash,J: PkmComHash,A: PkmComHash + ?Sized>
PkmComHash for (B,C,D,E,F,G,H,I,J,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,f,g,h,i,j,a) = self;
        (b,c,d,e,f,g,h,i,j).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,f,g,h,i,j,a) = self;
        b.size()+c.size()+d.size()+e.size()+f.size()+g.size()+h.size()+i.size()+j.size()+a.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,F: PkmComHash,G: PkmComHash,H: PkmComHash,I: PkmComHash,J: PkmComHash,K: PkmComHash,A: PkmComHash + ?Sized>
PkmComHash for (B,C,D,E,F,G,H,I,J,K,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,f,g,h,i,j,k,a) = self;
        (b,c,d,e,f,g,h,i,j,k).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,f,g,h,i,j,k,a) = self;
        b.size()+c.size()+d.size()+e.size()+f.size()+g.size()+h.size()+i.size()+j.size()+k.size()+a.size()
    }
}
impl<B: PkmComHash,C: PkmComHash,D: PkmComHash,E: PkmComHash,F: PkmComHash,G: PkmComHash,H: PkmComHash,I: PkmComHash,J: PkmComHash,K: PkmComHash,L: PkmComHash,A: PkmComHash + ?Sized>
PkmComHash for (B,C,D,E,F,G,H,I,J,K,A){
    fn hashcode(&self) -> Wrapping<u32>{
        let (b,c,d,e,f,g,h,i,j,k,l,a) = self;
        (b,c,d,e,f,g,h,i,j,k,l).hashcode()*31+a.hashcode()
    }
    fn size(&self) -> u32 {
        let (b,c,d,e,f,g,h,i,j,k,l,a) = self;
        b.size()+c.size()+d.size()+e.size()+f.size()+g.size()+h.size()+i.size()+j.size()+k.size()+l.size()+a.size()
    }
}

impl PkmComHash for !{
    fn hashcode(&self) -> Wrapping<u32> {
        self.clone()
    }

    fn size(&self) -> u32 {
        self.clone()
    }
}