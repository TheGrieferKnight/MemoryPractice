use std::io::{self, Read};

pub struct CursorReader<R> {
    pub inner: R,
    pub offset: u64,
}

impl<R: Read> CursorReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, offset: 0 }
    }

    pub fn position(&self) -> u64 {
        self.offset
    }

    pub fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        self.inner.read_exact(buffer)?;
        self.offset += buffer.len() as u64;
        Ok(())
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