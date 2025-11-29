#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorError {
    UnexpectedEof,
}

pub struct ByteCursor<'a> {
    data: &'a [u8],
    pos: usize,
    order: ByteOrder,
}

impl<'a> ByteCursor<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            order: ByteOrder::BigEndian,
        }
    }

    #[inline]
    pub fn with_order(data: &'a [u8], order: ByteOrder) -> Self {
        Self {
            data,
            pos: 0,
            order,
        }
    }

    #[inline]
    pub fn order(&self) -> ByteOrder {
        self.order
    }
    #[inline]
    pub fn position(&self) -> usize {
        self.pos
    }
    #[inline]
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    #[inline]
    pub fn skip(&mut self, n: usize) -> Result<(), CursorError> {
        if self.remaining() < n {
            return Err(CursorError::UnexpectedEof);
        }
        self.pos += n;
        Ok(())
    }

    #[inline]
    pub fn align(&mut self, align: usize) -> Result<(), CursorError> {
        let mis = self.pos % align;
        if mis == 0 {
            return Ok(());
        }
        self.skip(align - mis)
    }

    #[inline]
    pub fn slice(&mut self, n: usize) -> Result<&'a [u8], CursorError> {
        if self.remaining() < n {
            return Err(CursorError::UnexpectedEof);
        }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    #[inline]
    pub fn bytes(&mut self, n: usize) -> Result<&[u8], CursorError> {
        self.slice(n)
    }

    #[inline]
    fn take<const N: usize>(&mut self) -> Result<&'a [u8; N], CursorError> {
        if self.remaining() < N {
            return Err(CursorError::UnexpectedEof);
        }
        let p = self.pos;
        self.pos += N;
        let ptr = &self.data[p] as *const u8 as *const [u8; N];
        Ok(unsafe { &*ptr })
    }

    #[inline]
    pub fn u8(&mut self) -> Result<u8, CursorError> {
        self.take::<1>().map(|b| b[0])
    }
    #[inline]
    pub fn i8(&mut self) -> Result<i8, CursorError> {
        self.u8().map(|v| v as i8)
    }

    #[inline]
    pub fn u16(&mut self) -> Result<u16, CursorError> {
        let b = self.take::<2>()?;
        Ok(match self.order {
            ByteOrder::BigEndian => u16::from_be_bytes(*b),
            ByteOrder::LittleEndian => u16::from_le_bytes(*b),
        })
    }

    #[inline]
    pub fn i16(&mut self) -> Result<i16, CursorError> {
        self.u16().map(|v| v as i16)
    }

    #[inline]
    pub fn u32(&mut self) -> Result<u32, CursorError> {
        let b = self.take::<4>()?;
        Ok(match self.order {
            ByteOrder::BigEndian => u32::from_be_bytes(*b),
            ByteOrder::LittleEndian => u32::from_le_bytes(*b),
        })
    }

    #[inline]
    pub fn i32(&mut self) -> Result<i32, CursorError> {
        self.u32().map(|v| v as i32)
    }

    #[inline]
    pub fn u64(&mut self) -> Result<u64, CursorError> {
        let b = self.take::<8>()?;
        Ok(match self.order {
            ByteOrder::BigEndian => u64::from_be_bytes(*b),
            ByteOrder::LittleEndian => u64::from_le_bytes(*b),
        })
    }

    #[inline]
    pub fn i64(&mut self) -> Result<i64, CursorError> {
        self.u64().map(|v| v as i64)
    }

    #[inline]
    pub fn f32(&mut self) -> Result<f32, CursorError> {
        self.u32().map(f32::from_bits)
    }
    #[inline]
    pub fn f64(&mut self) -> Result<f64, CursorError> {
        self.u64().map(f64::from_bits)
    }

    #[inline]
    pub fn try_u8(&mut self) -> Option<u8> {
        if self.remaining() == 0 {
            None
        } else {
            let b = self.data[self.pos];
            self.pos += 1;
            Some(b)
        }
    }

    #[inline]
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), CursorError> {
        let n = buf.len();
        if self.remaining() < n {
            return Err(CursorError::UnexpectedEof);
        }

        buf.copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        Ok(())
    }
}
