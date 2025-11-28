use crate::{MethodId, VmConfig, build_exception, debug_log_method};
use common::error::{JavaExceptionFromJvm, JvmError};
use common::{HeapRef, Value};

#[derive(Clone)]
pub enum FrameType {
    JavaFrame(JavaFrame),
    NativeFrame(NativeFrame),
}

impl FrameType {
    pub fn method_id(&self) -> MethodId {
        match self {
            FrameType::JavaFrame(f) => f.method_id,
            FrameType::NativeFrame(f) => f.method_id,
        }
    }
}

#[derive(Clone)]
pub struct NativeFrame {
    method_id: MethodId,
}

impl NativeFrame {
    pub fn new(method_id: MethodId) -> Self {
        Self { method_id }
    }
}

pub struct FrameStack {
    max_size: usize,
    frames: Vec<FrameType>,
}

impl FrameStack {
    pub fn new(vm_config: &VmConfig) -> Self {
        let max_size = vm_config.frame_stack_size;
        Self {
            max_size,
            frames: Vec::with_capacity(max_size),
        }
    }

    pub fn frames(&self) -> &Vec<FrameType> {
        &self.frames
    }

    pub fn push_frame(&mut self, frame: FrameType) -> Result<(), JvmError> {
        match &frame {
            FrameType::JavaFrame(f) => {
                debug_log_method!(&f.method_id, "🚀 Executing");
            }
            FrameType::NativeFrame(f) => {
                debug_log_method!(&f.method_id, "🚀 Executing native method");
            }
        }
        if self.frames.len() >= self.max_size {
            return Err(JvmError::StackOverflow);
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<FrameType, JvmError> {
        let old_frame = self.frames.pop().ok_or(JvmError::FrameStackIsEmpty)?;
        match &old_frame {
            FrameType::JavaFrame(f) => {
                debug_log_method!(&f.method_id, "🏁 Execution finished");
            }
            FrameType::NativeFrame(f) => {
                debug_log_method!(&f.method_id, "🏁 Execution finished of native method");
            }
        };
        if let Some(cur_frame) = self.frames.last() {
            match cur_frame {
                FrameType::JavaFrame(f) => {
                    debug_log_method!(&f.method_id, "🔄 Resuming execution")
                }
                FrameType::NativeFrame(f) => {
                    debug_log_method!(&f.method_id, "🔄 Resuming execution of native method")
                }
            }
        };
        Ok(old_frame)
    }

    pub fn pop_java_frame(&mut self) -> Result<JavaFrame, JvmError> {
        match self.pop_frame()? {
            FrameType::JavaFrame(f) => Ok(f),
            FrameType::NativeFrame(_) => Err(JvmError::UnexpectedType(
                "Expected Java frame on top of the stack".to_string(),
            )),
        }
    }

    pub fn pop_native_frame(&mut self) -> Result<NativeFrame, JvmError> {
        match self.pop_frame()? {
            FrameType::NativeFrame(f) => Ok(f),
            FrameType::JavaFrame(_) => Err(JvmError::UnexpectedType(
                "Expected Native frame on top of the stack".to_string(),
            )),
        }
    }

    pub fn cur_java_frame_mut(&mut self) -> Result<&mut JavaFrame, JvmError> {
        self.frames
            .last_mut()
            .and_then(|frame| match frame {
                FrameType::JavaFrame(f) => Some(f),
                FrameType::NativeFrame(_) => None,
            })
            .ok_or(JvmError::UnexpectedType(
                "Expected Java frame on top of the stack".to_string(),
            ))
    }

    pub fn cur_java_frame(&self) -> Result<&JavaFrame, JvmError> {
        self.frames
            .last()
            .and_then(|frame| match frame {
                FrameType::JavaFrame(f) => Some(f),
                FrameType::NativeFrame(_) => None,
            })
            .ok_or(JvmError::UnexpectedType(
                "Expected Java frame on top of the stack".to_string(),
            ))
    }

    pub fn pc(&self) -> Result<usize, JvmError> {
        self.cur_java_frame().map(|v| v.pc)
    }

    pub fn pc_mut(&mut self) -> Result<&mut usize, JvmError> {
        self.cur_java_frame_mut().map(|v| &mut v.pc)
    }

    fn get_local(&self, index: u8) -> Result<&Value, JvmError> {
        self.cur_java_frame()?.get_local(index)
    }

    pub fn get_local_double(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Double(_) => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Double in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_long(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Long(_) => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Long in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_int(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Integer(_) => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_int_val(&self, index: u8) -> Result<i32, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Integer(v) => Ok(*v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_float(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Float(_) => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Float in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_ref(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Ref(_) | Value::Null => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Object or Array in local variable".to_string(),
            )),
        }
    }

    // TODO: check index bounds
    pub fn set_local(&mut self, idx: usize, value: Value) -> Result<(), JvmError> {
        self.cur_java_frame_mut()?.locals[idx] = Some(value);
        Ok(())
    }

    pub fn push_operand(&mut self, value: Value) -> Result<(), JvmError> {
        self.cur_java_frame_mut()?.operands.push(value);
        Ok(())
    }

    pub fn pop_int(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Integer(v) => Ok(Value::Integer(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_int_val(&mut self) -> Result<i32, JvmError> {
        match self.pop_operand()? {
            Value::Integer(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_long(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Long(v) => Ok(Value::Long(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Long on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_long_val(&mut self) -> Result<i64, JvmError> {
        match self.pop_operand()? {
            Value::Long(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Long on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_double(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Double(v) => Ok(Value::Double(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Double on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_double_val(&mut self) -> Result<f64, JvmError> {
        match self.pop_operand()? {
            Value::Double(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Double on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_float(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Float(v) => Ok(Value::Float(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Float on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_float_val(&mut self) -> Result<f32, JvmError> {
        match self.pop_operand()? {
            Value::Float(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Float on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_nullable_ref(&mut self) -> Result<Value, JvmError> {
        let value = self.pop_operand()?;
        match &value {
            Value::Ref(_) | Value::Null => Ok(value),
            _ => Err(JvmError::UnexpectedType(
                "Expected Object on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_nullable_ref_val(&mut self) -> Result<Option<HeapRef>, JvmError> {
        match self.pop_operand()? {
            Value::Ref(v) => Ok(Some(v)),
            Value::Null => Ok(None),
            _ => Err(JvmError::UnexpectedType(
                "Expected Object on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_obj_val(&mut self) -> Result<HeapRef, JvmError> {
        self.pop_nullable_ref_val()?
            .ok_or(build_exception!(NullPointerException))
    }

    pub fn pop_operand(&mut self) -> Result<Value, JvmError> {
        self.cur_java_frame_mut()?.pop_operand()
    }

    pub fn peek(&self) -> Result<&Value, JvmError> {
        self.cur_java_frame()?.peek()
    }

    pub fn peek_at(&self, index: usize) -> Result<&Value, JvmError> {
        let frame = self.cur_java_frame()?;
        if index >= frame.operands.len() {
            return Err(JvmError::OperandStackIsEmpty);
        }
        Ok(&frame.operands[frame.operands.len() - 1 - index])
    }

    pub fn dup_top(&mut self) -> Result<(), JvmError> {
        self.push_operand(*self.peek()?)
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-2.html#jvms-2.6
#[derive(Clone)]
pub struct JavaFrame {
    locals: Vec<Option<Value>>,
    operands: Vec<Value>,
    pc: usize,
    method_id: MethodId,
}

impl JavaFrame {
    // TODO: rethink params and this mapping
    fn args_to_frame_locals(params: Vec<Value>, max_locals: u16) -> Vec<Option<Value>> {
        let mut locals = vec![None; max_locals as usize];

        let mut pos = 0;
        for v in params {
            match v {
                Value::Long(_) | Value::Double(_) => {
                    locals[pos] = Some(v);
                    pos += 2;
                }
                _ => {
                    locals[pos] = Some(v);
                    pos += 1;
                }
            }
        }
        locals
    }

    pub fn new(method_id: MethodId, max_stack: u16, max_locals: u16, args: Vec<Value>) -> Self {
        Self {
            locals: Self::args_to_frame_locals(args, max_locals),
            operands: Vec::with_capacity(max_stack as usize),
            pc: 0,
            method_id,
        }
    }

    pub fn method_id(&self) -> MethodId {
        self.method_id
    }

    pub fn get_local(&self, index: u8) -> Result<&Value, JvmError> {
        self.locals
            .get(index as usize)
            .and_then(|v| v.as_ref())
            .ok_or(JvmError::LocalVariableNotInitialized(index))
    }

    pub fn peek(&self) -> Result<&Value, JvmError> {
        self.operands.last().ok_or(JvmError::OperandStackIsEmpty)
    }

    pub fn pop_operand(&mut self) -> Result<Value, JvmError> {
        self.operands.pop().ok_or(JvmError::OperandStackIsEmpty)
    }

    pub fn increment_pc(&mut self, offset: u16) {
        self.pc += offset as usize;
    }

    pub fn pc(&self) -> usize {
        self.pc
    }
}
