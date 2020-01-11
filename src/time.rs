

use std::ops::{Add, Sub,AddAssign,SubAssign};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(PartialOrd)]
pub struct Duration {
    seconds: i64,
    nanos: u32
}

impl Duration{
    pub const ZERO: Duration = Default::default();
    pub const MAX: Duration = Duration{seconds: std::i64::MAX, nanos: 999999999};
    pub const MIN: Duration = Duration{seconds: std::i64::MIN, nanos: 0};
    pub const EPSILON: Duration = Duration{seconds: 0,nanos: 1};
    pub fn get_seconds(&self) -> i64{
        return self.seconds;
    }
    pub fn get_nanos(&self) -> u32{
        return self.nanos;
    }
}

impl Add for Duration{
    type Output = Self;

    fn add(&self, rhs: &Self) -> Self::Output {
        let mut seconds = self.seconds + rhs.seconds;
        let mut nanos = self.nanos + rhs.nanos;
        while nanos>=1000000000 {
            seconds+=1;
            nanos-=1000000000;
        }
        return Self {seconds,nanos};
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.seconds += rhs.seconds;
        self.nanos += rhs.nanos;
        if nanos>=1000000000 {
            seconds += 1;
            nanos -= 1000000000;
        }
    }
}

impl Sub for Duration{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut seconds = self.seconds + rhs.seconds;
        let mut nanos :i32 = ((self.nanos as i32) - (rhs.nanos as i32));
        while nanos<0 {
            seconds-=1;
            nanos+=1000000000;
        }
        return Self {seconds, nanos: nanos as u32};
    }
}

impl From<std::time::Duration> for Duration{
    fn from(val: std::time::Duration) -> Self {
        let seconds = val.as_secs() as i64;
        let nanos = val.subsec_nanos();
        return Self{seconds,nanos};
    }
}