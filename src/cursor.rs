#![allow(dead_code)]

use std::io::{self, Read};

pub struct CursorReader<R> {
    inner: R,
    offset: u64,
}

impl<R: Read> CursorReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, offset: 0 }
    }

    pub fn position(&self) -> u64 {
        self.offset
    }

    pub fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        let start = self.offset;
        let requested = buffer.len();

        match self.inner.read_exact(buffer) {
            Ok(()) => {
                self.offset += requested as u64;
                Ok(())
            }
            Err(error) => Err(io::Error::new(
                error.kind(),
                format!(
                    "read_exact failed at offset {}, requested {} bytes: {}",
                    start, requested, error
                ),
            )),
        }
    }

    pub fn read_u128_le(&mut self) -> io::Result<u128> {
        let mut bytes = [0u8; 16];
        self.read_exact(&mut bytes)?;
        Ok(u128::from_le_bytes(bytes))
    }

    pub fn read_u64_le(&mut self) -> io::Result<u64> {
        let mut bytes = [0u8; 8];
        self.read_exact(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    pub fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_u16_le(&mut self) -> io::Result<u16> {
        let mut bytes = [0u8; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn read_u8_le(&mut self) -> io::Result<u8> {
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)?;
        Ok(u8::from_le_bytes(bytes))
    }

    pub fn read_u128_be(&mut self) -> io::Result<u128> {
        let mut bytes = [0u8; 16];
        self.read_exact(&mut bytes)?;
        Ok(u128::from_be_bytes(bytes))
    }

    pub fn read_u64_be(&mut self) -> io::Result<u64> {
        let mut bytes = [0u8; 8];
        self.read_exact(&mut bytes)?;
        Ok(u64::from_be_bytes(bytes))
    }

    pub fn read_u32_be(&mut self) -> io::Result<u32> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }

    pub fn read_u16_be(&mut self) -> io::Result<u16> {
        let mut bytes = [0u8; 2];
        self.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    pub fn read_u8_be(&mut self) -> io::Result<u8> {
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)?;
        Ok(u8::from_be_bytes(bytes))
    }

    pub fn read_i128_le(&mut self) -> io::Result<i128> {
        let mut bytes = [0u8; 16];
        self.read_exact(&mut bytes)?;
        Ok(i128::from_le_bytes(bytes))
    }

    pub fn read_i64_le(&mut self) -> io::Result<i64> {
        let mut bytes = [0u8; 8];
        self.read_exact(&mut bytes)?;
        Ok(i64::from_le_bytes(bytes))
    }

    pub fn read_i32_le(&mut self) -> io::Result<i32> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(i32::from_le_bytes(bytes))
    }

    pub fn read_i16_le(&mut self) -> io::Result<i16> {
        let mut bytes = [0u8; 2];
        self.read_exact(&mut bytes)?;
        Ok(i16::from_le_bytes(bytes))
    }

    pub fn read_i8_le(&mut self) -> io::Result<i8> {
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)?;
        Ok(i8::from_le_bytes(bytes))
    }

    pub fn read_i128_be(&mut self) -> io::Result<i128> {
        let mut bytes = [0u8; 16];
        self.read_exact(&mut bytes)?;
        Ok(i128::from_be_bytes(bytes))
    }

    pub fn read_i64_be(&mut self) -> io::Result<i64> {
        let mut bytes = [0u8; 8];
        self.read_exact(&mut bytes)?;
        Ok(i64::from_be_bytes(bytes))
    }

    pub fn read_i32_be(&mut self) -> io::Result<i32> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(i32::from_be_bytes(bytes))
    }

    pub fn read_i16_be(&mut self) -> io::Result<i16> {
        let mut bytes = [0u8; 2];
        self.read_exact(&mut bytes)?;
        Ok(i16::from_be_bytes(bytes))
    }

    pub fn read_i8_be(&mut self) -> io::Result<i8> {
        let mut bytes = [0u8; 1];
        self.read_exact(&mut bytes)?;
        Ok(i8::from_be_bytes(bytes))
    }

    pub fn read_f64_le(&mut self) -> io::Result<f64> {
        let mut bytes = [0u8; 8];
        self.read_exact(&mut bytes)?;
        Ok(f64::from_le_bytes(bytes))
    }
    pub fn read_f32_le(&mut self) -> io::Result<f32> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(f32::from_le_bytes(bytes))
    }

    pub fn read_f64_be(&mut self) -> io::Result<f64> {
        let mut bytes = [0u8; 8];
        self.read_exact(&mut bytes)?;
        Ok(f64::from_be_bytes(bytes))
    }
    pub fn read_f32_be(&mut self) -> io::Result<f32> {
        let mut bytes = [0u8; 4];
        self.read_exact(&mut bytes)?;
        Ok(f32::from_be_bytes(bytes))
    }

    pub fn read_utf16_le_to_string(&mut self, char_count: usize) -> io::Result<String> {
        let byte_count = char_count * 2;

        let mut bytes = vec![0u8; byte_count];
        self.read_exact(&mut bytes)?;

        let code_units: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        Ok(String::from_utf16_lossy(&code_units))
    }

    pub fn discard(&mut self, mut count: usize) -> io::Result<()> {
        let mut buffer = [0u8; 4096];

        while count > 0 {
            let amount = count.min(buffer.len());
            self.read_exact(&mut buffer[..amount])?;
            count -= amount;
        }

        Ok(())
    }
}
