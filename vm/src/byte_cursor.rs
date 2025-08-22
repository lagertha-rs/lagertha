use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
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

    fn take<const N: usize>(it: &mut impl Iterator<Item = u8>) -> Option<[u8; N]> {
        let mut buf = [0u8; N];
        for b in &mut buf {
            *b = it.next()?;
        }
        Some(buf)
    }

    pub fn u32(&mut self) -> Result<u32, CursorError> {
        ByteCursor::take::<4>(&mut self.it)
            .map(u32::from_be_bytes)
            .ok_or(CursorError::UnexpectedEof)
    }

    pub fn i32(&mut self) -> Result<i32, CursorError> {
        ByteCursor::take::<4>(&mut self.it)
            .map(i32::from_be_bytes)
            .ok_or(CursorError::UnexpectedEof)
    }

    pub fn i64(&mut self) -> Result<i64, CursorError> {
        ByteCursor::take::<8>(&mut self.it)
            .map(i64::from_be_bytes)
            .ok_or(CursorError::UnexpectedEof)
    }

    pub fn u16(&mut self) -> Result<u16, CursorError> {
        ByteCursor::take::<2>(&mut self.it)
            .map(u16::from_be_bytes)
            .ok_or(CursorError::UnexpectedEof)
    }

    pub fn i16(&mut self) -> Result<i16, CursorError> {
        ByteCursor::take::<2>(&mut self.it)
            .map(i16::from_be_bytes)
            .ok_or(CursorError::UnexpectedEof)
    }

    pub fn u8(&mut self) -> Result<u8, CursorError> {
        self.it.next().ok_or(CursorError::UnexpectedEof)
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
