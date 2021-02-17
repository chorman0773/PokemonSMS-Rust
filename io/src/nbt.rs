//! Module for the Named Binary Tag, and ShadeNBT format
//! This module deals with representing the NBT format as a structure,
//!  and binary serialization. It does not perform Shade version checking, nor does it parse the Shade or CryptoShade header
//!

use core::panic;

use crate::data::{DeserializeCopy, Deserializeable, Serializeable};

pub mod array {
    //!
    //! Types for NBT_Tag*Array
    use std::{
        mem::ManuallyDrop,
        ops::{Deref, DerefMut, Index, IndexMut},
        ptr,
        slice::{self, SliceIndex},
    };
    ///
    /// A type which can store a dynamic, fixed-size array of T
    #[derive(Clone, Debug)]
    pub struct NbtArray<T> {
        inner: Box<[T]>,
    }

    impl<T> Default for NbtArray<T> {
        fn default() -> Self {
            Self {
                inner: Box::new(<[T; 0]>::default()),
            }
        }
    }

    impl<T> Deref for NbtArray<T> {
        type Target = [T];

        #[inline]
        fn deref(&self) -> &Self::Target {
            self.inner.deref()
        }
    }

    impl<T> DerefMut for NbtArray<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.inner.deref_mut()
        }
    }

    impl<T> From<Vec<T>> for NbtArray<T> {
        #[inline]
        fn from(v: Vec<T>) -> Self {
            Self {
                inner: v.into_boxed_slice(),
            }
        }
    }

    impl<T> From<Box<[T]>> for NbtArray<T> {
        #[inline]
        fn from(v: Box<[T]>) -> Self {
            Self { inner: v }
        }
    }

    impl<T, const N: usize> From<[T; N]> for NbtArray<T> {
        #[inline]
        fn from(v: [T; N]) -> Self {
            Self { inner: Box::new(v) }
        }
    }

    impl<T, I: SliceIndex<[T]>> Index<I> for NbtArray<T> {
        type Output = I::Output;

        #[inline]
        fn index(&self, index: I) -> &Self::Output {
            self.inner.index(index)
        }
    }

    impl<T, I: SliceIndex<[T]>> IndexMut<I> for NbtArray<T> {
        #[inline]
        fn index_mut(&mut self, index: I) -> &mut Self::Output {
            self.inner.index_mut(index)
        }
    }

    ///
    /// Iterator for NbtArray<T>
    pub struct IntoIter<T> {
        inner: Box<[ManuallyDrop<T>]>,
        position: usize,
    }

    impl<T> Drop for IntoIter<T> {
        fn drop(&mut self) {
            for i in self.position..self.inner.len() {
                // SAFETY:
                // from position..len has not been taken. position is incremented first
                unsafe { ManuallyDrop::drop(&mut self.inner[i]) }
            }
        }
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.position = self.position.checked_add(1).unwrap();
            // SAFETY:
            // position is incremented first, so any item taken here will never be visited again
            self.inner
                .get_mut(self.position - 1)
                .map(|m| unsafe { ManuallyDrop::take(m) })
        }
    }

    impl<T> IntoIterator for NbtArray<T> {
        type Item = T;

        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            let len = self.inner.len();
            let ptr = Box::into_raw(self.inner).cast::<T>();
            // SAFETY: the slice is from self.inner: Box<[T]>. ManuallyDrop<T> is transparent over T
            let inner = unsafe {
                Box::from_raw(ptr::slice_from_raw_parts_mut(
                    ptr.cast::<ManuallyDrop<T>>(),
                    len,
                ))
            };
            IntoIter { inner, position: 0 }
        }
    }

    ///
    /// Iterator over references to elements of an NbtArray
    pub struct Iter<'a, T: 'a>(slice::Iter<'a, T>);

    impl<'a, T: 'a> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<&'a T> {
            self.0.next()
        }
    }

    ///
    /// Iterator over mutable references to elements of an NbtArray
    pub struct IterMut<'a, T: 'a>(slice::IterMut<'a, T>);

    impl<'a, T: 'a> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<&'a mut T> {
            self.0.next()
        }
    }

    impl<T> NbtArray<T> {
        ///
        /// Returns an iterator of references to the array elements
        pub fn iter(&self) -> Iter<T> {
            Iter(self.inner.iter())
        }
        ///
        /// Returns an iterator of mut references to the array elements
        pub fn iter_mut(&mut self) -> IterMut<T> {
            IterMut(self.inner.iter_mut())
        }
    }

    impl<'a, T> IntoIterator for &'a NbtArray<T> {
        type Item = &'a T;

        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut NbtArray<T> {
        type Item = &'a mut T;

        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
}

fake_enum! {
    #[repr(u8)]
    #[derive(Default,Hash)]
    pub enum struct TagType{
        //! The Type of an NBT Tag

        /// Placeholder Tag at the end of Compounds
        End = 0,
        /// TAG_Byte
        Byte = 1,
        /// TAG_Short
        Short = 2,
        /// TAG_Int
        Int = 3,
        /// TAG_Long
        Long = 4,
        /// TAG_Float
        Float = 5,
        /// TAG_Double
        Double = 6,
        /// TAG_ByteArray
        ByteArray = 7,
        /// TAG_String
        String = 8,
        /// TAG_List
        List = 9,
        /// TAG_Compound
        Compound = 10,
        /// TAG_IntArray
        IntArray = 11,
        /// TAG_LongArray
        LongArray = 12,
        /// TAG_FloatArray
        FloatArray = 13,
        /// TAG_DoubleArray
        DoubleArray = 14,
        /// TAG_UUID
        Uuid = 15
    }
}

impl Serializeable for TagType {
    fn serialize<W: crate::data::DataOutput + ?Sized>(
        &self,
        output: &mut W,
    ) -> std::io::Result<()> {
        self.0.serialize(output)
    }
}

impl Deserializeable for TagType {
    fn deserialize<R: crate::data::DataInput + ?Sized>(
        &mut self,
        input: &mut R,
    ) -> std::io::Result<()> {
        self.0.deserialize(input)
    }
}

impl DeserializeCopy for TagType {
    fn deserialize_copy<R: crate::data::DataInput + ?Sized>(
        input: &mut R,
    ) -> std::io::Result<Self> {
        Ok(Self(DeserializeCopy::deserialize_copy(input)?))
    }
}

///
/// Types for Tag_List
pub mod list {
    use std::{fmt::Display, io::ErrorKind};

    use crate::data::{DeserializeCopy, Deserializeable, OutOfRange, Serializeable};

    use super::{NbtTag, TagType};

    ///
    /// A homogenous list of NBT Tags.
    /// Each element in the List has the same tag
    #[derive(Clone, Default, Debug)]
    pub struct NbtList {
        tag: TagType,
        elements: Vec<NbtTag>,
    }

    ///
    /// The Error returned when adding a tag with an invalid type
    #[derive(Debug, Clone)]
    pub struct WrongTagType {
        tag: NbtTag,
        expected: TagType,
    }

    impl Display for WrongTagType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "Invalid tag with type {:?} (list elements have type {:?})",
                &self.tag, &self.expected
            ))
        }
    }

    impl WrongTagType {
        ///
        /// Unwraps the tag that was attempted to be inserted
        pub fn into_tag(self) -> NbtTag {
            self.tag
        }

        ///
        /// Gets the type of tags that can be inserted into the list
        pub fn expected_tag(&self) -> TagType {
            self.expected
        }
    }

    impl std::error::Error for WrongTagType {}

    impl NbtList {
        ///
        /// Creates a new NbtList
        pub const fn new() -> Self {
            Self {
                tag: TagType::End,
                elements: Vec::new(),
            }
        }

        ///
        /// Attempts to insert tag into
        pub fn insert(&mut self, tag: NbtTag) -> Result<(), WrongTagType> {
            if self.elements.is_empty() {
                self.tag = tag.tag_type();
                self.elements.push(tag);
                Ok(())
            } else if self.tag == tag.tag_type() {
                self.elements.push(tag);
                Ok(())
            } else {
                Err(WrongTagType {
                    tag,
                    expected: self.tag,
                })
            }
        }
    }

    impl Serializeable for NbtList {
        fn serialize<W: crate::data::DataOutput + ?Sized>(
            &self,
            output: &mut W,
        ) -> std::io::Result<()> {
            self.tag.serialize(output)?;
            let len = self.elements.len();
            if len > (i32::MAX as usize) {
                return Err(std::io::Error::new(ErrorKind::InvalidData, OutOfRange(len)));
            }
            (len as i32).serialize(output)?;
            for t in &self.elements {
                t.serialize(output)?;
            }
            Ok(())
        }
    }
    impl Deserializeable for NbtList {
        fn deserialize<R: crate::data::DataInput + ?Sized>(
            &mut self,
            input: &mut R,
        ) -> std::io::Result<()> {
            let tt = TagType::deserialize_copy(input)?;
            let len = i32::deserialize_copy(input)?;
            if len < 0 {
                return Err(std::io::Error::new(ErrorKind::InvalidData, OutOfRange(len)));
            }
            let mut items = Vec::with_capacity(len as usize);
            items.resize_with(len as usize, || NbtTag::default_for_tag(tt));
            for i in &mut items {
                i.deserialize(input)?;
            }
            self.tag = tt;
            self.elements = items;
            Ok(())
        }
    }
    impl DeserializeCopy for NbtList {
        fn deserialize_copy<R: crate::data::DataInput + ?Sized>(
            input: &mut R,
        ) -> std::io::Result<Self> {
            let tt = TagType::deserialize_copy(input)?;
            let len = i32::deserialize_copy(input)?;
            if len < 0 {
                return Err(std::io::Error::new(ErrorKind::InvalidData, OutOfRange(len)));
            }
            let mut items = Vec::with_capacity(len as usize);
            items.resize_with(len as usize, || NbtTag::default_for_tag(tt));
            for i in &mut items {
                i.deserialize(input)?;
            }
            Ok(Self {
                tag: tt,
                elements: items,
            })
        }
    }
}

