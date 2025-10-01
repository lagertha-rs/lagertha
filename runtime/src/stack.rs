use crate::VmConfig;
use crate::error::JvmError;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::method::java::Method;
use common::jtype::{HeapAddr, Value};
use log::debug;
use std::sync::Arc;

#[derive(Clone)]
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

    pub fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }

    pub fn push_frame(&mut self, frame: Frame) -> Result<(), JvmError> {
        debug!(
            "🚀 Executing {}.{}",
            frame.method.class()?.name()?,
            frame.method.name()
        );
        if self.frames.len() >= self.max_size {
            return Err(JvmError::StackOverflow);
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<Frame, JvmError> {
        let res = self.frames.pop().ok_or(JvmError::FrameStackIsEmpty)?;
        debug!(
            "🏁 Execution finished of {}.{}",
            res.method.class()?.name()?,
            res.method.name()
        );
        if let Some(frame) = self.frames.last() {
            debug!(
                "🔄 Resuming execution of {}.{}",
                frame.method.class()?.name()?,
                frame.method.name()
            );
        }
        Ok(res)
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

    pub fn pc(&self) -> Result<&usize, JvmError> {
        self.cur_frame().map(|v| &v.pc)
    }

    pub fn pc_mut(&mut self) -> Result<&mut usize, JvmError> {
        self.cur_frame_mut().map(|v| &mut v.pc)
    }

    pub fn get_local(&self, index: u8) -> Result<&Value, JvmError> {
        self.cur_frame()?.get_local(index)
    }

    pub fn get_local_int(&self, index: u8) -> Result<i32, JvmError> {
        match self.get_local(index)? {
            Value::Integer(v) => Ok(*v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer in local variable".to_string(),
            )),
        }
    }

    // TODO: check index bounds
    pub fn set_local(&mut self, idx: usize, value: Value) -> Result<(), JvmError> {
        self.cur_frame_mut()?.locals[idx] = Some(value);
        Ok(())
    }

    pub fn push_operand(&mut self, value: Value) -> Result<(), JvmError> {
        if self.cur_frame()?.operands.len() >= self.max_operand_stack_size {
            return Err(JvmError::StackOverflow);
        }
        self.cur_frame_mut()?.operands.push(value);
        Ok(())
    }

    pub fn pop_int(&mut self) -> Result<i32, JvmError> {
        match self.pop_operand()? {
            Value::Integer(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_float(&mut self) -> Result<f32, JvmError> {
        match self.pop_operand()? {
            Value::Float(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Float on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_nullable_obj_ref(&mut self) -> Result<Option<HeapAddr>, JvmError> {
        match self.pop_operand()? {
            Value::Object(v) | Value::Array(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Object on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_obj_ref(&mut self) -> Result<HeapAddr, JvmError> {
        self.pop_nullable_obj_ref()?
            .ok_or(JvmError::NullPointerException)
    }

    pub fn pop_nullable_array_ref(&mut self) -> Result<Option<HeapAddr>, JvmError> {
        match self.pop_operand()? {
            Value::Array(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Array on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_array_ref(&mut self) -> Result<HeapAddr, JvmError> {
        self.pop_nullable_array_ref()?
            .ok_or(JvmError::NullPointerException)
    }

    pub fn pop_ref(&mut self) -> Result<Option<HeapAddr>, JvmError> {
        match self.pop_operand()? {
            Value::Object(o) | Value::Array(o) => Ok(o),
            _ => Err(JvmError::UnexpectedType(
                "Expected reference (Object or Array) on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_operand(&mut self) -> Result<Value, JvmError> {
        self.cur_frame_mut()?.pop_operand()
    }

    //TODO: cloning cp every time may be inefficient, even if it's Arc
    pub fn cp(&self) -> Result<Arc<RuntimeConstantPool>, JvmError> {
        self.cur_frame().map(|v| v.cp.clone())
    }

    pub fn top_operand(&self) -> Result<&Value, JvmError> {
        self.cur_frame()?.get_top_operand()
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.6
#[derive(Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct Frame {
    locals: Vec<Option<Value>>,
    operands: Vec<Value>,
    #[cfg_attr(test, serde(skip_serializing))]
    cp: Arc<RuntimeConstantPool>,
    pc: usize,
    #[cfg_attr(test, serde(skip_serializing))]
    method: Arc<Method>,
}

impl Frame {
    pub fn new(
        cp: Arc<RuntimeConstantPool>,
        method: Arc<Method>,
        locals: Vec<Option<Value>>,
        max_stack: usize,
    ) -> Self {
        Self {
            locals,
            operands: Vec::with_capacity(max_stack),
            cp,
            pc: 0,
            method,
        }
    }

    pub fn method(&self) -> &Arc<Method> {
        &self.method
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
