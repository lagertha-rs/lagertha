use crate::heap::Heap;
use crate::method_area::MethodArea;
use crate::native::JNIEnv;
use crate::rt::class::class::{Class, InitState};
use crate::rt::constant_pool::RuntimeConstant;
use crate::rt::constant_pool::reference::MethodReference;
use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;
use crate::rt::method::{StaticMethodType, VirtualMethodType};
use crate::stack::{Frame, FrameStack};
use crate::{JvmError, MethodKey, VmConfig};
use common::instruction::Instruction;
use common::jtype::Value;
use std::sync::Arc;
use tracing_log::log::debug;

#[cfg_attr(test, derive(serde::Serialize))]
pub struct Interpreter {
    frame_stack: FrameStack,
    jni_env: JNIEnv,
    #[cfg(test)]
    last_pop_frame: Option<Frame>,
}

impl Interpreter {
    pub fn new(vm_config: &VmConfig, method_area: MethodArea) -> Self {
        let thread_stack = FrameStack::new(vm_config);
        let jni_env = JNIEnv::new(Heap::new(), method_area);
        Self {
            #[cfg(test)]
            last_pop_frame: None,
            frame_stack: thread_stack,
            jni_env,
        }
    }

    fn heap(&mut self) -> &mut Heap {
        self.jni_env.heap()
    }

    fn method_area(&self) -> &MethodArea {
        self.jni_env.method_area()
    }

    fn pop_frame(&mut self) -> Result<(), JvmError> {
        let _frame = self.frame_stack.pop_frame()?;
        #[cfg(test)]
        {
            self.last_pop_frame = Some(_frame);
        }
        Ok(())
    }

    fn ensure_initialized(&mut self, class: Option<&Arc<Class>>) -> Result<(), JvmError> {
        if let Some(class) = class {
            if let Some(super_class) = &class.super_class() {
                self.ensure_initialized(Some(super_class))?;
            }
            if !class.initialized()
                && let Some(initializer) = class.initializer()
            {
                class.set_state(InitState::Initializing);
                debug!("Initializing class {}", class.name()?);

                self.run_static_method_type(class, initializer)?;

                // TODO: need to be placed in better place
                // https://stackoverflow.com/questions/78321427/initialization-of-static-final-fields-in-javas-system-class-in-jdk-14-and-beyon
                if class.name()? == "java/lang/System" {
                    let init = class.get_static_method("initPhase1", "()V")?;
                    self.run_static_method_type(class, init)?;
                }

                class.set_state(InitState::Initialized);
                debug!("Class {} initialized", class.name()?);
            }
        }
        Ok(())
    }

    fn interpret_method_code(&mut self, code: &Vec<u8>, frame: Frame) -> Result<(), JvmError> {
        self.frame_stack.push_frame(frame)?;
        loop {
            let instruction = Instruction::new_at(code, *self.frame_stack.cur_frame_pc()?)?;
            if self.interpret_instruction(&instruction)? {
                break;
            }
        }

        Ok(())
    }

    // TODO: find a better solution for branches
    // maybe make Instruction::new_at return the next pc?
    fn branch16(bci: usize, off: i16) -> usize {
        ((bci as isize) + (off as isize)) as usize
    }
    fn branch32(bci: usize, off: i32) -> usize {
        ((bci as isize) + (off as isize)) as usize
    }