///
/// Types for Tag_Compound
pub mod compound {
    use std::{collections::HashMap, io::ErrorKind, ops::Index};

    use crate::data::{DeserializeCopy, Deserializeable, Serializeable};

    use super::{NbtTag, TagType};

    ///
    /// A Compound NBT Tag, containing multiple, named, unordered, NBT Tags
    #[derive(Clone, Debug, Default)]
    pub struct NbtCompound {
        inner: HashMap<String, NbtTag>,
    }

    impl NbtCompound {
        ///
        /// Creates a new, empty, Compound
        pub fn new() -> NbtCompound {
            Self {
                inner: HashMap::new(),
            }
        }

        ///
        /// Gets the element of the Compound with the given Name
        pub fn get<S: AsRef<str> + ?Sized>(&self, st: &S) -> Option<&NbtTag> {
            self.inner.get(st.as_ref())
        }

        ///
        /// Gets a mutable reference to the element of the Compound with the given Name
        pub fn get_mut<S: AsRef<str> + ?Sized>(&mut self, st: &S) -> Option<&mut NbtTag> {
            self.inner.get_mut(st.as_ref())
        }

        ///
        /// Inserts a tag into the Compound if the name is unique, otherwise returns the value.
        pub fn insert(&mut self, name: String, value: NbtTag) -> Option<NbtTag> {
            self.inner.insert(name, value)
        }
    }

