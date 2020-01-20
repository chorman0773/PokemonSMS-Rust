use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU64;
use std::num::Wrapping;
use core::panicking::panic;

pub struct Random{
    seed: u64,
    nextNextGaussian: Option<f64>
}

impl Random{
    fn gen_unique_seed() -> u64{
        static num: AtomicU64 = AtomicU64::new(76085871501389);
        const cnum: u64 = 167477818489483;
        let mut val = num.get();
        let timeval = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        while let Err(nval) = num.compare_exchange(val,cnum*val+1,Ordering::SeqCst,Ordering.SeqCst){
            val = nval;
        }
        return (Wrapping(val)*31+Wrapping(timeval.as_micros())) as Wrapping<u64>.0;
    }
    fn init_randomize_seed(seed: u64) -> u64{
        (seed ^ 0x5DEECE66Du64) & ((1u64 << 48) - 1)
    }
    pub fn with_seed(seed: u64) -> Self{
        Random{seed: Self::init_randomize_seed(seed),nextNextGaussian: None}
    }
    pub fn new_seeded() -> Self{
        Self::with_seed(Self::gen_unique_seed())
    }
    unsafe fn next<T: From<u32>>(&mut self,bits: u32) -> T{
        //Expects: 0 <= bits <= 32
        self.seed = (seed * 0x5DEECE66Du8 + 0xBu8) & ((1u8 << 48) - 1);
        ((self.seed >> (48u64 - bits)) as u32).into()
    }
    pub fn next_int(&mut self) -> i32{
        unsafe { self.next(32) }
    }
    pub fn next_int_bounded(&mut self,max: u32) -> i32{
        if max > i32::max_value() as u32{
            panic!("Maximum Value cannot exceed {}. {} was given",i32::max_value(),max);
        }
        let bound = max as i32;
        if (bound & -bound) == bound { // i.e., bound is a power of 2
            return (((bound as u64) * unsafe{ self.next::<u64>(31)}) >> 31) as i32;
        }

        let mut bits: i32;
        let mut val: i32;
        loop {
            bits = unsafe{self.next(31)};
            val = bits % bound;
            if bits - val + (bound-1) >= 0{
                break val;
            }
        }
    }
    pub fn next_bool(&mut self) -> bool{
        unsafe{ self.next(1)}
    }

    pub fn next_long(&mut self) -> i64 {
        unsafe{ self.next(32) << 32u8 | self.next(32)}
    }

    pub fn next_float(&mut self) -> f32 {
        unsafe { self.next(24)/((1<<24)as f32)}
    }

    pub fn next_double(&mut self) -> f64 {
        unsafe {(self.next(26) << 27u64 + self.next(27))/((1u64<<53) as f64)}
    }

    pub fn next_gaussian(&mut self) -> f64{
        if let Some(next) = self.nextNextGaussian{
            self.nextNextGaussian = None;
            next
        }else{
            let mut v1: f64;
            let mut v2: f64;
            let mut s: f64;
            loop{
                v1 = 2 * nextDouble() - 1;   // between -1.0 and 1.0
                v2 = 2 * nextDouble() - 1;   // between -1.0 and 1.0
                s = v1 * v1 + v2 * v2;
                if s > 0f64 && s< 1f64{
                    break;
                }
            }
            let multiplier = (-2.0 * s.ln()/s).sqrt();
            self.nextNextGaussian = Some(v2*multiplier);
            v1*multiplier
        }
    }

    pub fn next_bytes(&mut self,bytes: &mut [u8]){
        let max = bytes.len()/4;
        for n in 0..max{
            let val = unsafe{self.next::<u32>(32)};
            bytes[4*n] = val as u8;
            bytes[4*n+1] = (val>>8) as u8;
            bytes[4*n+2] = (val>>16) as u8;
            bytes[4*n+3] = (val>>24) as u8;
        }
        match bytes.len()%4{
            0 => {},
            i @ 1..3 => {
                let val = unsafe{self.next::<u32>(32)};
                (0..i).for_each(|i| bytes[4 * max + i] = (val >> (8 * i) as u32) as u8);
            },
            _ => unsafe{ std::intrinsics::unreachable()}
        }
    }

    pub fn ints(&mut self) -> impl Iterator<Item=i32>{
        Generator{rand: self,f: &Random::next_int}
    }
    pub fn ints_bounded(&mut self,min: i32,max: i32) -> Result<impl Iterator<Item=i32>,std::string::String>{
        if max <= min{
            Err(format!("min ({}) must be less than max ({})",max,min))
        }
        Ok(Generator{rand: self,f: &|r| r.next_int_bounded((max-min) as u32)+min})
    }
    pub fn doubles(&mut self) -> impl Iterator<Item=f64>{
        Generator{rand: self,f: &Random::next_double}
    }
    pub fn doubles_bounded(&mut self,min: f64,max: f64) -> Result<impl Iterator<Item=f64>,std::string::String>{
        if max <= min{
            Err(format!("min ({}) must be less than max ({})",max,min))
        }
        Ok(Generator{rand: self,f: &|r| r.next_double()*(max-min)+min})
    }
    pub fn longs(&mut self) -> impl Iterator<Item=i64>{
        Generator{rand: self,f: &Random::next_long}
    }
    pub fn gaussians(&mut self) -> impl Iterator<Item=i64>{
        Generator{rand: self,f: &Random::next_gaussian}
    }

    pub fn booleans(&mut self) -> impl Iterator<Item=bool>{
        Generator{rand: self,f: &Random::next_bool}
    }

    pub fn next_value<T,F: ?Sized>(&mut self,f: &F) ->T
        where F: FnOnce(&mut Random)->T{
        f(self)
    }

    pub fn generator<'a,'b,T,F: ?Sized>(&'a mut self,f: &'b F) -> impl Iterator<Item=T>
        where F: Fn(&mut Random)->T{
        Generator::<'a,'b>{rand: self,f}
    }
}

struct Generator<'a,'b,T,F: ?Sized>
 where F: Fn(&'a mut Random)->T{
    rand: &'a mut Random,
    f: &'b F
}

impl<'a,'b,T,F> Iterator for Generator<'a,'b,T,F>
 where F: Fn(&'a mut Random)->T{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.f(&mut self.rand)
    }

    fn size_hint(&self) -> (usize,Option<usize>){
        (usize::max_value(),None)
    }
}

