use crate::JvmError;
use crate::rt::constant_pool::RuntimeConstantPool;
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct ThreadStack {
    max_size: usize,
    frames: RefCell<Vec<Frame>>,
}

impl ThreadStack {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            frames: RefCell::new(Vec::with_capacity(max_size)),
        }
    }

    pub fn push_frame(&self, frame: Frame) -> Result<(), JvmError> {
        let mut frames = self.frames.borrow_mut();
        if frames.len() >= self.max_size {
            return Err(JvmError::StackOverflow);
        }
        frames.push(frame);
        Ok(())
    }

    pub fn pop_frame(&self) -> Result<Frame, JvmError> {
        let mut frames = self.frames.borrow_mut();
        frames.pop().ok_or(JvmError::StackIsEmpty)
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.6
#[derive(Debug)]
pub struct Frame {
    locals: Vec<()>,
    operands: Vec<()>,
    cp: Arc<RuntimeConstantPool>,
}

impl Frame {
    pub fn new(cp: Arc<RuntimeConstantPool>, max_locals: usize, max_stack: usize) -> Self {
        Self {
            locals: vec![(); max_locals],
            operands: Vec::with_capacity(max_stack),
            cp,
        }
    }
}