    impl<S: AsRef<str>> Index<S> for NbtCompound {
        type Output = NbtTag;

        fn index(&self, index: S) -> &Self::Output {
            &self.inner[index.as_ref()]
        }
    }

    impl Serializeable for NbtCompound {
        fn serialize<W: crate::data::DataOutput + ?Sized>(
            &self,
            output: &mut W,
        ) -> std::io::Result<()> {
            for (k, v) in &self.inner {
                let ty = v.tag_type();
                if TagType::End == ty {
                    return Err(std::io::Error::new(
                        ErrorKind::InvalidData,
                        "Embedded Tag Ends cannot be serialized",
                    ));
                }
                ty.serialize(output)?;
                k.serialize(output)?;
                v.serialize(output)?;
            }
            TagType::End.serialize(output)
        }
    }

    impl Deserializeable for NbtCompound {
        fn deserialize<R: crate::data::DataInput + ?Sized>(
            &mut self,
            input: &mut R,
        ) -> std::io::Result<()> {
            self.inner.clear();
            loop {
                let ty = TagType::deserialize_copy(input)?;
                if ty == TagType::End {
                    return Ok(());
                }
                let name = String::deserialize_copy(input)?;
                let mut tag = NbtTag::default_for_tag(ty);
                tag.deserialize(input)?;
                self.inner.insert(name, tag);
            }
        }
    }

    impl DeserializeCopy for NbtCompound {
        fn deserialize_copy<R: crate::data::DataInput + ?Sized>(
            input: &mut R,
        ) -> std::io::Result<Self> {
            let mut inner = HashMap::new();
            loop {
                let ty = TagType::deserialize_copy(input)?;
                if ty == TagType::End {
                    return Ok(Self { inner });
                }
                let name = String::deserialize_copy(input)?;
                let mut tag = NbtTag::default_for_tag(ty);
                tag.deserialize(input)?;
                inner.insert(name, tag);
            }
        }
    }
}

///
/// An NBT Tag
#[derive(Clone, Debug)]
pub enum NbtTag {
    ///
    /// The end Tag
    /// Appears at the end of each Compound tag
    End,
    ///
    /// A single byte
    Byte(u8),
    ///
    /// A single short
    Short(i16),
    ///
    /// A single int
    Int(i32),
    ///
    /// A single long
    Long(i64),
    ///
    /// A single float
    Float(f32),
    ///
    /// A single double
    Double(f64),
    ///
    /// An array of bytes
    ByteArray(array::NbtArray<u8>),
    ///
    /// A string
    String(String),
    ///
    /// A list of Tags of the same type
    List(list::NbtList),
    ///
    /// A compound tag
    Compound(compound::NbtCompound),
    ///
    /// An array of ints
    IntArray(array::NbtArray<i32>),
    ///
    /// An array of longs
    LongArray(array::NbtArray<i64>),
    ///
    /// An array of floats
    FloatArray(array::NbtArray<f32>),
    ///
    /// An array of doubles
    DoubleArray(array::NbtArray<f64>),
    ///
    /// A UUID
    Uuid(crate::uuid::UUID),
}

