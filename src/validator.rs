use std::fmt;

use eyre::{eyre, Result};

pub trait Validator: Sized + Ord + Eq + fmt::Display + fmt::Debug + fmt::UpperHex {
    fn validate_equals(self, val: Self) -> Result<Self> {
        if self.eq(&val) {
            Ok(self)
        } else {
            Err(eyre!(
                "value 0x{:X} not equal to expected 0x{:X}",
                &self,
                val
            ))
        }
    }

    fn validate_not_equals(self, val: Self) -> Result<Self> {
        if self.ne(&val) {
            Ok(self)
        } else {
            Err(eyre!("value is unexpectedly equal to 0x{:X}", val))
        }
    }

    fn validate_in_range<R>(self, range: R) -> Result<Self>
    where
        R: std::ops::RangeBounds<Self> + fmt::Debug,
    {
        if range.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value 0x{:X} not in expected range {:X?}",
                &self,
                range
            ))
        }
    }

    fn validate_not_in_range<R>(self, range: R) -> Result<Self>
    where
        R: std::ops::RangeBounds<Self> + fmt::Debug,
    {
        if !range.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value 0x{:X} unexpectedly is in range {:X?}",
                &self,
                range
            ))
        }
    }

    fn validate_in_list(self, list: &[Self]) -> Result<Self> {
        if list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value 0x{:X} is not on expected list {:X?}",
                &self,
                list
            ))
        }
    }
    fn validate_not_in_list(self, list: &[Self]) -> Result<Self> {
        if !list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value 0x{:X} is unexpectedly on list {:X?}",
                &self,
                list
            ))
        }
    }

    fn validate_in_array<const N: usize>(self, list: [Self; N]) -> Result<Self> {
        if list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value 0x{:X} is not on expected list {:X?}",
                &self,
                list
            ))
        }
    }

    fn validate_not_in_array<const N: usize>(self, list: [Self; N]) -> Result<Self> {
        if !list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value 0x{:X} is unexpectedly on list {:X?}",
                &self,
                list
            ))
        }
    }

    fn validate<P: FnOnce(&Self) -> bool>(self, predicate: P) -> Result<Self> {
        if predicate(&self) {
            Ok(self)
        } else {
            Err(eyre!("value 0x{:X} failed test", &self))
        }
    }

    fn validate_not<P: FnOnce(&Self) -> bool>(self, predicate: P) -> Result<Self> {
        if !predicate(&self) {
            Ok(self)
        } else {
            Err(eyre!("value 0x{:X} unexpectedly succeeded test", &self))
        }
    }

    fn validate_equals_ctx(self, val: Self, msg: &str) -> Result<Self> {
        if self.eq(&val) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} not equal to expected 0x{:X}",
                &self,
                val
            ))
        }
    }

    fn validate_not_equals_ctx(self, val: Self, msg: &str) -> Result<Self> {
        if self.ne(&val) {
            Ok(self)
        } else {
            Err(eyre!("value {msg} is unexpectedly equal to 0x{:X}", val))
        }
    }

    fn validate_in_range_ctx<R>(self, range: R, msg: &str) -> Result<Self>
    where
        R: std::ops::RangeBounds<Self> + fmt::Debug,
    {
        if range.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} not in expected range {:X?}",
                &self,
                range
            ))
        }
    }

    fn validate_not_in_range_ctx<R>(self, range: R, msg: &str) -> Result<Self>
    where
        R: std::ops::RangeBounds<Self> + fmt::Debug,
    {
        if !range.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} unexpectedly is in range {:X?}",
                &self,
                range
            ))
        }
    }

    fn validate_in_list_ctx(self, list: &[Self], msg: &str) -> Result<Self> {
        if list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} is not on expected list {:X?}",
                &self,
                list
            ))
        }
    }
    fn validate_not_in_list_ctx(self, list: &[Self], msg: &str) -> Result<Self> {
        if !list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} is unexpectedly on list {:X?}",
                &self,
                list
            ))
        }
    }

    fn validate_in_array_ctx<const N: usize>(self, list: [Self; N], msg: &str) -> Result<Self> {
        if list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} is not on expected list {:X?}",
                &self,
                list
            ))
        }
    }

    fn validate_not_in_array_ctx<const N: usize>(self, list: [Self; N], msg: &str) -> Result<Self> {
        if !list.contains(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} is unexpectedly on list {:X?}",
                &self,
                list
            ))
        }
    }

    fn validate_ctx<P: FnOnce(&Self) -> bool>(self, predicate: P, msg: &str) -> Result<Self> {
        if predicate(&self) {
            Ok(self)
        } else {
            Err(eyre!("value {msg}=0x{:X} failed test", &self))
        }
    }

    fn validate_not_ctx<P: FnOnce(&Self) -> bool>(self, predicate: P, msg: &str) -> Result<Self> {
        if !predicate(&self) {
            Ok(self)
        } else {
            Err(eyre!(
                "value {msg}=0x{:X} unexpectedly succeeded test",
                &self
            ))
        }
    }
}

impl Validator for u8 {}

impl Validator for i8 {}

impl Validator for u16 {}

impl Validator for i16 {}

impl Validator for u32 {}

impl Validator for i32 {}

impl Validator for u64 {}

impl Validator for i64 {}

impl Validator for u128 {}

impl Validator for i128 {}

pub trait Validate
where
    Self: Sized,
{
    fn validate(self) -> Result<Self> { Ok(self) }
}
