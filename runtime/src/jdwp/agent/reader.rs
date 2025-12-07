use crate::jdwp::agent::error_code::JdwpError;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub struct PacketReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> PacketReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, JdwpError> {
        Ok(self.cursor.read_u8()?)
    }

    pub fn read_i8(&mut self) -> Result<i8, JdwpError> {
        Ok(self.cursor.read_i8()?)
    }

    pub fn read_u16(&mut self) -> Result<u16, JdwpError> {
        Ok(self.cursor.read_u16::<BigEndian>()?)
    }

    pub fn read_i16(&mut self) -> Result<i16, JdwpError> {
        Ok(self.cursor.read_i16::<BigEndian>()?)
    }

    pub fn read_u32(&mut self) -> Result<u32, JdwpError> {
        Ok(self.cursor.read_u32::<BigEndian>()?)
    }

    pub fn read_i32(&mut self) -> Result<i32, JdwpError> {
        Ok(self.cursor.read_i32::<BigEndian>()?)
    }

    pub fn read_u64(&mut self) -> Result<u64, JdwpError> {
        Ok(self.cursor.read_u64::<BigEndian>()?)
    }

    pub fn read_i64(&mut self) -> Result<i64, JdwpError> {
        Ok(self.cursor.read_i64::<BigEndian>()?)
    }

    pub fn read_f32(&mut self) -> Result<f32, JdwpError> {
        Ok(self.cursor.read_f32::<BigEndian>()?)
    }

    pub fn read_f64(&mut self) -> Result<f64, JdwpError> {
        Ok(self.cursor.read_f64::<BigEndian>()?)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, JdwpError> {
        let mut buf = vec![0u8; len];
        self.cursor.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn remaining(&self) -> usize {
        let pos = self.cursor.position() as usize;
        self.cursor.get_ref().len() - pos
    }

    pub fn is_empty(&self) -> bool {
        self.remaining() == 0
    }
}
