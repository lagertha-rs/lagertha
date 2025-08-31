use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize)]
pub enum CursorError {
    #[error("Unexpected end of data")]
    UnexpectedEof,
}
pub struct ByteCursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteCursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], CursorError> {
        if self.remaining() < N {
            return Err(CursorError::UnexpectedEof);
        }
        let mut buf = [0u8; N];
        buf.copy_from_slice(&self.data[self.pos..self.pos + N]);
        self.pos += N;
        Ok(buf)
    }

    pub fn u8(&mut self) -> Result<u8, CursorError> {
        if self.remaining() < 1 {
            return Err(CursorError::UnexpectedEof);
        }
        let b = self.data[self.pos];
        self.pos += 1;
        Ok(b)
    }
    pub fn i8(&mut self) -> Result<i8, CursorError> {
        Ok(self.u8()? as i8)
    }
    pub fn u16(&mut self) -> Result<u16, CursorError> {
        Ok(u16::from_be_bytes(self.take::<2>()?))
    }
    pub fn i16(&mut self) -> Result<i16, CursorError> {
        Ok(i16::from_be_bytes(self.take::<2>()?))
    }
    pub fn u32(&mut self) -> Result<u32, CursorError> {
        Ok(u32::from_be_bytes(self.take::<4>()?))
    }
    pub fn i32(&mut self) -> Result<i32, CursorError> {
        Ok(i32::from_be_bytes(self.take::<4>()?))
    }
    pub fn u64(&mut self) -> Result<u64, CursorError> {
        Ok(u64::from_be_bytes(self.take::<8>()?))
    }
    pub fn i64(&mut self) -> Result<i64, CursorError> {
        Ok(i64::from_be_bytes(self.take::<8>()?))
    }

    pub fn try_u8(&mut self) -> Option<u8> {
        if self.remaining() == 0 {
            None
        } else {
            let b = self.data[self.pos];
            self.pos += 1;
            Some(b)
        }
    }

    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>, CursorError> {
        if self.remaining() < n {
            return Err(CursorError::UnexpectedEof);
        }
        let v = self.data[self.pos..self.pos + n].to_vec();
        self.pos += n;
        Ok(v)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), CursorError> {
        let n = buf.len();
        if self.remaining() < n {
            return Err(CursorError::UnexpectedEof);
        }
        buf.copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        Ok(())
    }

    pub fn align_to(&mut self, align: usize) -> Result<(), CursorError> {
        if align == 0 {
            return Ok(());
        }
        let mis = self.pos % align;
        if mis == 0 {
            return Ok(());
        }
        let skip = align - mis;
        if self.remaining() < skip {
            return Err(CursorError::UnexpectedEof);
        }
        self.pos += skip;
        Ok(())
    }
}
