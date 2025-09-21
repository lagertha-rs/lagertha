use crate::rt::constant_pool::RuntimeConstantPool;
use crate::{JvmError, VmConfig};
use common::jtype::Value;
use std::sync::Arc;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct FrameStack {
    max_size: usize,
    max_operand_stack_size: usize,
    frames: Vec<Frame>,
}

impl FrameStack {
    pub fn new(vm_config: &VmConfig) -> Self {
        let max_size = vm_config.frame_stack_size;
        Self {
            max_size,
            max_operand_stack_size: vm_config.operand_stack_size,
            frames: Vec::with_capacity(max_size),
        }
    }

    pub fn push_frame(&mut self, frame: Frame) -> Result<(), JvmError> {
        if self.frames.len() >= self.max_size {
            return Err(JvmError::StackOverflow);
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<Frame, JvmError> {
        self.frames.pop().ok_or(JvmError::FrameStackIsEmpty)
    }

    fn cur_frame_mut(&mut self) -> Result<&mut Frame, JvmError> {
        if self.frames.is_empty() {
            return Err(JvmError::FrameStackIsEmpty);
        }

        self.frames.last_mut().ok_or(JvmError::FrameStackIsEmpty)
    }

    fn cur_frame(&self) -> Result<&Frame, JvmError> {
        if self.frames.is_empty() {
            return Err(JvmError::FrameStackIsEmpty);
        }

        self.frames.last().ok_or(JvmError::FrameStackIsEmpty)
    }

    pub fn cur_frame_pc(&self) -> Result<&usize, JvmError> {
        self.cur_frame().map(|v| &v.pc)
    }

    pub fn cur_frame_pc_mut(&mut self) -> Result<&mut usize, JvmError> {
        self.cur_frame_mut().map(|v| &mut v.pc)
    }

    pub fn cur_frame_get_local(&self, index: u8) -> Result<&Value, JvmError> {
        self.cur_frame()?.get_local(index)
    }

    // TODO: check index bounds
    pub fn cur_frame_set_local(&mut self, idx: usize, value: Value) -> Result<(), JvmError> {
        self.cur_frame_mut()?.locals[idx] = Some(value);
        Ok(())
    }

    pub fn cur_frame_push_operand(&mut self, value: Value) -> Result<(), JvmError> {
        if self.cur_frame()?.operands.len() >= self.max_operand_stack_size {
            return Err(JvmError::StackOverflow);
        }
        self.cur_frame_mut()?.operands.push(value);
        Ok(())
    }

    pub fn cur_frame_pop_operand(&mut self) -> Result<Value, JvmError> {
        self.cur_frame_mut()?.pop_operand()
    }

    //TODO: cloning cp every time may be inefficient
    pub fn cur_frame_cp(&self) -> Result<Arc<RuntimeConstantPool>, JvmError> {
        self.cur_frame().map(|v| v.cp.clone())
    }

    pub fn cur_frame_top_operand(&self) -> Result<&Value, JvmError> {
        self.cur_frame()?.get_top_operand()
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.6
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Frame {
    locals: Vec<Option<Value>>,
    operands: Vec<Value>,
    #[cfg_attr(test, serde(skip_serializing))]
    cp: Arc<RuntimeConstantPool>,
    pc: usize,
}

impl Frame {
    pub fn new(cp: Arc<RuntimeConstantPool>, locals: Vec<Option<Value>>, max_stack: usize) -> Self {
        Self {
            locals,
            operands: Vec::with_capacity(max_stack),
            cp,
            pc: 0,
        }
    }

    pub fn get_local(&self, index: u8) -> Result<&Value, JvmError> {
        self.locals
            .get(index as usize)
            .and_then(|v| v.as_ref())
            .ok_or(JvmError::LocalVariableNotInitialized(index))
    }

    pub fn get_top_operand(&self) -> Result<&Value, JvmError> {
        self.operands.last().ok_or(JvmError::OperandStackIsEmpty)
    }

    pub fn pop_operand(&mut self) -> Result<Value, JvmError> {
        self.operands.pop().ok_or(JvmError::OperandStackIsEmpty)
    }
}
