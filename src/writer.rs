use std::{
    fs::File,
    io::{Cursor, Seek, Write},
};

use eyre::Result;

impl SafeWrite for Cursor<Vec<u8>> {}

impl SafeWrite for Cursor<&mut Vec<u8>> {}

impl SafeWrite for Cursor<&mut [u8]> {}

impl SafeWrite for &mut File {}

impl SafeWrite for File {}

pub trait SafeWrite: Write + Seek {
    fn tell(&mut self) -> Result<u64> {
        let pos = self.stream_position()?;
        Ok(pos)
    }

    fn write_i8(&mut self, val: i8) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u8(&mut self, val: u8) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i16_be(&mut self, val: i16) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i16_le(&mut self, val: i16) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u16_be(&mut self, val: u16) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u16_le(&mut self, val: u16) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i32_be(&mut self, val: i32) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i32_le(&mut self, val: i32) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u32_be(&mut self, val: u32) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u32_le(&mut self, val: u32) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i64_be(&mut self, val: i64) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i64_le(&mut self, val: i64) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u64_be(&mut self, val: u64) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u64_le(&mut self, val: u64) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i128_be(&mut self, val: i128) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_i128_le(&mut self, val: i128) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u128_be(&mut self, val: u128) -> Result<()> {
        let bytes = val.to_be_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }
    fn write_u128_le(&mut self, val: u128) -> Result<()> {
        let bytes = val.to_le_bytes();
        self.write_all(&bytes)?;
        Ok(())
    }

    fn write_exact(&mut self, val: &[u8]) -> Result<()> {
        self.write_all(val)?;
        Ok(())
    }
}
