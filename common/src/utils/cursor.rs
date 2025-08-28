use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize)]
pub enum CursorError {
    #[error("Unexpected end of data")]
    UnexpectedEof,
}
pub struct ByteCursor<'a> {
    it: std::iter::Copied<std::slice::Iter<'a, u8>>,
}

impl<'a> ByteCursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            it: data.iter().copied(),
        }
    }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], CursorError> {
        let mut buf = [0u8; N];
        for b in &mut buf {
            *b = self.it.next().ok_or(CursorError::UnexpectedEof)?;
        }
        Ok(buf)
    }

    pub fn u8(&mut self) -> Result<u8, CursorError> {
        self.it.next().ok_or(CursorError::UnexpectedEof)
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
        self.it.next()
    }

    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>, CursorError> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.it.next().ok_or(CursorError::UnexpectedEof)?);
        }
        Ok(v)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), CursorError> {
        for byte in buf {
            *byte = self.it.next().ok_or(CursorError::UnexpectedEof)?;
        }
        Ok(())
    }
}
