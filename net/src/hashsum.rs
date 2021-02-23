use std::num::Wrapping;

use io::{uuid::UUID, version::Version};

pub trait Hashcode {
    fn hashcode(&self) -> i32;
}

impl Hashcode for bool {
    fn hashcode(&self) -> i32 {
        if *self {
            1337
        } else {
            1331
        }
    }
}

impl Hashcode for u8 {
    fn hashcode(&self) -> i32 {
        *self as i32
    }
}

impl Hashcode for i8 {
    fn hashcode(&self) -> i32 {
        *self as i32
    }
}

impl Hashcode for u16 {
    fn hashcode(&self) -> i32 {
        *self as i32
    }
}

impl Hashcode for i16 {
    fn hashcode(&self) -> i32 {
        *self as i32
    }
}

impl Hashcode for u32 {
    fn hashcode(&self) -> i32 {
        *self as i32
    }
}

impl Hashcode for i32 {
    fn hashcode(&self) -> i32 {
        *self
    }
}

impl Hashcode for i64 {
    fn hashcode(&self) -> i32 {
        (*self >> 32) as i32 ^ (*self as i32)
    }
}

impl Hashcode for UUID {
    fn hashcode(&self) -> i32 {
        let (high, low) = self.into_fields();
        (high as i64)
            .hashcode()
            .wrapping_mul(31)
            .wrapping_add((low as i64).hashcode())
    }
}

impl Hashcode for Version {
    fn hashcode(&self) -> i32 {
        self.major().get().hashcode() * 31 + self.minor().hashcode()
    }
}

impl<T: Hashcode> Hashcode for [T] {
    fn hashcode(&self) -> i32 {
        let mut wrapping = Wrapping::<i32>(0);
        for v in self {
            wrapping *= Wrapping(31);
            wrapping += Wrapping(v.hashcode());
        }
        wrapping.0
    }
}

impl Hashcode for str {
    fn hashcode(&self) -> i32 {
        self.as_bytes().hashcode()
    }
}

impl<T: Hashcode> Hashcode for &'_ T {
    fn hashcode(&self) -> i32 {
        <T as Hashcode>::hashcode(self)
    }
}

impl<T: Hashcode> Hashcode for &'_ mut T {
    fn hashcode(&self) -> i32 {
        <T as Hashcode>::hashcode(self)
    }
}

impl<T: Hashcode> Hashcode for Vec<T> {
    fn hashcode(&self) -> i32 {
        <[T]>::hashcode(self)
    }
}

impl<T: Hashcode> Hashcode for Option<T> {
    fn hashcode(&self) -> i32 {
        if let Some(x) = self {
            x.hashcode()
        } else {
            0
        }
    }
}

impl Hashcode for String {
    fn hashcode(&self) -> i32 {
        self.as_bytes().hashcode()
    }
}
