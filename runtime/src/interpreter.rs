use crate::rt::class::{Class, InitState};
use crate::rt::constant_pool::RuntimeConstant;
use crate::rt::constant_pool::reference::MethodReference;
use crate::rt::method::{Method, MethodType};
use crate::stack::{FrameType, JavaFrame, NativeFrame};
use crate::{ClassId, VirtualMachine, throw_exception};
use common::error::JavaExceptionFromJvm;
use common::error::JvmError;
use common::instruction::Instruction;
use common::jtype::Value;
use log::warn;
use std::cmp::Ordering;
use std::sync::Arc;
use tracing_log::log::debug;

pub struct Interpreter {
    vm: VirtualMachine,
}

impl Interpreter {
    pub fn new(vm: VirtualMachine) -> Self {
        Self { vm }
    }

    pub fn vm(&mut self) -> &mut VirtualMachine {
        &mut self.vm
    }

    fn pop_frame(&mut self) -> Result<(), JvmError> {
        let _frame = self.vm.frame_stack.pop_frame()?;
        Ok(())
    }

    pub fn ensure_initialized(&mut self, class_id: &ClassId) -> Result<(), JvmError> {
        self.ensure_initialized_recursive(Some(class_id))
    }

    // FIXME: very bad
    fn ensure_initialized_recursive(&mut self, class_id: Option<&ClassId>) -> Result<(), JvmError> {
        if let Some(class_id) = class_id {
            let class = self.vm.method_area.get_class_by_id(class_id)?.clone();
            if let Some(super_class) = class.super_class() {
                self.ensure_initialized_recursive(Some(super_class.id()))?;
            }
            if !class.initialized()
                && let Some(initializer) = class.initializer()
            {
                class.set_state(InitState::Initializing);
                debug!("Initializing class {}", class.name());

                self.run_static_method_type(&class, initializer, vec![])?;

                // TODO: need to be placed in better place
                // https://stackoverflow.com/questions/78321427/initialization-of-static-final-fields-in-javas-system-class-in-jdk-14-and-beyon
                if class.name() == "java/lang/System" {
                    let init = class.get_static_method("initPhase1", "()V")?;
                    self.run_static_method_type(&class, init, vec![])?;
                }

                class.set_state(InitState::Initialized);
                debug!("Class {} initialized", class.name());
            }
        }
        Ok(())
    }

    //TODO: probably need to move it, refactor and it will still probably will not work for catch
    fn allocate_and_throw(&mut self, exception: JavaExceptionFromJvm) -> Result<(), JvmError> {
        let exception_ref = exception.as_reference();
        let class_id = self.vm.method_area.get_class_id(exception_ref.class)?;
        let class = self.vm.method_area.get_class_by_id(&class_id)?.clone();
        let instance = self.vm.heap.alloc_instance(&class)?;
        let method = class.get_virtual_method(exception_ref.name, exception_ref.descriptor)?;
        let params = if let Some(msg) = exception.get_message() {
            vec![
                Value::Ref(instance),
                Value::Ref(self.vm.heap.get_or_new_string(msg)),
            ]
        } else {
            vec![Value::Ref(instance)]
        };
        self.run_instance_method(method, params)?;
        Err(JvmError::JavaExceptionThrown(instance))
    }