impl NbtTag {
    ///
    /// Gets the type of the tag
    pub fn tag_type(&self) -> TagType {
        match self {
            Self::End => TagType::End,
            Self::Byte(_) => TagType::Byte,
            NbtTag::Short(_) => TagType::Short,
            NbtTag::Int(_) => TagType::Int,
            NbtTag::Long(_) => TagType::Long,
            NbtTag::Float(_) => TagType::Float,
            NbtTag::Double(_) => TagType::Double,
            NbtTag::ByteArray(_) => TagType::ByteArray,
            NbtTag::String(_) => TagType::String,
            NbtTag::List(_) => TagType::List,
            NbtTag::Compound(_) => TagType::Compound,
            NbtTag::IntArray(_) => TagType::IntArray,
            NbtTag::LongArray(_) => TagType::LongArray,
            NbtTag::FloatArray(_) => TagType::FloatArray,
            NbtTag::DoubleArray(_) => TagType::DoubleArray,
            NbtTag::Uuid(_) => TagType::Uuid,
        }
    }

    ///
    /// Returns a default (empty) value for the given tag type
    ///
    /// Panics if TagType is not a valid tag
    pub fn default_for_tag(ty: TagType) -> Self {
        match ty {
            TagType::End => Self::End,
            TagType::Byte => Self::Byte(0),
            TagType::Short => Self::Short(0),
            TagType::Int => Self::Int(0),
            TagType::Float => Self::Float(0.0),
            TagType::Double => Self::Double(0.0),
            TagType::ByteArray => Self::ByteArray(Default::default()),
            TagType::String => Self::String(Default::default()),
            TagType::List => Self::List(Default::default()),
            TagType::Compound => Self::Compound(Default::default()),
            TagType::IntArray => Self::IntArray(Default::default()),
            TagType::LongArray => Self::LongArray(Default::default()),
            TagType::FloatArray => Self::FloatArray(Default::default()),
            TagType::DoubleArray => Self::DoubleArray(Default::default()),
            TagType::Uuid => Self::Uuid(Default::default()),
            _ => panic!("Invalid tag type"),
        }
    }
}

impl Serializeable for NbtTag {
    fn serialize<W: crate::data::DataOutput + ?Sized>(
        &self,
        output: &mut W,
    ) -> std::io::Result<()> {
        match self {
            NbtTag::End => Ok(()),
            NbtTag::Byte(v) => v.serialize(output),
            NbtTag::Short(v) => v.serialize(output),
            NbtTag::Int(v) => v.serialize(output),
            NbtTag::Long(v) => v.serialize(output),
            NbtTag::Float(v) => v.serialize(output),
            NbtTag::Double(v) => v.serialize(output),
            NbtTag::ByteArray(v) => v.serialize(output),
            NbtTag::String(v) => v.serialize(output),
            NbtTag::List(v) => v.serialize(output),
            NbtTag::Compound(v) => v.serialize(output),
            NbtTag::IntArray(v) => v.serialize(output),
            NbtTag::LongArray(v) => v.serialize(output),
            NbtTag::FloatArray(v) => v.serialize(output),
            NbtTag::DoubleArray(v) => v.serialize(output),
            NbtTag::Uuid(v) => v.serialize(output),
        }
    }
}

impl Deserializeable for NbtTag {
    fn deserialize<W: crate::data::DataInput + ?Sized>(
        &mut self,
        output: &mut W,
    ) -> std::io::Result<()> {
        match self {
            NbtTag::End => Ok(()),
            NbtTag::Byte(v) => v.deserialize(output),
            NbtTag::Short(v) => v.deserialize(output),
            NbtTag::Int(v) => v.deserialize(output),
            NbtTag::Long(v) => v.deserialize(output),
            NbtTag::Float(v) => v.deserialize(output),
            NbtTag::Double(v) => v.deserialize(output),
            NbtTag::ByteArray(v) => v.deserialize(output),
            NbtTag::String(v) => v.deserialize(output),
            NbtTag::List(v) => v.deserialize(output),
            NbtTag::Compound(v) => v.deserialize(output),
            NbtTag::IntArray(v) => v.deserialize(output),
            NbtTag::LongArray(v) => v.deserialize(output),
            NbtTag::FloatArray(v) => v.deserialize(output),
            NbtTag::DoubleArray(v) => v.deserialize(output),
            NbtTag::Uuid(v) => v.deserialize(output),
        }
    }
}
