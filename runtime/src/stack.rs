use crate::rt::constant_pool::RuntimeConstantPool;
use crate::{JvmError, VmConfig};
use common::jtype::Value;
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug)]
pub struct FrameStack {
    max_size: usize,
    max_operand_stack_size: usize,
    frames: RefCell<Vec<Frame>>,
}

impl FrameStack {
    pub fn new(vm_config: &VmConfig) -> Self {
        let max_size = vm_config.frame_stack_size;
        Self {
            max_size,
            max_operand_stack_size: vm_config.operand_stack_size,
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
        frames.pop().ok_or(JvmError::FrameStackIsEmpty)
    }

    pub fn cur_frame_get_local(&self, index: u8) -> Result<Value, JvmError> {
        let frames = self.frames.borrow();
        if let Some(frame) = frames.last() {
            frame.get_local(index)
        } else {
            Err(JvmError::FrameStackIsEmpty)
        }
    }

    pub fn cur_frame_push_operand(&self, value: Value) -> Result<(), JvmError> {
        let mut frames = self.frames.borrow_mut();
        if let Some(frame) = frames.last_mut() {
            if frame.operands.len() >= self.max_operand_stack_size {
                return Err(JvmError::StackOverflow);
            }
            frame.operands.push(value);
            Ok(())
        } else {
            Err(JvmError::OperandStackIsEmpty)
        }
    }

    pub fn cur_frame_pop_operand(&self) -> Result<Value, JvmError> {
        let mut frames = self.frames.borrow_mut();
        if let Some(frame) = frames.last_mut() {
            frame.operands.pop().ok_or(JvmError::OperandStackIsEmpty)
        } else {
            Err(JvmError::FrameStackIsEmpty)
        }
    }

    //TODO: arc clone is cheap, but probably not in each instruction execution
    pub fn cur_frame_cp(&self) -> Result<Arc<RuntimeConstantPool>, JvmError> {
        self.frames
            .borrow()
            .last()
            .map(|v| v.cp.clone())
            .ok_or(JvmError::FrameStackIsEmpty)
    }

    pub fn cur_frame_top_operand(&self) -> Result<Value, JvmError> {
        let frames = self.frames.borrow();
        if let Some(frame) = frames.last() {
            frame
                .operands
                .last()
                .cloned()
                .ok_or(JvmError::OperandStackIsEmpty)
        } else {
            Err(JvmError::FrameStackIsEmpty)
        }
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.6
#[derive(Debug)]
pub struct Frame {
    locals: Vec<Value>,
    operands: Vec<Value>,
    cp: Arc<RuntimeConstantPool>,
}

impl Frame {
    pub fn new(cp: Arc<RuntimeConstantPool>, locals: Vec<Value>, max_stack: usize) -> Self {
        Self {
            locals,
            operands: Vec::with_capacity(max_stack),
            cp,
        }
    }

    pub fn get_local(&self, index: u8) -> Result<Value, JvmError> {
        self.locals
            .get(index as usize)
            .cloned()
            .ok_or(JvmError::LocalVariableNotFound(index))
    }
}
