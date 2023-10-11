use eyre::Result;

use super::{SafeRead, SafeWrite};

pub trait SafeReader<T>: SafeRead {
    fn read_auto(&mut self) -> Result<T>;
}

pub trait SafeWriter<T>: SafeWrite {
    fn write_auto(&mut self, val: T) -> Result<()>;
}

impl<SR> SafeReader<u8> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<u8> { self.read_u8() }
}

impl<SR> SafeReader<u16> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<u16> { self.read_u16be() }
}

impl<SR> SafeReader<u32> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<u32> { self.read_u32be() }
}

impl<SR> SafeReader<u64> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<u64> { self.read_u64be() }
}

impl<SR> SafeReader<u128> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<u128> { self.read_u128be() }
}

impl<SR> SafeReader<i8> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<i8> { self.read_i8() }
}

impl<SR> SafeReader<i16> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<i16> { self.read_i16be() }
}

impl<SR> SafeReader<i32> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<i32> { self.read_i32be() }
}

impl<SR> SafeReader<i64> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<i64> { self.read_i64be() }
}

impl<SR> SafeReader<i128> for SR
where
    SR: SafeRead,
{
    fn read_auto(&mut self) -> Result<i128> { self.read_i128be() }
}

impl<SW> SafeWriter<u8> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: u8) -> Result<()> { self.write_u8(val) }
}

impl<SW> SafeWriter<i8> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: i8) -> Result<()> { self.write_i8(val) }
}

impl<SW> SafeWriter<u16> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: u16) -> Result<()> { self.write_u16_be(val) }
}

impl<SW> SafeWriter<i16> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: i16) -> Result<()> { self.write_i16_be(val) }
}

impl<SW> SafeWriter<u32> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: u32) -> Result<()> { self.write_u32_be(val) }
}

impl<SW> SafeWriter<i32> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: i32) -> Result<()> { self.write_i32_be(val) }
}

impl<SW> SafeWriter<u64> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: u64) -> Result<()> { self.write_u64_be(val) }
}

impl<SW> SafeWriter<i64> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: i64) -> Result<()> { self.write_i64_be(val) }
}

impl<SW> SafeWriter<u128> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: u128) -> Result<()> { self.write_u128_be(val) }
}

impl<SW> SafeWriter<i128> for SW
where
    SW: SafeWrite,
{
    fn write_auto(&mut self, val: i128) -> Result<()> { self.write_i128_be(val) }
}
