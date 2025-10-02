use crate::VirtualMachine;
use crate::error::JvmError;
use crate::heap::{Heap, HeapObject};
use crate::method_area::MethodArea;
use crate::native::MethodKey;
use crate::rt::class::class::{Class, InitState};
use crate::rt::constant_pool::RuntimeConstant;
use crate::rt::constant_pool::reference::MethodReference;
use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;
use crate::rt::method::{StaticMethodType, VirtualMethodType};
use crate::stack::Frame;
use common::instruction::Instruction;
use common::jtype::Value;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use std::sync::Arc;
use tracing_log::log::debug;

#[cfg_attr(test, derive(serde::Serialize))]
pub struct Interpreter {
    vm: VirtualMachine,
    #[cfg_attr(test, serde(skip_serializing))]
    heap: Rc<RefCell<Heap>>,
    #[cfg(test)]
    last_pop_frame: Option<Frame>,
}

impl Interpreter {
    pub fn new(vm: VirtualMachine) -> Self {
        Self {
            #[cfg(test)]
            last_pop_frame: None,
            heap: vm.heap(),
            vm,
        }
    }

    fn method_area(&mut self) -> &mut MethodArea {
        self.vm.method_area()
    }

    fn pop_frame(&mut self) -> Result<(), JvmError> {
        let _frame = self.vm.frame_stack.pop_frame()?;
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
                debug!("Initializing class {}", class.name());

                self.run_static_method_type(class, initializer)?;

                // TODO: need to be placed in better place
                // https://stackoverflow.com/questions/78321427/initialization-of-static-final-fields-in-javas-system-class-in-jdk-14-and-beyon
                if class.name() == "java/lang/System" {
                    let init = class.get_static_method("initPhase1", "()V")?;
                    self.run_static_method_type(class, init)?;
                }

                class.set_state(InitState::Initialized);
                debug!("Class {} initialized", class.name());
            }
        }
        Ok(())
    }

    fn interpret_method_code(&mut self, code: &Vec<u8>, frame: Frame) -> Result<(), JvmError> {
        self.vm.frame_stack.push_frame(frame)?;
        loop {
            let instruction = Instruction::new_at(code, *self.vm.frame_stack.pc()?)?;
            if self.interpret_instruction(instruction)? {
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
    fn interpret_instruction(&mut self, instruction: Instruction) -> Result<bool, JvmError> {
        debug!("Executing instruction: {:?}", instruction);
        if !matches!(
            instruction,
            Instruction::Goto(_)
                | Instruction::IfIcmpge(_)
                | Instruction::IfIcmpeq(_)
                | Instruction::IfIcmpne(_)
                | Instruction::Ifnonnull(_)
                | Instruction::IfIcmplt(_)
                | Instruction::IfNe(_)
                | Instruction::IfGe(_)
                | Instruction::IfLe(_)
                | Instruction::IfEq(_)
                | Instruction::IfIcmple(_)
                | Instruction::IfGt(_)
                | Instruction::Ifnull(_)
        ) {
            *self.vm.frame_stack.pc_mut()? += instruction.byte_size() as usize;
        }
        match instruction {
            Instruction::Athrow => {
                let exception_ref = self.vm.frame_stack.pop_obj_ref()?;
                Err(JvmError::JavaExceptionThrown(exception_ref))?
            }
            Instruction::Checkcast(_idx) => {
                //TODO: stub
                let object_ref = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.push_operand(object_ref)?;
            }
            Instruction::IconstM1 => {
                self.vm.frame_stack.push_operand(Value::Integer(-1))?;
            }
            Instruction::Iconst0 => {
                self.vm.frame_stack.push_operand(Value::Integer(0))?;
            }
            Instruction::Iconst1 => {
                self.vm.frame_stack.push_operand(Value::Integer(1))?;
            }
            Instruction::Iconst2 => {
                self.vm.frame_stack.push_operand(Value::Integer(2))?;
            }
            Instruction::Iconst3 => {
                self.vm.frame_stack.push_operand(Value::Integer(3))?;
            }
            Instruction::Iconst4 => {
                self.vm.frame_stack.push_operand(Value::Integer(4))?;
            }
            Instruction::Iconst5 => {
                self.vm.frame_stack.push_operand(Value::Integer(5))?;
            }
            Instruction::Istore0 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(1, value)?;
            }
            Instruction::Istore1 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(1, value)?;
            }
            Instruction::Istore2 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(2, value)?;
            }
            Instruction::Istore3 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(3, value)?;
            }
            Instruction::Istore(idx) => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(idx as usize, value)?;
            }
            Instruction::Iload0 => {
                let value = self.vm.frame_stack.get_local(0)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload1 => {
                let value = self.vm.frame_stack.get_local(1)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload2 => {
                let value = self.vm.frame_stack.get_local(2)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload3 => {
                let value = self.vm.frame_stack.get_local(3)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload(n) => {
                let value = self.vm.frame_stack.get_local(n)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fload0 => {
                let value = self.vm.frame_stack.get_local(0)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fload2 => {
                let value = self.vm.frame_stack.get_local(2)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fconst0 => {
                self.vm.frame_stack.push_operand(Value::Float(0.0))?;
            }
            Instruction::Fcmpl => {
                let v2 = self.vm.frame_stack.pop_float()?;
                let v1 = self.vm.frame_stack.pop_float()?;
                let res = match v1.total_cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                self.vm.frame_stack.push_operand(Value::Integer(res))?;
            }
            Instruction::Fcmpg => {
                let v2 = self.vm.frame_stack.pop_float()?;
                let v1 = self.vm.frame_stack.pop_float()?;
                let res = match v1.total_cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                self.vm.frame_stack.push_operand(Value::Integer(res))?;
            }
            Instruction::Ifnull(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_ref()?;
                let new_pc = if value.is_none() {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfEq(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int()?;
                let new_pc = if value == 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmplt(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;

                let new_pc = if v1 < v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfGt(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int()?;
                let new_pc = if value > 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfLe(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int()?;
                let new_pc = if value <= 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfGe(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int()?;
                let new_pc = if value >= 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpne(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                let new_pc = if v1 != v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpge(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                let new_pc = if v1 >= v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpeq(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                let new_pc = if v1 == v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmple(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                let new_pc = if v1 <= v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::Ifnonnull(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let obj = self.vm.frame_stack.pop_nullable_obj_ref()?;
                let new_pc = if obj.is_some() {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfNe(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let i = self.vm.frame_stack.pop_int()?;
                let new_pc = if i != 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::Iushr => {
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                let shift = (v2 & 0x1F) as u32;
                let result = ((v1 as u32) >> shift) as i32;
                self.vm.frame_stack.push_operand(Value::Integer(result))?;
            }
            Instruction::Ishl => {
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                let shift = (v2 & 0x1F) as u32;
                let result = v1.wrapping_shl(shift);
                self.vm.frame_stack.push_operand(Value::Integer(result))?;
            }
            Instruction::Ishr => {
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                let shift = (v2 & 0x1F) as u32;
                let result = v1.wrapping_shr(shift);
                self.vm.frame_stack.push_operand(Value::Integer(result))?;
            }
            Instruction::Putstatic(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let field_ref = cp.get_fieldref(&idx)?;
                let class = self.method_area().get_class(field_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let field_nat = field_ref.name_and_type()?;
                class.set_static_field(field_nat, self.vm.frame_stack.pop_operand()?)?;
            }
            Instruction::Getstatic(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let field_ref = cp.get_fieldref(&idx)?;
                let class = self.method_area().get_class(field_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let field_nat = field_ref.name_and_type()?;
                let value = class.get_static_field_value(field_nat)?;
                self.vm.frame_stack.push_operand(value)?;
            }
            Instruction::InvokeStatic(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_methodref(&idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let method = class.get_static_method_by_nat(method_ref)?;
                self.run_static_method_type(&class, method)?;
            }
            Instruction::InvokeInterface(idx, _count) => {
                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_interface_methodref(&idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                let method = class.get_virtual_method_by_nat(method_ref)?;
                self.run_instance_method_type(method, method_ref)?;
            }
            Instruction::AconstNull => {
                self.vm.frame_stack.push_operand(Value::Object(None))?;
            }
            Instruction::Ldc(idx) | Instruction::LdcW(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let raw = cp.get(&idx)?;
                match raw {
                    RuntimeConstant::String(data) => {
                        let string_addr = self.heap.borrow_mut().get_or_new_string(data.value()?);
                        self.vm
                            .frame_stack
                            .push_operand(Value::Object(Some(string_addr)))?;
                    }
                    RuntimeConstant::Class(class) => {
                        let class_mirror = self
                            .vm
                            .method_area()
                            .get_mirror_addr_by_name(class.name()?)?;
                        self.vm
                            .frame_stack
                            .push_operand(Value::Object(Some(class_mirror)))?;
                    }
                    RuntimeConstant::Float(value) => {
                        self.vm.frame_stack.push_operand(Value::Float(*value))?;
                    }
                    RuntimeConstant::Integer(value) => {
                        self.vm.frame_stack.push_operand(Value::Integer(*value))?;
                    }
                    _ => unimplemented!("Ldc for constant {:?}", raw),
                }
            }
            Instruction::Anewarray(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let class_ref = cp.get_class(&idx)?;
                let class = self.method_area().get_class(class_ref.name()?)?;
                let size = self.vm.frame_stack.pop_int()?;
                if size >= 0 {
                    let addr = self.heap.borrow_mut().alloc_array(class, size as usize);
                    self.vm.frame_stack.push_operand(Value::Array(Some(addr)))?;
                } else {
                    return Err(JvmError::NegativeArraySizeException)?;
                }
            }
            Instruction::Newarray(array_type) => {
                let count = self.vm.frame_stack.pop_int()?;
                if count >= 0 {
                    let primitive_type_name = array_type.descriptor();
                    let primitive_class = self.method_area().get_class(primitive_type_name)?;
                    let addr = self
                        .heap
                        .borrow_mut()
                        .alloc_array(primitive_class, count as usize);
                    self.vm.frame_stack.push_operand(Value::Array(Some(addr)))?;
                } else {
                    Err(JvmError::NegativeArraySizeException)?
                }
            }
            Instruction::New(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let class_ref = cp.get_class(&idx)?;
                let class = self.method_area().get_class(class_ref.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let addr = self.heap.borrow_mut().alloc_instance(class);
                self.vm
                    .frame_stack
                    .push_operand(Value::Object(Some(addr)))?;
            }
            Instruction::Dup => {
                let value = self.vm.frame_stack.top_operand()?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::InvokeSpecial(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_methodref(&idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                let method = class.get_virtual_method_by_nat(method_ref)?;
                self.run_instance_method_type(method, method_ref)?;
            }
            Instruction::InvokeVirtual(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_methodref(&idx)?;
                let class = self.method_area().get_class(method_ref.class()?.name()?)?;
                let method = class.get_virtual_method_by_nat(method_ref)?;
                self.run_instance_method_type(method, method_ref)?;
            }
            Instruction::Aload0 => {
                let value = self.vm.frame_stack.get_local(0)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload1 => {
                let value = self.vm.frame_stack.get_local(1)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload2 => {
                let value = self.vm.frame_stack.get_local(2)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload3 => {
                let value = self.vm.frame_stack.get_local(3)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload(idx) => {
                let value = self.vm.frame_stack.get_local(idx)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Castore => {
                let value = self.vm.frame_stack.pop_int()?;
                let index = self.vm.frame_stack.pop_int()?;
                let array_addr = self.vm.frame_stack.pop_array_ref()?;
                self.heap.borrow_mut().write_array_element(
                    array_addr,
                    index as usize,
                    Value::Integer(value & 0xFF),
                )?;
            }
            Instruction::Iastore => {
                let value = self.vm.frame_stack.pop_int()?;
                let index = self.vm.frame_stack.pop_int()?;
                let array_addr = self.vm.frame_stack.pop_array_ref()?;
                self.heap.borrow_mut().write_array_element(
                    array_addr,
                    index as usize,
                    Value::Integer(value),
                )?;
            }
            Instruction::Aastore => {
                let value = self.vm.frame_stack.pop_nullable_obj_ref()?;
                let index = self.vm.frame_stack.pop_int()?;
                let array_addr = self.vm.frame_stack.pop_array_ref()?;
                self.heap.borrow_mut().write_array_element(
                    array_addr,
                    index as usize,
                    Value::Object(value),
                )?;
            }
            Instruction::Astore0 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(0, value)?;
            }
            Instruction::Astore1 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(1, value)?;
            }
            Instruction::Astore2 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(2, value)?;
            }
            Instruction::Astore3 => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(3, value)?;
            }
            Instruction::Astore(idx) => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.set_local(idx as usize, value)?;
            }
            Instruction::ArrayLength => {
                let array_addr = self.vm.frame_stack.pop_array_ref()?;
                let length = self.heap.borrow().get_array(&array_addr).length();
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer(length as i32))?;
            }
            Instruction::Bipush(value) => {
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::Sipush(value) => {
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer(value as i32))?;
            }
            Instruction::Getfield(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let field_nat = cp.get_fieldref(&idx)?.name_and_type()?;
                let object_addr = self.vm.frame_stack.pop_obj_ref()?;
                let value = *self
                    .heap
                    .borrow_mut()
                    .get_instance_field(&object_addr, field_nat);
                self.vm.frame_stack.push_operand(value)?;
            }
            Instruction::Iadd => {
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                self.vm.frame_stack.push_operand(Value::Integer(v1 + v2))?;
            }
            Instruction::Isub => {
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                self.vm.frame_stack.push_operand(Value::Integer(v1 - v2))?;
            }
            Instruction::Idiv => {
                let v2 = self.vm.frame_stack.pop_int()?;
                let v1 = self.vm.frame_stack.pop_int()?;
                if v2 == 0 {
                    Err(JvmError::ArithmeticException(
                        "Division by zero".to_string(),
                    ))?
                }
                self.vm.frame_stack.push_operand(Value::Integer(v1 / v2))?;
            }
            Instruction::Iinc(index, const_val) => {
                let value = self.vm.frame_stack.get_local_int(index)?;
                self.vm
                    .frame_stack
                    .set_local(index as usize, Value::Integer(value + (const_val as i32)))?;
            }
            Instruction::Goto(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let new_pc = Self::branch16(pc, offset);
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::Putfield(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let field_nat = cp.get_fieldref(&idx)?.name_and_type()?;
                let value = self.vm.frame_stack.pop_operand()?;
                let object_addr = self.vm.frame_stack.pop_obj_ref()?;
                self.heap.borrow_mut().write_instance_field_by_nat(
                    object_addr,
                    field_nat,
                    value,
                )?;
            }
            Instruction::Areturn => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.pop_frame()?;
                self.vm.frame_stack.push_operand(value)?;
                return Ok(true);
            }
            Instruction::Ireturn => {
                let value = self.vm.frame_stack.pop_operand()?;
                self.pop_frame()?;
                self.vm.frame_stack.push_operand(value)?;
                return Ok(true);
            }
            Instruction::Return => {
                self.pop_frame()?;
                return Ok(true);
            }
            Instruction::Pop => {
                self.vm.frame_stack.pop_operand()?;
            }
            unimp => unimplemented!("Instruction {:?} not implemented", unimp),
        }
        Ok(false)
    }

    fn run_instance_native_method(&mut self, method: &NativeMethod) -> Result<(), JvmError> {
        let class = method.class()?;

        // TODO: delete
        let debug_msg = format!(
            "instance native method {}{} of class {}",
            method.name(),
            method.descriptor().raw(),
            class.name()
        );
        debug!("Running {debug_msg}");

        let params_count = method.descriptor().resolved().params.len() + 1; // +1 for this
        let mut params = Vec::with_capacity(params_count);
        for _ in 0..params_count {
            params.push(self.vm.frame_stack.pop_operand()?);
        }
        params.reverse();

        let method_key = MethodKey::new(
            class.name().to_string(),
            method.name().to_string(),
            method.descriptor().raw().to_string(),
        );
        let method = self
            .vm
            .native_registry
            .get(&method_key)
            .ok_or(JvmError::NoSuchMethod(format!("{method_key:?}")))?;

        let ret_value = method(&mut self.vm, params.as_slice());
        self.vm.frame_stack.push_operand(ret_value)?;

        Ok(())
    }

    fn run_instance_method(&mut self, method: &Arc<Method>) -> Result<(), JvmError> {
        let class = method.class()?;
        let mut params = vec![None; method.max_locals()?];
        let params_count = method.descriptor().resolved().params.len() + 1; // +1 for this
        for i in (0..params_count).rev() {
            params[i] = Some(self.vm.frame_stack.pop_operand()?);
        }

        let frame = Frame::new(
            class.cp().clone(),
            method.clone(),
            params,
            method.max_stack()?,
        );

        self.interpret_method_code(method.instructions()?, frame)?;
        Ok(())
    }

    fn run_abstract_method(
        &mut self,
        abstract_method: &Arc<Method>,
        method_ref: &MethodReference,
    ) -> Result<(), JvmError> {
        let params_count = abstract_method.descriptor().resolved().params.len() + 1;
        let mut params = vec![None; params_count];
        for i in (0..params_count).rev() {
            params[i] = Some(self.vm.frame_stack.pop_operand()?);
        }

        let class = match &params[0] {
            Some(Value::Object(Some(o))) => self.heap.borrow_mut().get_instance(o).class().clone(),
            Some(Value::Object(None)) => return Err(JvmError::NullPointerException),
            _ => panic!("Abstract method called on non-object"),
        };
        let method = class.get_virtual_method_by_nat(method_ref)?;

        if let VirtualMethodType::Java(method) = method {
            for _ in 0..(method.max_locals()? - params_count) {
                params.push(None);
            }
            let frame = Frame::new(
                class.cp().clone(),
                method.clone(),
                params,
                method.max_stack()?,
            );

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
            VirtualMethodType::Native(method) => self.run_instance_native_method(method)?,
        }
        Ok(())
    }

    fn run_static_method(
        &mut self,
        class: &Arc<Class>,
        method: &Arc<Method>,
    ) -> Result<(), JvmError> {
        let mut params = vec![None; method.max_locals()?];
        let params_count = method.descriptor().resolved().params.len();

        for i in (0..params_count).rev() {
            params[i] = Some(self.vm.frame_stack.pop_operand()?);
        }

        let frame = Frame::new(
            class.cp().clone(),
            method.clone(),
            params,
            method.max_stack()?,
        );

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
            class.name()
        );

        let params_count = method.descriptor().resolved().params.len();
        let mut params = Vec::with_capacity(params_count);
        for _ in 0..params_count {
            params.push(self.vm.frame_stack.pop_operand()?);
        }
        params.reverse();

        let method_key = MethodKey::new(
            class.name().to_string(),
            method.name().to_string(),
            method.descriptor().raw().to_string(),
        );
        let method = self
            .vm
            .native_registry
            .get(&method_key)
            .ok_or(JvmError::NoSuchMethod(format!("{method_key:?}")))?;
        let ret_value = method(&mut self.vm, params.as_slice());
        self.vm.frame_stack.push_operand(ret_value)?;
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
        let main_class = self.method_area().add_raw_bytecode(data)?;
        let main_method = main_class
            .find_main_method()
            .ok_or(JvmError::NoMainClassFound(main_class.name().to_string()))?;
        debug!("Found main method of class {}", main_class.name());
        self.ensure_initialized(Some(&main_class))?;
        let instructions = main_method.instructions()?;
        //TODO: handle args
        let frame = Frame::new(
            main_class.cp().clone(),
            main_method.clone(),
            vec![None; main_method.max_locals()?],
            main_method.max_stack()?,
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