    fn interpret_method_code(&mut self, code: &Vec<u8>, frame: JavaFrame) -> Result<(), JvmError> {
        self.vm
            .frame_stack
            .push_frame(FrameType::JavaFrame(frame))?;
        loop {
            let instruction = Instruction::new_at(code, *self.vm.frame_stack.pc()?)?;
            // TODO: cleanup here and interpret_instruction return type
            let res = self.interpret_instruction(instruction);
            match res {
                Ok(should_return) => {
                    if should_return {
                        break;
                    }
                }
                Err(e) => match e {
                    JvmError::JavaException(exception) => {
                        self.allocate_and_throw(exception)?;
                    }
                    e => return Err(e),
                },
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

    // TODO: replace return bool with enum or set special cp values
    fn interpret_instruction(&mut self, instruction: Instruction) -> Result<bool, JvmError> {
        debug!("Executing instruction: {:?}", instruction);
        let need_increase_pc = !matches!(
            instruction,
            Instruction::Goto(_)
                | Instruction::Lookupswitch(_)
                | Instruction::TableSwitch(_)
                | Instruction::IfLt(_)
                | Instruction::IfAcmpEq(_)
                | Instruction::IfAcmpNe(_)
                | Instruction::IfIcmpge(_)
                | Instruction::IfIcmpgt(_)
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
        );
        let instruction_byte_size = instruction.byte_size();

        match instruction {
            Instruction::Athrow => {
                let exception_ref = self.vm.frame_stack.pop_obj_val()?;
                Err(JvmError::JavaExceptionThrown(exception_ref))?
            }
            Instruction::Checkcast(_idx) => {
                //TODO: stub
                let object_ref = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.push_operand(object_ref)?;
            }
            Instruction::Dadd => {
                let v2 = self.vm.frame_stack.pop_double_val()?;
                let v1 = self.vm.frame_stack.pop_double_val()?;
                self.vm.frame_stack.push_operand(Value::Double(v1 + v2))?;
            }
            Instruction::Dconst0 => {
                self.vm.frame_stack.push_operand(Value::Double(0.0))?;
            }
            Instruction::Dconst1 => {
                self.vm.frame_stack.push_operand(Value::Double(1.0))?;
            }
            Instruction::Lconst0 => {
                self.vm.frame_stack.push_operand(Value::Long(0))?;
            }
            Instruction::Lconst1 => {
                self.vm.frame_stack.push_operand(Value::Long(1))?;
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
            Instruction::Lstore(n) => {
                let value = self.vm.frame_stack.pop_long()?;
                self.vm.frame_stack.set_local(n as usize, value)?;
            }
            Instruction::Lstore0 => {
                let value = self.vm.frame_stack.pop_long()?;
                self.vm.frame_stack.set_local(0, value)?;
            }
            Instruction::Lstore1 => {
                let value = self.vm.frame_stack.pop_long()?;
                self.vm.frame_stack.set_local(1, value)?;
            }
            Instruction::Lstore2 => {
                let value = self.vm.frame_stack.pop_long()?;
                self.vm.frame_stack.set_local(2, value)?;
            }
            Instruction::Lstore3 => {
                let value = self.vm.frame_stack.pop_long()?;
                self.vm.frame_stack.set_local(3, value)?;
            }
            Instruction::Fstore(n) => {
                let value = self.vm.frame_stack.pop_float()?;
                self.vm.frame_stack.set_local(n as usize, value)?;
            }
            Instruction::Fstore0 => {
                let value = self.vm.frame_stack.pop_float()?;
                self.vm.frame_stack.set_local(0, value)?;
            }
            Instruction::Fstore1 => {
                let value = self.vm.frame_stack.pop_float()?;
                self.vm.frame_stack.set_local(1, value)?;
            }
            Instruction::Fstore2 => {
                let value = self.vm.frame_stack.pop_float()?;
                self.vm.frame_stack.set_local(2, value)?;
            }
            Instruction::Fstore3 => {
                let value = self.vm.frame_stack.pop_float()?;
                self.vm.frame_stack.set_local(3, value)?;
            }
            Instruction::Istore0 => {
                let value = self.vm.frame_stack.pop_int()?;
                self.vm.frame_stack.set_local(0, value)?;
            }
            Instruction::Istore1 => {
                let value = self.vm.frame_stack.pop_int()?;
                self.vm.frame_stack.set_local(1, value)?;
            }
            Instruction::Istore2 => {
                let value = self.vm.frame_stack.pop_int()?;
                self.vm.frame_stack.set_local(2, value)?;
            }
            Instruction::Istore3 => {
                let value = self.vm.frame_stack.pop_int()?;
                self.vm.frame_stack.set_local(3, value)?;
            }
            Instruction::Istore(idx) => {
                let value = self.vm.frame_stack.pop_int()?;
                self.vm.frame_stack.set_local(idx as usize, value)?;
            }
            Instruction::Lload0 => {
                let value = self.vm.frame_stack.get_local_long(0)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Lload1 => {
                let value = self.vm.frame_stack.get_local_long(1)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Lload2 => {
                let value = self.vm.frame_stack.get_local_long(2)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Lload3 => {
                let value = self.vm.frame_stack.get_local_long(3)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Lload(n) => {
                let value = self.vm.frame_stack.get_local_long(n)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Aaload => {
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                let value = *self.vm.heap.get_array(&array_addr)?.get_element(index)?;
                if matches!(value, Value::Ref(_) | Value::Null) {
                    self.vm.frame_stack.push_operand(value)?;
                } else {
                    panic!("Expected object reference in aaload");
                }
            }
            Instruction::Caload | Instruction::Baload | Instruction::Iaload => {
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                let value = *self.vm.heap.get_array(&array_addr)?.get_element(index)?;
                if let Value::Integer(i) = value {
                    self.vm.frame_stack.push_operand(Value::Integer(i & 0xFF))?;
                } else {
                    panic!("Expected integer value in caload");
                }
            }
            Instruction::Fload0 => {
                let value = self.vm.frame_stack.get_local_float(0)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fload1 => {
                let value = self.vm.frame_stack.get_local_float(1)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fload2 => {
                let value = self.vm.frame_stack.get_local_float(2)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fload3 => {
                let value = self.vm.frame_stack.get_local_float(3)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fload(n) => {
                let value = self.vm.frame_stack.get_local_float(n)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload0 => {
                let value = self.vm.frame_stack.get_local_int(0)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload1 => {
                let value = self.vm.frame_stack.get_local_int(1)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload2 => {
                let value = self.vm.frame_stack.get_local_int(2)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload3 => {
                let value = self.vm.frame_stack.get_local_int(3)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Iload(n) => {
                let value = self.vm.frame_stack.get_local_int(n)?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Fconst0 => {
                self.vm.frame_stack.push_operand(Value::Float(0.0))?;
            }
            Instruction::Fconst1 => {
                self.vm.frame_stack.push_operand(Value::Float(1.0))?;
            }
            Instruction::Lcmp => {
                let v2 = self.vm.frame_stack.pop_long_val()?;
                let v1 = self.vm.frame_stack.pop_long_val()?;
                let res = match v1.cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                self.vm.frame_stack.push_operand(Value::Integer(res))?;
            }
            Instruction::Fcmpl => {
                let v2 = self.vm.frame_stack.pop_float_val()?;
                let v1 = self.vm.frame_stack.pop_float_val()?;
                let res = match v1.total_cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                self.vm.frame_stack.push_operand(Value::Integer(res))?;
            }
            Instruction::Fcmpg => {
                let v2 = self.vm.frame_stack.pop_float_val()?;
                let v1 = self.vm.frame_stack.pop_float_val()?;
                let res = match v1.total_cmp(&v2) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
                self.vm.frame_stack.push_operand(Value::Integer(res))?;
            }
            Instruction::Ifnull(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_nullable_ref_val()?;
                let new_pc = if value.is_none() {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfEq(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if value == 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmplt(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;

                let new_pc = if v1 < v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfGt(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if value > 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfLe(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if value <= 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfLt(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if value < 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfGe(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let value = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if value >= 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfAcmpEq(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_nullable_ref_val()?;
                let v1 = self.vm.frame_stack.pop_nullable_ref_val()?;
                let new_pc = if v1 == v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfAcmpNe(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_nullable_ref_val()?;
                let v1 = self.vm.frame_stack.pop_nullable_ref_val()?;
                let new_pc = if v1 != v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpne(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if v1 != v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpge(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if v1 >= v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpgt(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if v1 > v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmpeq(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if v1 == v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfIcmple(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if v1 <= v2 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::Ifnonnull(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let obj = self.vm.frame_stack.pop_nullable_ref_val()?;
                let new_pc = if obj.is_some() {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::IfNe(offset) => {
                let pc = *self.vm.frame_stack.pc()?;
                let i = self.vm.frame_stack.pop_int_val()?;
                let new_pc = if i != 0 {
                    Self::branch16(pc, offset)
                } else {
                    pc + instruction.byte_size() as usize
                };
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::Iushr => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let shift = (v2 & 0x1F) as u32;
                let result = ((v1 as u32) >> shift) as i32;
                self.vm.frame_stack.push_operand(Value::Integer(result))?;
            }
            Instruction::Lshl => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_long_val()?;
                let shift = (v2 & 0x3F) as u32;
                let result = v1.wrapping_shl(shift);
                self.vm.frame_stack.push_operand(Value::Long(result))?;
            }
            Instruction::Ishl => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let shift = (v2 & 0x1F) as u32;
                let result = v1.wrapping_shl(shift);
                self.vm.frame_stack.push_operand(Value::Integer(result))?;
            }
            Instruction::Ishr => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                let shift = (v2 & 0x1F) as u32;
                let result = v1.wrapping_shr(shift);
                self.vm.frame_stack.push_operand(Value::Integer(result))?;
            }
            Instruction::Putstatic(idx) => {
                let value = self.vm.frame_stack.pop_operand()?;
                let class_id = {
                    let cp = self.vm.frame_stack.cp()?;
                    let field_ref = cp.get_fieldref(&idx)?;
                    self.vm
                        .method_area
                        .get_class_id(field_ref.class_ref()?.name()?)?
                };
                self.ensure_initialized(&class_id)?;
                let cp = self.vm.frame_stack.cp()?;
                let field_ref = cp.get_fieldref(&idx)?;
                let field_nat = field_ref.name_and_type_ref()?;
                let class = self.vm.method_area.get_class_by_id(&class_id)?;
                class.set_static_field_by_nat(field_nat, value)?;
            }
            Instruction::Getstatic(idx) => {
                let class_id = {
                    let cp = self.vm.frame_stack.cp()?;
                    let field_ref = cp.get_fieldref(&idx)?;
                    self.vm
                        .method_area
                        .get_class_id(field_ref.class_ref()?.name()?)?
                };
                self.ensure_initialized(&class_id)?;
                let cp = self.vm.frame_stack.cp()?;
                let field_ref = cp.get_fieldref(&idx)?;
                let field_nat = field_ref.name_and_type_ref()?;
                let class = self.vm.method_area.get_class_by_id(&class_id)?;
                let value = class.get_static_field_value_by_nat(field_nat)?;
                self.vm.frame_stack.push_operand(value)?;
            }
            Instruction::InvokeStatic(idx) => {
                let class_id = {
                    let cp = self.vm.frame_stack.cp()?;
                    let method_ref = cp.get_methodref(&idx)?;
                    self.vm
                        .method_area
                        .get_class_id(method_ref.class_ref()?.name()?)?
                };
                self.ensure_initialized(&class_id)?;

                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_methodref(&idx)?;
                let class = self.vm.method_area.get_class_by_id(&class_id)?.clone();
                /// FIXME
                let method = class.get_static_method_by_nat(method_ref)?;
                let params = self.prepare_method_params(method)?;
                self.run_static_method_type(&class, method, params)?;
            }
            Instruction::InvokeInterface(idx, _count) => {
                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_interface_methodref(&idx)?.clone(); // FIXME
                let class = self
                    .vm
                    .method_area
                    .get_class_or_load_by_name(method_ref.class_ref()?.name()?)?
                    .clone(); // FIXME
                let method = class.get_virtual_method_by_nat(&method_ref)?;
                let params = self.prepare_method_params(method)?;
                if method.name() == "startThreads"
                    && class.name() == "jdk/internal/access/JavaLangRefAccess"
                {
                    warn!(
                        "TODO: Stub: Ignoring call to jdk/internal/access/JavaLangRefAccess.startThreads"
                    );
                } else {
                    self.run_instance_method_type(method, &method_ref, params)?;
                }
            }
            Instruction::AconstNull => {
                self.vm.frame_stack.push_operand(Value::Null)?;
            }
            Instruction::Ldc(idx) | Instruction::LdcW(idx) | Instruction::Ldc2W(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let raw = cp.get(&idx)?;
                match raw {
                    RuntimeConstant::String(data) => {
                        let string_addr = self.vm.heap.get_or_new_string(data.value()?);
                        self.vm.frame_stack.push_operand(Value::Ref(string_addr))?;
                    }
                    RuntimeConstant::Class(class) => {
                        let class = self
                            .vm
                            .method_area
                            .get_class_or_load_by_name(class.name()?)?;
                        let class_mirror = self.vm.heap.get_mirror_addr(class)?;
                        self.vm.frame_stack.push_operand(Value::Ref(class_mirror))?;
                    }
                    RuntimeConstant::Double(value) => {
                        self.vm.frame_stack.push_operand(Value::Double(*value))?;
                    }
                    RuntimeConstant::Float(value) => {
                        self.vm.frame_stack.push_operand(Value::Float(*value))?;
                    }
                    RuntimeConstant::Integer(value) => {
                        self.vm.frame_stack.push_operand(Value::Integer(*value))?;
                    }
                    RuntimeConstant::Long(value) => {
                        self.vm.frame_stack.push_operand(Value::Long(*value))?;
                    }
                    _ => throw_exception!(
                        UnsupportedOperationException,
                        "Ldc for constant {:?}",
                        raw
                    )?,
                }
            }
            Instruction::Anewarray(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let class_ref = cp.get_class(&idx)?;
                let class = self
                    .vm
                    .method_area
                    .get_class_or_load_by_name(class_ref.name()?)?;
                let size = self.vm.frame_stack.pop_int_val()?;
                if size >= 0 {
                    let addr = self.vm.heap.alloc_array(class, size as usize)?;
                    self.vm.frame_stack.push_operand(Value::Ref(addr))?;
                } else {
                    throw_exception!(NegativeArraySizeException, size.to_string())?
                }
            }
            Instruction::Newarray(array_type) => {
                let size = self.vm.frame_stack.pop_int_val()?;
                if size >= 0 {
                    let primitive_type_name = array_type.descriptor();
                    let primitive_class = self
                        .vm
                        .method_area
                        .get_class_or_load_by_name(primitive_type_name)?;
                    let addr = self.vm.heap.alloc_array(primitive_class, size as usize)?;
                    self.vm.frame_stack.push_operand(Value::Ref(addr))?;
                } else {
                    throw_exception!(NegativeArraySizeException, size.to_string())?
                }
            }
            Instruction::New(idx) => {
                let class_id = {
                    let cp = self.vm.frame_stack.cp()?;
                    let class_ref = cp.get_class(&idx)?;
                    self.vm.method_area.get_class_id(class_ref.name()?)?
                };
                self.ensure_initialized(&class_id)?;
                let class = self.vm.method_area.get_class_by_id(&class_id)?;
                let addr = self.vm.heap.alloc_instance(class)?;
                self.vm.frame_stack.push_operand(Value::Ref(addr))?;
            }
            Instruction::Dup => {
                let value = self.vm.frame_stack.peek()?;
                self.vm.frame_stack.push_operand(*value)?;
            }
            Instruction::Dup2 => match self.vm.frame_stack.peek()? {
                Value::Long(_) => {
                    let value = self.vm.frame_stack.pop_long()?;
                    self.vm.frame_stack.push_operand(value)?;
                    self.vm.frame_stack.push_operand(value)?;
                }
                Value::Double(_) => {
                    let value = self.vm.frame_stack.pop_double()?;
                    self.vm.frame_stack.push_operand(value)?;
                    self.vm.frame_stack.push_operand(value)?;
                }
                _ => {
                    let value1 = self.vm.frame_stack.pop_operand()?;
                    let value2 = self.vm.frame_stack.pop_operand()?;
                    self.vm.frame_stack.push_operand(value2)?;
                    self.vm.frame_stack.push_operand(value1)?;
                    self.vm.frame_stack.push_operand(value2)?;
                    self.vm.frame_stack.push_operand(value1)?;
                }
            },
            Instruction::DupX1 => {
                let value1 = self.vm.frame_stack.pop_operand()?;
                let value2 = self.vm.frame_stack.pop_operand()?;
                self.vm.frame_stack.push_operand(value1)?;
                self.vm.frame_stack.push_operand(value2)?;
                self.vm.frame_stack.push_operand(value1)?;
            }
            Instruction::InvokeSpecial(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_methodref(&idx)?;
                let class = self
                    .vm
                    .method_area
                    .get_class_or_load_by_name(method_ref.class_ref()?.name()?)?
                    .clone(); // FIXME
                let method = class.get_virtual_method_by_nat(method_ref)?;
                if !matches!(method.type_of(), MethodType::Java) {
                    unimplemented!("InvokeSpecial for native or abstract methods");
                }
                let class = method.class()?;
                let params = self.prepare_method_params(method)?;
                let locals = self.params_to_frame_locals(method, params)?;

                let frame = JavaFrame::new(class.cp().clone(), method.clone(), locals)?;

                self.interpret_method_code(method.instructions()?, frame)?;
            }
            Instruction::InvokeVirtual(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let method_ref = cp.get_methodref(&idx)?.clone(); // FIXME
                let class = self
                    .vm
                    .method_area
                    .get_class_or_load_by_name(method_ref.class_ref()?.name()?)?
                    .clone(); // FIXME
                let method = class.get_virtual_method_by_nat(&method_ref)?;

                let params = self.prepare_method_params(method)?;
                match params.first() {
                    // try to dynamically dispatch the method
                    Some(Value::Ref(obj)) if self.vm.heap.addr_is_instance(obj)? => {
                        let instance = self.vm.heap.get_instance(obj)?;
                        let class_id = instance.class_id();
                        let this_class = self.vm.method_area.get_class_by_id(class_id)?.clone(); ////////////////////
                        let method = this_class.get_virtual_method_by_nat(&method_ref)?;
                        self.run_instance_method_type(method, &method_ref, params)
                    }
                    Some(Value::Ref(_)) => {
                        self.run_instance_method_type(method, &method_ref, params)
                    }
                    Some(Value::Null) => Err(JvmError::JavaException(
                        JavaExceptionFromJvm::NullPointerException(None),
                    ))?,
                    _ => panic!("First parameter of instance method must be object reference"),
                }?;
            }
            Instruction::Aload0 => {
                let value = self.vm.frame_stack.get_local_ref(0)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload1 => {
                let value = self.vm.frame_stack.get_local_ref(1)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload2 => {
                let value = self.vm.frame_stack.get_local_ref(2)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload3 => {
                let value = self.vm.frame_stack.get_local_ref(3)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Aload(idx) => {
                let value = self.vm.frame_stack.get_local_ref(idx)?;
                self.vm.frame_stack.push_operand(*value)?
            }
            Instruction::Bastore => {
                let value = self.vm.frame_stack.pop_int_val()?;
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                self.vm.heap.write_array_element(
                    array_addr,
                    index,
                    Value::Integer(value & 0xFF),
                )?;
            }
            Instruction::Sastore => {
                let value = self.vm.frame_stack.pop_int_val()?;
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                self.vm.heap.write_array_element(
                    array_addr,
                    index,
                    Value::Integer(value & 0xFFFF),
                )?;
            }
            Instruction::Saload => {
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                let value = *self.vm.heap.get_array(&array_addr)?.get_element(index)?;
                if let Value::Integer(i) = value {
                    self.vm
                        .frame_stack
                        .push_operand(Value::Integer(i & 0xFFFF))?;
                } else {
                    panic!("Expected integer value in saload");
                }
            }
            Instruction::Castore => {
                let value = self.vm.frame_stack.pop_int_val()?;
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                self.vm.heap.write_array_element(
                    array_addr,
                    index,
                    Value::Integer(value & 0xFF),
                )?;
            }
            Instruction::Instanceof(idx) => {
                let cp = self.vm.frame_stack.cp()?;
                let class_ref = cp.get_class(&idx)?;
                let other_class = self.vm.method_area.get_class_id(class_ref.name()?)?;
                let obj_addr = self.vm.frame_stack.pop_nullable_ref_val()?;
                if let Some(addr) = &obj_addr {
                    let target_class_id = self.vm.heap.get_instance(addr)?.class_id();
                    let target_class = self.vm.method_area.get_class_by_id(target_class_id)?;
                    let result = target_class.instance_of(&other_class);
                    self.vm
                        .frame_stack
                        .push_operand(Value::Integer(if result { 1 } else { 0 }))?;
                } else {
                    self.vm.frame_stack.push_operand(Value::Integer(0))?;
                }
            }
            Instruction::Iastore => {
                let value = self.vm.frame_stack.pop_int_val()?;
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                self.vm
                    .heap
                    .write_array_element(array_addr, index, Value::Integer(value))?;
            }
            Instruction::Aastore => {
                let value = self.vm.frame_stack.pop_nullable_ref()?;
                let index = self.vm.frame_stack.pop_int_val()?;
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                self.vm.heap.write_array_element(array_addr, index, value)?;
            }
            Instruction::Astore0 => {
                let value = self.vm.frame_stack.pop_nullable_ref()?;
                self.vm.frame_stack.set_local(0, value)?;
            }
            Instruction::Astore1 => {
                let value = self.vm.frame_stack.pop_nullable_ref()?;
                self.vm.frame_stack.set_local(1, value)?;
            }
            Instruction::Astore2 => {
                let value = self.vm.frame_stack.pop_nullable_ref()?;
                self.vm.frame_stack.set_local(2, value)?;
            }
            Instruction::Astore3 => {
                let value = self.vm.frame_stack.pop_nullable_ref()?;
                self.vm.frame_stack.set_local(3, value)?;
            }
            Instruction::Astore(idx) => {
                let value = self.vm.frame_stack.pop_nullable_ref()?;
                self.vm.frame_stack.set_local(idx as usize, value)?;
            }
            Instruction::ArrayLength => {
                let array_addr = self.vm.frame_stack.pop_obj_val()?;
                let length = self.vm.heap.get_array(&array_addr)?.length();
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
                let object_addr = self.vm.frame_stack.pop_obj_val()?;
                let class_id = self.vm.heap.get_class_id(&object_addr);
                let field_offset = {
                    let cp = self.vm.frame_stack.cp()?;
                    let nat = cp.get_fieldref(&idx)?.name_and_type_ref()?;
                    self.vm
                        .method_area
                        .get_class_by_id(&class_id)?
                        .get_field_index_by_nat(nat)?
                };
                let value = *self
                    .vm
                    .heap
                    .get_instance_field(&object_addr, field_offset)?;
                self.vm.frame_stack.push_operand(value)?;
            }
            Instruction::Iand => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                self.vm.frame_stack.push_operand(Value::Integer(v1 & v2))?;
            }
            Instruction::Ior => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                self.vm.frame_stack.push_operand(Value::Integer(v1 | v2))?;
            }
            Instruction::Ixor => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                self.vm.frame_stack.push_operand(Value::Integer(v1 ^ v2))?;
            }
            Instruction::L2i => {
                let v = self.vm.frame_stack.pop_long_val()?;
                self.vm.frame_stack.push_operand(Value::Integer(v as i32))?;
            }
            Instruction::L2f => {
                let v = self.vm.frame_stack.pop_long_val()?;
                self.vm.frame_stack.push_operand(Value::Float(v as f32))?;
            }
            Instruction::D2l => {
                let v = self.vm.frame_stack.pop_double_val()?;
                self.vm.frame_stack.push_operand(Value::Long(v as i64))?;
            }
            Instruction::F2i => {
                let v = self.vm.frame_stack.pop_float_val()?;
                self.vm.frame_stack.push_operand(Value::Integer(v as i32))?;
            }
            Instruction::F2d => {
                let v = self.vm.frame_stack.pop_float_val()?;
                self.vm.frame_stack.push_operand(Value::Double(v as f64))?;
            }
            Instruction::Ineg => {
                let v = self.vm.frame_stack.pop_int_val()?;
                self.vm.frame_stack.push_operand(Value::Integer(-v))?;
            }
            Instruction::I2s => {
                let v = self.vm.frame_stack.pop_int_val()?;
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer((v as i16) as i32))?;
            }
            Instruction::I2c => {
                let v = self.vm.frame_stack.pop_int_val()?;
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer((v as u16) as i32))?;
            }
            Instruction::I2l => {
                let v = self.vm.frame_stack.pop_int_val()?;
                self.vm.frame_stack.push_operand(Value::Long(v as i64))?;
            }
            Instruction::I2f => {
                let v = self.vm.frame_stack.pop_int_val()?;
                self.vm.frame_stack.push_operand(Value::Float(v as f32))?;
            }
            Instruction::I2b => {
                let v = self.vm.frame_stack.pop_int_val()?;
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer((v as i8) as i32))?;
            }
            Instruction::Fmul => {
                let v2 = self.vm.frame_stack.pop_float_val()?;
                let v1 = self.vm.frame_stack.pop_float_val()?;
                self.vm.frame_stack.push_operand(Value::Float(v1 * v2))?;
            }
            Instruction::Fdiv => {
                let v2 = self.vm.frame_stack.pop_float_val()?;
                let v1 = self.vm.frame_stack.pop_float_val()?;
                self.vm.frame_stack.push_operand(Value::Float(v1 / v2))?;
            }
            Instruction::Irem => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                if v2 == 0 {
                    throw_exception!(ArithmeticException, "Division by zero")?
                }
                self.vm.frame_stack.push_operand(Value::Integer(v1 % v2))?;
            }
            Instruction::Iadd => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer(v1.wrapping_add(v2)))?;
            }
            Instruction::Ladd => {
                let v2 = self.vm.frame_stack.pop_long_val()?;
                let v1 = self.vm.frame_stack.pop_long_val()?;
                self.vm
                    .frame_stack
                    .push_operand(Value::Long(v1.wrapping_add(v2)))?;
            }
            Instruction::Isub => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                self.vm.frame_stack.push_operand(Value::Integer(v1 - v2))?;
            }
            Instruction::Imul => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                self.vm
                    .frame_stack
                    .push_operand(Value::Integer(v1.wrapping_mul(v2)))?;
            }
            Instruction::Idiv => {
                let v2 = self.vm.frame_stack.pop_int_val()?;
                let v1 = self.vm.frame_stack.pop_int_val()?;
                if v2 == 0 {
                    throw_exception!(ArithmeticException, "/ by zero")?
                }
                self.vm.frame_stack.push_operand(Value::Integer(v1 / v2))?;
            }
            Instruction::Iinc(index, const_val) => {
                let value = self.vm.frame_stack.get_local_int_val(index)?;
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
                let value = self.vm.frame_stack.pop_operand()?;
                let object_addr = self.vm.frame_stack.pop_obj_val()?;
                let offset = {
                    let cp = self.vm.frame_stack.cp()?;
                    let nat = cp.get_fieldref(&idx)?.name_and_type_ref()?;
                    self.vm
                        .method_area
                        .get_class_by_id(&self.vm.heap.get_class_id(&object_addr))?
                        .get_field_index_by_nat(nat)?
                };
                self.vm
                    .heap
                    .write_instance_field(object_addr, offset, value)?;
            }
            Instruction::Lookupswitch(switch) => {
                let key = self.vm.frame_stack.pop_int_val()?;
                let pc = *self.vm.frame_stack.pc()?;
                let target_offset = match switch.pairs.binary_search_by_key(&key, |p| p.0) {
                    Ok(i) => switch.pairs[i].1,
                    Err(_) => switch.default_offset,
                };
                let new_pc = Self::branch32(pc, target_offset);
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::TableSwitch(switch) => {
                let index = self.vm.frame_stack.pop_int_val()?;
                let pc = *self.vm.frame_stack.pc()?;
                let target_offset = if index < switch.low || index > switch.high {
                    switch.default_offset
                } else {
                    let idx = (index - switch.low) as usize;
                    switch.offsets[idx]
                };
                let new_pc = Self::branch32(pc, target_offset);
                *self.vm.frame_stack.pc_mut()? = new_pc;
            }
            Instruction::Areturn => {
                let value = self.vm.frame_stack.pop_nullable_ref()?;
                self.pop_frame()?;
                self.vm.frame_stack.push_operand(value)?;
                return Ok(true);
            }
            Instruction::Ireturn => {
                let value = self.vm.frame_stack.pop_int()?;
                self.pop_frame()?;
                self.vm.frame_stack.push_operand(value)?;
                return Ok(true);
            }
            Instruction::Lreturn => {
                let value = self.vm.frame_stack.pop_long()?;
                self.pop_frame()?;
                self.vm.frame_stack.push_operand(value)?;
                return Ok(true);
            }
            Instruction::Freturn => {
                let value = self.vm.frame_stack.pop_float()?;
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
            Instruction::Monitorenter => {
                let _obj = self.vm.frame_stack.pop_obj_val()?;
            }
            Instruction::Monitorexit => {
                let _obj = self.vm.frame_stack.pop_obj_val()?;
            }
            unimpl => throw_exception!(
                UnsupportedOperationException,
                "Instruction {:?} is not implemented",
                unimpl
            )?,
        }
        if need_increase_pc {
            *self.vm.frame_stack.pc_mut()? += instruction_byte_size as usize;
        }
        Ok(false)
    }

    fn run_instance_native_method(
        &mut self,
        method: &Arc<Method>,
        params: Vec<Value>,
    ) -> Result<(), JvmError> {
        let class = method.class()?;

        // TODO: delete
        let debug_msg = format!(
            "instance native method {}{} of class {}",
            method.name(),
            method.descriptor().raw(),
            class.name()
        );
        debug!("Running {debug_msg}");

        // TODO: optimize and don't calculate MethodKey every time
        let method_key = method.build_method_key(&self.vm.string_interner)?;
        let method = self
            .vm
            .native_registry
            .get(&method_key)
            .ok_or(JvmError::NoSuchMethod(format!(
                "{} {} {}",
                class.name(),
                method.name(),
                method.descriptor().raw()
            )))?;

        if let Some(ret_value) = method(&mut self.vm, params.as_slice())? {
            self.vm.frame_stack.push_operand(ret_value)?;
        }

        Ok(())
    }

    fn prepare_method_params(&mut self, method_type: &Method) -> Result<Vec<Value>, JvmError> {
        let params_count = method_type.params_count();
        let mut params = Vec::with_capacity(params_count);
        for _ in 0..params_count {
            params.push(self.vm.frame_stack.pop_operand()?);
        }
        params.reverse();
        Ok(params)
    }

    fn params_to_frame_locals(
        &mut self,
        method: &Arc<Method>,
        params: Vec<Value>,
    ) -> Result<Vec<Option<Value>>, JvmError> {
        let mut locals = vec![None; method.max_locals()?];

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
        Ok(locals)
    }

    pub fn run_instance_method(
        &mut self,
        method: &Arc<Method>,
        params: Vec<Value>,
    ) -> Result<(), JvmError> {
        let class = method.class()?;
        let locals = self.params_to_frame_locals(method, params)?;

        let frame = JavaFrame::new(class.cp().clone(), method.clone(), locals)?;

        self.interpret_method_code(method.instructions()?, frame)?;
        Ok(())
    }

    fn run_abstract_method(
        &mut self,
        abstract_method: &Arc<Method>,
        method_ref: &MethodReference,
        params: Vec<Value>,
    ) -> Result<(), JvmError> {
        let obj_ref = params[0].as_obj_ref()?;
        let class_id = self.vm.heap.get_instance(&obj_ref)?.class_id();
        let class = self.vm.method_area.get_class_by_id(class_id)?.clone(); // FIXME
        let (method, cp) = class.get_virtual_method_and_cp_by_nat(method_ref)?;

        let locals = self.params_to_frame_locals(&method, params)?;
        let frame = JavaFrame::new(cp.clone(), method.clone(), locals)?;

        self.interpret_method_code(method.instructions()?, frame)?;
        Ok(())
    }

    fn run_instance_method_type(
        &mut self,
        method: &Arc<Method>,
        method_ref: &MethodReference,
        params: Vec<Value>,
    ) -> Result<(), JvmError> {
        match method.type_of() {
            MethodType::Java => self.run_instance_method(method, params)?,
            MethodType::Abstract => self.run_abstract_method(method, method_ref, params)?,
            MethodType::Native => self.run_instance_native_method(method, params)?,
        }
        Ok(())
    }

    fn run_static_method(
        &mut self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        params: Vec<Value>,
    ) -> Result<(), JvmError> {
        let locals = self.params_to_frame_locals(method, params)?;
        let frame = JavaFrame::new(class.cp().clone(), method.clone(), locals)?;

        self.interpret_method_code(method.instructions()?, frame)?;

        Ok(())
    }

    fn run_static_native_method(
        &mut self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        params: Vec<Value>,
    ) -> Result<(), JvmError> {
        debug!(
            "Running static native method {}{} of class {}",
            method.name(),
            method.descriptor().raw(),
            class.name()
        );

        let method_key = method.build_method_key(&self.vm.string_interner)?;
        let frame = FrameType::NativeFrame(NativeFrame::new(method.clone()));
        let method = self
            .vm
            .native_registry
            .get(&method_key)
            .ok_or(JvmError::NoSuchMethod(format!(
                "{} {} {}",
                class.name(),
                method.name(),
                method.descriptor().raw()
            )))?;
        self.vm.frame_stack.push_frame(frame)?;
        match method(&mut self.vm, params.as_slice()) {
            Ok(ret_value) => {
                self.vm.frame_stack.pop_native_frame()?;
                if let Some(ret_value) = ret_value {
                    self.vm.frame_stack.push_operand(ret_value)?;
                }
            }
            Err(e) => match e {
                JvmError::JavaException(exception) => {
                    self.allocate_and_throw(exception)?;
                }
                e => return Err(e),
            },
        };
        Ok(())
    }

    fn run_static_method_type(
        &mut self,
        class: &Arc<Class>,
        method: &Arc<Method>,
        params: Vec<Value>,
    ) -> Result<(), JvmError> {
        match method.type_of() {
            MethodType::Java => self.run_static_method(class, method, params),
            MethodType::Native => self.run_static_native_method(class, method, params),
            MethodType::Abstract => panic!("Static method cannot be abstract"),
        }
    }

    //TODO: redisign start method (maybe return Value, maybe take args)
    pub fn start(&mut self) -> Result<(), JvmError> {
        let main_class = self
            .vm
            .method_area
            .get_class_or_load_by_name(&self.vm.config.main_class)?
            .clone();
        self.ensure_initialized(main_class.id())?;
        let main_method = main_class
            .find_main_method()
            .ok_or(JvmError::NoMainClassFound(main_class.name().to_string()))?;
        debug!("Found main method of class {}", main_class.name());
        let instructions = main_method.instructions()?;
        //TODO: handle args
        let frame = JavaFrame::new(
            main_class.cp().clone(),
            main_method.clone(),
            vec![None; main_method.max_locals()?],
        )?;
        debug!("Executing main method...");
        self.interpret_method_code(instructions, frame)?;
        debug!("Main method finished.");

        //TODO: delete, since I don't have return in main and tests for it
        // just to be sure that stack is empty
        assert!(self.pop_frame().is_err());

        Ok(())
    }
}