    // TODO: replace return book with enum or set special cp values
    fn interpret_instruction(&mut self, instruction: &Instruction) -> Result<bool, JvmError> {
        debug!("Executing instruction: {:?}", instruction);
        if !matches!(
            instruction,
            Instruction::Goto(_)
                | Instruction::IfIcmpge(_)
                | Instruction::Ifnonnull(_)
                | Instruction::IfNe(_)
                | Instruction::IfGe(_)
                | Instruction::IfLe(_)
                | Instruction::IfIcmple(_)
        ) {
            *self.frame_stack.cur_frame_pc_mut()? += instruction.byte_size() as usize;
        }
        match instruction {
            Instruction::Iconst0 => {
                self.frame_stack.cur_frame_push_operand(Value::Integer(0))?;
            }
            Instruction::Iconst1 => {
                self.frame_stack.cur_frame_push_operand(Value::Integer(1))?;
            }
            Instruction::Iconst2 => {
                self.frame_stack.cur_frame_push_operand(Value::Integer(2))?;
            }
            Instruction::Iconst3 => {
                self.frame_stack.cur_frame_push_operand(Value::Integer(3))?;
            }
            Instruction::Iconst4 => {
                self.frame_stack.cur_frame_push_operand(Value::Integer(4))?;
            }
            Instruction::Istore1 => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(1, value)?;
            }
            Instruction::Istore2 => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(2, value)?;
            }
            Instruction::Istore3 => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(3, value)?;
            }
            Instruction::Istore(idx) => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(*idx as usize, value)?;
            }
            Instruction::Iload1 => {
                let value = self.frame_stack.cur_frame_get_local(1)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?;
            }
            Instruction::Iload2 => {
                let value = self.frame_stack.cur_frame_get_local(2)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?;
            }
            Instruction::Fload2 => {
                let value = self.frame_stack.cur_frame_get_local(2)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?;
            }
            Instruction::Fconst0 => {
                self.frame_stack.cur_frame_push_operand(Value::Float(0.0))?;
            }
            Instruction::Fcmpg => {
                let value2 = self.frame_stack.cur_frame_pop_operand()?;
                let value1 = self.frame_stack.cur_frame_pop_operand()?;
                match (value1, value2) {
                    (Value::Float(v1), Value::Float(v2)) => {
                        let res = if v1 > v2 {
                            1
                        } else if v1 == v2 {
                            0
                        } else {
                            -1
                        };
                        self.frame_stack
                            .cur_frame_push_operand(Value::Integer(res))?;
                    }
                    _ => panic!("fcmpg on non-float values"),
                }
            }
            Instruction::IfLe(offset) => {
                let pc = *self.frame_stack.cur_frame_pc()?;
                let value = self.frame_stack.cur_frame_pop_operand()?;
                match value {
                    Value::Integer(i) => {
                        let new_pc = if i <= 0 {
                            Self::branch16(pc, *offset)
                        } else {
                            pc + instruction.byte_size() as usize
                        };
                        *self.frame_stack.cur_frame_pc_mut()? = new_pc;
                    }
                    _ => panic!("ifle on non-integer value"),
                }
            }
            Instruction::IfGe(offset) => {
                let pc = *self.frame_stack.cur_frame_pc()?;
                let value = self.frame_stack.cur_frame_pop_operand()?;
                match value {
                    Value::Integer(i) => {
                        let new_pc = if i >= 0 {
                            Self::branch16(pc, *offset)
                        } else {
                            pc + instruction.byte_size() as usize
                        };
                        *self.frame_stack.cur_frame_pc_mut()? = new_pc;
                    }
                    _ => panic!("ifge on non-integer value"),
                }
            }
            Instruction::IfIcmpge(offset) => {
                let pc = *self.frame_stack.cur_frame_pc()?;

                let value2 = self.frame_stack.cur_frame_pop_operand()?;
                let value1 = self.frame_stack.cur_frame_pop_operand()?;

                match (value1, value2) {
                    (Value::Integer(v1), Value::Integer(v2)) => {
                        let new_pc = if v1 >= v2 {
                            Self::branch16(pc, *offset)
                        } else {
                            pc + instruction.byte_size() as usize
                        };
                        *self.frame_stack.cur_frame_pc_mut()? = new_pc;
                    }
                    _ => panic!("if_icmpge on non-integer values"),
                }
            }
            Instruction::IfIcmple(offset) => {
                let pc = *self.frame_stack.cur_frame_pc()?;

                let value2 = self.frame_stack.cur_frame_pop_operand()?;
                let value1 = self.frame_stack.cur_frame_pop_operand()?;

                match (value1, value2) {
                    (Value::Integer(v1), Value::Integer(v2)) => {
                        let new_pc = if v1 <= v2 {
                            Self::branch16(pc, *offset)
                        } else {
                            pc + instruction.byte_size() as usize
                        };
                        *self.frame_stack.cur_frame_pc_mut()? = new_pc;
                    }
                    _ => panic!("if_icmple on non-integer values"),
                }
            }
            Instruction::Ifnonnull(offset) => {
                let pc = *self.frame_stack.cur_frame_pc()?;
                let value = self.frame_stack.cur_frame_pop_operand()?;
                match value {
                    Value::Object(o) => {
                        let new_pc = if o.is_some() {
                            Self::branch16(pc, *offset)
                        } else {
                            pc + instruction.byte_size() as usize
                        };
                        *self.frame_stack.cur_frame_pc_mut()? = new_pc;
                    }
                    _ => panic!("ifnonnull on non-object value"),
                }
            }
            Instruction::IfNe(offset) => {
                let pc = *self.frame_stack.cur_frame_pc()?;
                let value = self.frame_stack.cur_frame_pop_operand()?;
                match value {
                    Value::Integer(i) => {
                        let new_pc = if i != 0 {
                            Self::branch16(pc, *offset)
                        } else {
                            pc + instruction.byte_size() as usize
                        };
                        *self.frame_stack.cur_frame_pc_mut()? = new_pc;
                    }
                    _ => panic!("ifne on non-integer value"),
                }
            }
            Instruction::Putstatic(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let field_ref = cp.get_fieldref(idx)?;
                let class = self.method_area().get_class(field_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let field_nat = field_ref.name_and_type()?;
                class.set_static_field(field_nat, self.frame_stack.cur_frame_pop_operand()?)?;
            }
            Instruction::Getstatic(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let field_ref = cp.get_fieldref(idx)?;
                let class = self.method_area().get_class(field_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let field_nat = field_ref.name_and_type()?;
                let value = class.get_static_field_value(field_nat)?;
                self.frame_stack.cur_frame_push_operand(value)?;
            }
            Instruction::InvokeStatic(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let method_ref = cp.get_methodref(idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let method = class.get_static_method_by_nat(method_ref)?;
                self.run_static_method_type(&class, method)?;
            }
            Instruction::InvokeInterface(idx, _count) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let method_ref = cp.get_interface_methodref(idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                let method = class.get_virtual_method_by_nat(method_ref)?;
                self.run_instance_method_type(method, method_ref)?;
            }
            Instruction::AconstNull => {
                self.frame_stack
                    .cur_frame_push_operand(Value::Object(None))?;
            }
            Instruction::Ldc(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let raw = cp.get(idx)?;
                match raw {
                    RuntimeConstant::String(data) => {
                        let string_addr = self.heap().get_or_new(data.value()?);
                        self.frame_stack
                            .cur_frame_push_operand(Value::Object(Some(string_addr)))?;
                    }
                    RuntimeConstant::Class(class) => {
                        let class_mirror = self.jni_env.get_mirror(class.name()?)?;
                        self.frame_stack
                            .cur_frame_push_operand(Value::Object(Some(class_mirror)))?;
                    }
                    RuntimeConstant::Float(value) => {
                        self.frame_stack
                            .cur_frame_push_operand(Value::Float(*value))?;
                    }
                    RuntimeConstant::Integer(value) => {
                        self.frame_stack
                            .cur_frame_push_operand(Value::Integer(*value))?;
                    }
                    _ => unimplemented!("Ldc for constant {:?}", raw),
                }
            }
            Instruction::Anewarray(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let class_ref = cp.get_class(idx)?;
                let class = self.method_area().get_class(class_ref.name()?)?;
                let count = self.frame_stack.cur_frame_pop_operand()?;
                match count {
                    Value::Integer(c) if c >= 0 => {
                        let addr = self.heap().alloc_array_ref(class, c as usize);
                        self.frame_stack
                            .cur_frame_push_operand(Value::Array(Some(addr)))?;
                    }
                    Value::Integer(_) => {
                        return Err(JvmError::NegativeArraySizeException);
                    }
                    _ => panic!("anewarray with non-integer count"),
                }
            }
            Instruction::New(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let class_ref = cp.get_class(idx)?;
                let class = self.method_area().get_class(class_ref.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let addr = self.heap().alloc_instance(class);
                self.frame_stack
                    .cur_frame_push_operand(Value::Object(Some(addr)))?;
            }
            Instruction::Dup => {
                let value = self.frame_stack.cur_frame_top_operand()?;
                self.frame_stack.cur_frame_push_operand(value.clone())?;
            }
            Instruction::InvokeSpecial(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let method_ref = cp.get_methodref(idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                let method = class.get_virtual_method_by_nat(method_ref)?;
                self.run_instance_method_type(method, method_ref)?;
            }
            Instruction::InvokeVirtual(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let method_ref = cp.get_methodref(idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                let method = class.get_virtual_method_by_nat(method_ref)?;
                self.run_instance_method_type(method, method_ref)?;
            }
            Instruction::Aload0 => {
                let value = self.frame_stack.cur_frame_get_local(0)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?
            }
            Instruction::Aload1 => {
                let value = self.frame_stack.cur_frame_get_local(1)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?
            }
            Instruction::Aload2 => {
                let value = self.frame_stack.cur_frame_get_local(2)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?
            }
            Instruction::Aload3 => {
                let value = self.frame_stack.cur_frame_get_local(3)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?
            }
            Instruction::Aload(idx) => {
                let value = self.frame_stack.cur_frame_get_local(*idx)?.clone();
                self.frame_stack.cur_frame_push_operand(value)?
            }
            Instruction::Aastore => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                let index = self.frame_stack.cur_frame_pop_operand()?;
                let array_ref = self.frame_stack.cur_frame_pop_operand()?;
                match (array_ref, index) {
                    (Value::Array(Some(arr_addr)), Value::Integer(i)) => {
                        let heap_obj = self.heap().get_mut(arr_addr);
                        match heap_obj {
                            crate::heap::HeapObject::ArrayRef { elements, .. } => {
                                if i < 0 || (i as usize) >= elements.len() {
                                    return Err(JvmError::ArrayIndexOutOfBoundsException);
                                }
                                elements[i as usize] = value;
                            }
                            _ => panic!("aastore on non-array object"),
                        }
                    }
                    (Value::Object(None), _) => {
                        return Err(JvmError::NullPointerException);
                    }
                    _ => panic!("aastore on non-array object or non-integer index"),
                }
            }
            Instruction::Astore0 => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(0, value)?;
            }
            Instruction::Astore1 => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(1, value)?;
            }
            Instruction::Astore2 => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(2, value)?;
            }
            Instruction::Astore3 => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(3, value)?;
            }
            Instruction::Astore(idx) => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.frame_stack.cur_frame_set_local(*idx as usize, value)?;
            }
            Instruction::ArrayLength => {
                let array_ref = self.frame_stack.cur_frame_pop_operand()?;
                match array_ref {
                    Value::Array(Some(arr_addr)) => {
                        let heap_obj = self.heap().get(arr_addr);
                        match heap_obj {
                            crate::heap::HeapObject::ArrayRef { elements, .. } => {
                                let length = elements.len() as i32;
                                self.frame_stack
                                    .cur_frame_push_operand(Value::Integer(length))?;
                            }
                            _ => panic!("arraylength on non-array object"),
                        }
                    }
                    Value::Array(None) => {
                        return Err(JvmError::NullPointerException);
                    }
                    _ => panic!("arraylength on non-array object"),
                }
            }
            Instruction::Bipush(value) => {
                self.frame_stack
                    .cur_frame_push_operand(Value::Integer(*value as i32))?;
            }
            Instruction::Sipush(value) => {
                self.frame_stack
                    .cur_frame_push_operand(Value::Integer(*value as i32))?;
            }
            Instruction::Getfield(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let field_nat = cp.get_fieldref(idx)?.name_and_type()?;
                let object_ref = self.frame_stack.cur_frame_pop_operand()?;
                match object_ref {
                    Value::Object(Some(o)) => {
                        let value = self.heap().get_instance_field(&o, field_nat).clone();
                        self.frame_stack.cur_frame_push_operand(value)?;
                    }
                    Value::Object(None) => {
                        return Err(JvmError::NullPointerException);
                    }
                    _ => {
                        panic!("getfield on non-object");
                    }
                }
            }
            Instruction::Iadd => {
                let value2 = self.frame_stack.cur_frame_pop_operand()?;
                let value1 = self.frame_stack.cur_frame_pop_operand()?;
                match (value1, value2) {
                    (Value::Integer(v1), Value::Integer(v2)) => {
                        self.frame_stack
                            .cur_frame_push_operand(Value::Integer(v1 + v2))?;
                    }
                    _ => panic!("iadd on non-integer values"),
                }
            }
            Instruction::Idiv => {
                let value2 = self.frame_stack.cur_frame_pop_operand()?;
                let value1 = self.frame_stack.cur_frame_pop_operand()?;
                match (value1, value2) {
                    (Value::Integer(_), Value::Integer(0)) => {
                        return Err(JvmError::ArithmeticException(
                            "Division by zero".to_string(),
                        ));
                    }
                    (Value::Integer(v1), Value::Integer(v2)) => {
                        self.frame_stack
                            .cur_frame_push_operand(Value::Integer(v1 / v2))?;
                    }
                    _ => panic!("idiv on non-integer values"),
                }
            }
            Instruction::Iinc(index, const_val) => {
                let value = self.frame_stack.cur_frame_get_local(*index)?.clone();
                match value {
                    Value::Integer(v) => {
                        self.frame_stack.cur_frame_set_local(
                            *index as usize,
                            Value::Integer(v + (*const_val as i32)),
                        )?;
                    }
                    _ => panic!("iinc on non-integer value"),
                }
            }
            Instruction::Goto(offset) => {
                let pc = *self.frame_stack.cur_frame_pc()?;
                let new_pc = Self::branch16(pc, *offset);
                *self.frame_stack.cur_frame_pc_mut()? = new_pc;
            }
            Instruction::Putfield(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let field_nat = cp.get_fieldref(idx)?.name_and_type()?;
                let value = self.frame_stack.cur_frame_pop_operand()?;
                let object_ref = self.frame_stack.cur_frame_pop_operand()?;
                match object_ref {
                    Value::Object(Some(o)) => {
                        self.heap().write_instance_field(o, field_nat, value)?;
                    }
                    Value::Object(None) => {
                        return Err(JvmError::NullPointerException);
                    }
                    _ => {
                        panic!("putfield on non-object");
                    }
                }
            }
            Instruction::Areturn => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.pop_frame()?;
                self.frame_stack.cur_frame_push_operand(value)?;
                return Ok(true);
            }
            Instruction::Ireturn => {
                let value = self.frame_stack.cur_frame_pop_operand()?;
                self.pop_frame()?;
                self.frame_stack.cur_frame_push_operand(value)?;
                return Ok(true);
            }
            Instruction::Return => {
                self.pop_frame()?;
                return Ok(true);
            }
            unimp => unimplemented!("Instruction {:?} not implemented", unimp),
        }
        Ok(false)
    }

    fn run_instance_method(&mut self, method: &Method) -> Result<(), JvmError> {
        let class = method.class()?;

        // TODO: delete
        let debug_msg = format!(
            "instance method {}{} of class {}",
            method.name(),
            method.descriptor().raw(),
            class.name()?
        );
        debug!("Running {debug_msg}");

        let mut params = vec![None; method.max_locals()?];
        let params_count = method.descriptor().resolved().params.len() + 1; // +1 for this
        for i in (0..params_count).rev() {
            params[i] = Some(self.frame_stack.cur_frame_pop_operand()?);
        }

        let frame = Frame::new(class.cp().clone(), params, method.max_stack()?, debug_msg);

        self.interpret_method_code(method.instructions()?, frame)?;
        Ok(())
    }

    fn run_abstract_method(
        &mut self,
        abstract_method: &Method,
        method_ref: &MethodReference,
    ) -> Result<(), JvmError> {
        let interface = abstract_method.class()?;

        let params_count = abstract_method.descriptor().resolved().params.len() + 1;
        let mut params = vec![None; params_count];
        for i in (0..params_count).rev() {
            params[i] = Some(self.frame_stack.cur_frame_pop_operand()?);
        }

        let class = match &params[0] {
            Some(Value::Object(Some(o))) => self.heap().get_instance(o).class().clone(),
            Some(Value::Object(None)) => return Err(JvmError::NullPointerException),
            _ => panic!("Abstract method called on non-object"),
        };
        let method = class.get_virtual_method_by_nat(method_ref)?;

        if let VirtualMethodType::Java(method) = method {
            for _ in 0..(method.max_locals()? - params_count) {
                params.push(None);
            }
            let debug_msg = format!(
                "abstract method {}{} of {} that is implemented by class {}",
                abstract_method.name(),
                abstract_method.descriptor().raw(),
                interface.name()?,
                class.name()?
            );
            debug!("Running {debug_msg}");
            let frame = Frame::new(class.cp().clone(), params, method.max_stack()?, debug_msg);

            self.interpret_method_code(method.instructions()?, frame)?;
        } else {
            unimplemented!()
        }
        Ok(())
    }

    fn run_instance_method_type(
        &mut self,
        method: &VirtualMethodType,
        method_ref: &MethodReference,
    ) -> Result<(), JvmError> {
        match method {
            VirtualMethodType::Java(method) => self.run_instance_method(method)?,
            VirtualMethodType::Abstract(method) => self.run_abstract_method(method, method_ref)?,
            VirtualMethodType::Native(_method) => {
                unimplemented!("Native instance methods not implemented yet");
            }
        }
        Ok(())
    }

    fn run_static_method(&mut self, class: &Arc<Class>, method: &Method) -> Result<(), JvmError> {
        let debug_msg = format!(
            "static method {}{} of class {}",
            method.name(),
            method.descriptor().raw(),
            class.name()?
        );
        debug!("Running {debug_msg}");

        let mut params = vec![None; method.max_locals()?];
        let params_count = method.descriptor().resolved().params.len();

        for i in (0..params_count).rev() {
            params[i] = Some(self.frame_stack.cur_frame_pop_operand()?);
        }

        let frame = Frame::new(class.cp().clone(), params, method.max_stack()?, debug_msg);

        self.interpret_method_code(method.instructions()?, frame)?;

        Ok(())
    }

    fn run_static_native_method(
        &mut self,
        class: &Arc<Class>,
        method: &NativeMethod,
    ) -> Result<(), JvmError> {
        debug!(
            "Running static native method {}{} of class {}",
            method.name(),
            method.descriptor().raw(),
            class.name()?
        );

        let params_count = method.descriptor().resolved().params.len();
        let params: Vec<_> = (0..params_count)
            .map(|_| self.frame_stack.cur_frame_pop_operand())
            .rev()
            .collect::<Result<_, _>>()?;

        let method_key = MethodKey::new(
            class.name()?.to_string(),
            method.name().to_string(),
            method.descriptor().raw().to_string(),
        );
        let method = self
            .jni_env
            .native_registry
            .get(&method_key)
            .ok_or(JvmError::NoSuchMethod(format!("{method_key:?}")))?;
        let ret_value = method(&mut self.jni_env, params.as_slice());
        self.frame_stack.cur_frame_push_operand(ret_value)?;
        Ok(())
    }

    fn run_static_method_type(
        &mut self,
        class: &Arc<Class>,
        method: &StaticMethodType,
    ) -> Result<(), JvmError> {
        match method {
            StaticMethodType::Java(method) => self.run_static_method(class, method),
            StaticMethodType::Native(method) => self.run_static_native_method(class, method),
        }
    }

    //TODO: redisign start method (maybe return Value, maybe take args)
    pub fn start(&mut self, data: Vec<u8>) -> Result<(), JvmError> {
        let main_class = self.method_area().add_class(data)?;
        let main_method = main_class
            .find_main_method()
            .ok_or(JvmError::NoMainClassFound(main_class.name()?.to_string()))?;
        let debug_msg = format!("main method of class {}", main_class.name()?);
        debug!("Found {debug_msg}");
        self.ensure_initialized(Some(&main_class))?;
        let instructions = main_method.instructions()?;
        //TODO: handle args
        let frame = Frame::new(
            main_class.cp().clone(),
            vec![None; main_method.max_locals()?],
            main_method.max_stack()?,
            debug_msg,
        );
        debug!("Executing main method...");
        self.interpret_method_code(instructions, frame)?;
        debug!("Main method finished.");

        //TODO: delete, since I don't have return in main and tests for it
        // just to be sure that stack is empty
        assert!(self.pop_frame().is_err());

        Ok(())
    }
}
