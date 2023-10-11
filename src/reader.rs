use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
};

use eyre::{eyre, Result};

impl SafeRead for Cursor<Vec<u8>> {}

impl SafeRead for Cursor<&Vec<u8>> {}

impl SafeRead for Cursor<&[u8]> {}

impl SafeRead for &mut File {}

impl SafeRead for File {}

#[allow(clippy::len_without_is_empty)]
pub trait SafeRead: Read + Seek {
    fn tell(&mut self) -> Result<u64> {
        let pos = self.stream_position()?;
        Ok(pos)
    }

    fn len(&mut self) -> Result<usize> {
        let here = self.tell()?;
        let end_pos = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(here))?;
        Ok(end_pos as usize)
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(buf.as_mut())?;
        Ok(u8::from_be_bytes(buf))
    }

    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0u8; 1];
        self.read_exact(buf.as_mut())?;
        Ok(i8::from_be_bytes(buf))
    }

    fn read_u16be(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(buf.as_mut())?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_u16le(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(buf.as_mut())?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_i16be(&mut self) -> Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(buf.as_mut())?;
        Ok(i16::from_be_bytes(buf))
    }

    fn read_i16le(&mut self) -> Result<i16> {
        let mut buf = [0u8; 2];
        self.read_exact(buf.as_mut())?;
        Ok(i16::from_le_bytes(buf))
    }

    fn read_u32be(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(buf.as_mut())?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_u32le(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(buf.as_mut())?;
        Ok(u32::from_le_bytes(buf))
    }

    fn read_i32be(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(buf.as_mut())?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_i32le(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.read_exact(buf.as_mut())?;
        Ok(i32::from_le_bytes(buf))
    }

    fn read_u64be(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(buf.as_mut())?;
        Ok(u64::from_be_bytes(buf))
    }

    fn read_u64le(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(buf.as_mut())?;
        Ok(u64::from_le_bytes(buf))
    }

    fn read_i64be(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(buf.as_mut())?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_i64le(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.read_exact(buf.as_mut())?;
        Ok(i64::from_le_bytes(buf))
    }

    fn read_u128be(&mut self) -> Result<u128> {
        let mut buf = [0u8; 16];
        self.read_exact(buf.as_mut())?;
        Ok(u128::from_be_bytes(buf))
    }

    fn read_u128le(&mut self) -> Result<u128> {
        let mut buf = [0u8; 16];
        self.read_exact(buf.as_mut())?;
        Ok(u128::from_le_bytes(buf))
    }

    fn read_i128be(&mut self) -> Result<i128> {
        let mut buf = [0u8; 16];
        self.read_exact(buf.as_mut())?;
        Ok(i128::from_be_bytes(buf))
    }

    fn read_i128le(&mut self) -> Result<i128> {
        let mut buf = [0u8; 16];
        self.read_exact(buf.as_mut())?;
        Ok(i128::from_le_bytes(buf))
    }

    fn read_into(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)?;
        Ok(())
    }

    fn read_as_vec(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_until(&mut self, end: u8) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        loop {
            match self.read_u8() {
                Ok(v) if v == end => return Ok(buf),
                Ok(v) => buf.push(v),
                Err(e) => return Err(e),
            }
        }
    }

    fn read_asciiz(&mut self) -> Result<String> {
        match self.read_until(0) {
            Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
            Err(e) => Err(e),
        }
    }

    fn read_leb128(&mut self) -> Result<u128> {
        let mut res = 0_u128;
        let mut cnt = 0;

        while cnt <= 128 {
            let byte = self.read_u8()?;

            let last = (byte & 0x80) == 0;
            let data = (byte & 0x7F) as u128;

            res = res << 7 | data;

            if last {
                return Ok(res);
            }
            cnt += 7;
        }
        Err(eyre!("read_leb128 overflow - no end byte after {cnt} bits"))
    }

    fn read_vlq128(&mut self) -> Result<u128> {
        let mut res = 0_u128;
        let mut cnt = 0;

        while cnt <= 128 {
            let byte = self.read_u8()?;

            let last = (byte & 0x80) == 0;
            let data = (byte & 0x7F) as u128;

            res |= data << cnt;

            if last {
                return Ok(res);
            }
            cnt += 7;
        }
        Err(eyre!("read_vlq128 overflow - no end byte after {cnt} bits"))
    }
}
