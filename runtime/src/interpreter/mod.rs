use crate::error::JvmError;
use crate::heap::HeapRef;
use crate::interpreter::handlers::*;
use crate::interpreter::return_handlers::*;
use crate::keys::{ClassId, FieldKey};
use crate::rt::{ClassLike, JvmClass};
use crate::thread::JavaThreadState;
use crate::vm::Value;
use crate::vm::stack::{FrameType, JavaFrame, NativeFrame};
use crate::{MethodId, VirtualMachine, build_exception, debug_log_instruction, error_log_method};
use common::instruction::Instruction;
use jclass::attribute::method::ExceptionTableEntry;
use std::ops::ControlFlow;
use tracing_log::log::warn;

mod handlers;
mod return_handlers;

pub struct Interpreter;

impl Interpreter {
    fn interpret_instruction(
        thread: &mut JavaThreadState,
        instruction: Instruction,
        vm: &VirtualMachine,
    ) -> Result<ControlFlow<Option<Value>>, JvmError> {
        let is_branch = instruction.is_branch();
        let instr_size = instruction.byte_size();
        warn!("Executing instruction: {:?}", instruction);

        //debug_log_instruction!(&instruction, &thread);

        match instruction {
            Instruction::Athrow => handle_athrow(thread)?,
            Instruction::Aaload => handle_aaload(thread, vm)?,
            Instruction::Aastore => handle_aastore(thread, vm)?,
            Instruction::Bastore => handle_bastore(thread, vm)?,
            Instruction::Iaload => handle_iaload(thread, vm)?,
            Instruction::Caload => handle_caload(thread, vm)?,
            Instruction::Baload => handle_baload(thread, vm)?,
            Instruction::Checkcast(_idx) => handle_checkcast(thread)?,
            Instruction::AconstNull => handle_aconst_null(thread)?,
            Instruction::Aload0 => handle_aload0(thread)?,
            Instruction::Aload1 => handle_aload1(thread)?,
            Instruction::Aload2 => handle_aload2(thread)?,
            Instruction::Aload3 => handle_aload3(thread)?,
            Instruction::Aload(pos) => handle_aload(thread, pos)?,
            Instruction::Anewarray(idx) => handle_anewarray(thread, vm, idx)?,
            Instruction::ArrayLength => handle_arraylength(thread, vm)?,
            Instruction::Astore0 => handle_astore0(thread)?,
            Instruction::Astore1 => handle_astore1(thread)?,
            Instruction::Astore2 => handle_astore2(thread)?,
            Instruction::Astore3 => handle_astore3(thread)?,
            Instruction::Astore(pos) => handle_astore(thread, pos)?,
            Instruction::Bipush(value) => handle_bipush(thread, value)?,
            Instruction::Castore => handle_castore(thread, vm)?,
            Instruction::Dadd => handle_dadd(thread)?,
            Instruction::Ddiv => handle_ddiv(thread)?,
            Instruction::Dcmpl => handle_dcmpl(thread)?,
            Instruction::Dcmpg => handle_dcmpg(thread)?,
            Instruction::Dconst0 => handle_dconst0(thread)?,
            Instruction::Dconst1 => handle_dconst1(thread)?,
            Instruction::Dload0 => handle_dload0(thread)?,
            Instruction::Dload1 => handle_dload1(thread)?,
            Instruction::Dload2 => handle_dload2(thread)?,
            Instruction::Dload3 => handle_dload3(thread)?,
            Instruction::Dload(n) => handle_dload(thread, n)?,
            Instruction::Dmul => handle_dmul(thread)?,
            Instruction::Dstore(n) => handle_dstore(thread, n)?,
            Instruction::Dup => handle_dup(thread)?,
            Instruction::Dup2 => handle_dup2(thread)?,
            Instruction::DupX1 => handle_dup_x1(thread)?,
            Instruction::Fcmpl => handle_fcmpl(thread)?,
            Instruction::Fcmpg => handle_fcmpg(thread)?,
            Instruction::Fconst0 => handle_fconst0(thread)?,
            Instruction::Fconst1 => handle_fconst1(thread)?,
            Instruction::Fload0 => handle_fload0(thread)?,
            Instruction::Fload1 => handle_fload1(thread)?,
            Instruction::Fload2 => handle_fload2(thread)?,
            Instruction::Fload3 => handle_fload3(thread)?,
            Instruction::Fload(n) => handle_fload(thread, n)?,
            Instruction::Fstore0 => handle_fstore0(thread)?,
            Instruction::Fstore1 => handle_fstore1(thread)?,
            Instruction::Fstore2 => handle_fstore2(thread)?,
            Instruction::Fstore3 => handle_fstore3(thread)?,
            Instruction::Fstore(n) => handle_fstore(thread, n)?,
            Instruction::Getfield(idx) => handle_getfield(thread, vm, idx)?,
            Instruction::Getstatic(idx) => handle_getstatic(thread, vm, idx)?,
            Instruction::Goto(offset) => handle_goto(thread, offset)?,
            Instruction::Iadd => handle_iadd(thread)?,
            Instruction::Iconst0 => handle_iconst0(thread)?,
            Instruction::Iconst1 => handle_iconst1(thread)?,
            Instruction::Iconst2 => handle_iconst2(thread)?,
            Instruction::Iconst3 => handle_iconst3(thread)?,
            Instruction::Iconst4 => handle_iconst4(thread)?,
            Instruction::Iconst5 => handle_iconst5(thread)?,
            Instruction::IconstM1 => handle_iconst_m1(thread)?,
            Instruction::Idiv => handle_idiv(thread)?,
            Instruction::IfEq(offset) => handle_ifeq(thread, offset, instr_size)?,
            Instruction::IfGe(offset) => handle_ifge(thread, offset, instr_size)?,
            Instruction::IfGt(offset) => handle_ifgt(thread, offset, instr_size)?,
            Instruction::Lcmp => handle_lcmp(thread)?,
            Instruction::Lconst0 => handle_lconst0(thread)?,
            Instruction::Lconst1 => handle_lconst1(thread)?,
            Instruction::Lookupswitch(switch) => handle_lookupswitch(thread, switch)?,
            Instruction::Ifnull(offset) => handle_ifnull(thread, offset, instr_size)?,
            Instruction::IfIcmplt(offset) => handle_ificmplt(thread, offset, instr_size)?,
            Instruction::IfLe(offset) => handle_ifle(thread, offset, instr_size)?,
            Instruction::IfLt(offset) => handle_iflt(thread, offset, instr_size)?,
            Instruction::IfAcmpEq(offset) => handle_ifacmpeq(thread, offset, instr_size)?,
            Instruction::IfAcmpNe(offset) => handle_ifacmpne(thread, offset, instr_size)?,
            Instruction::IfIcmpne(offset) => handle_ificmpne(thread, offset, instr_size)?,
            Instruction::IfIcmpge(offset) => handle_ificmpge(thread, offset, instr_size)?,
            Instruction::IfIcmpgt(offset) => handle_ificmpgt(thread, offset, instr_size)?,
            Instruction::IfIcmpeq(offset) => handle_ificmpeq(thread, offset, instr_size)?,
            Instruction::IfIcmple(offset) => handle_ificmple(thread, offset, instr_size)?,
            Instruction::Ifnonnull(offset) => handle_ifnonnull(thread, offset, instr_size)?,
            Instruction::IfNe(offset) => handle_ifne(thread, offset, instr_size)?,
            Instruction::Iload0 => handle_iload0(thread)?,
            Instruction::Iload1 => handle_iload1(thread)?,
            Instruction::Iload2 => handle_iload2(thread)?,
            Instruction::Iload3 => handle_iload3(thread)?,
            Instruction::Iload(pos) => handle_iload(thread, pos)?,
            Instruction::InvokeVirtual(idx) => handle_invokevirtual(thread, vm, idx)?,
            Instruction::Instanceof(idx) => handle_instanceof(thread, vm, idx)?,
            Instruction::Fmul => handle_fmul(thread)?,
            Instruction::Fdiv => handle_fdiv(thread)?,
            Instruction::Irem => handle_irem(thread)?,
            Instruction::Ladd => handle_ladd(thread)?,
            Instruction::Ldiv => handle_ldiv(thread)?,
            Instruction::Lmul => handle_lmul(thread)?,
            Instruction::Lrem => handle_lrem(thread)?,
            Instruction::Land => handle_land(thread)?,
            Instruction::Lor => handle_lor(thread)?,
            Instruction::Lxor => handle_lxor(thread)?,
            Instruction::Iand => handle_iand(thread)?,
            Instruction::Ior => handle_ior(thread)?,
            Instruction::Ixor => handle_ixor(thread)?,
            Instruction::L2i => handle_l2i(thread)?,
            Instruction::L2f => handle_l2f(thread)?,
            Instruction::D2i => handle_d2i(thread)?,
            Instruction::D2l => handle_d2l(thread)?,
            Instruction::F2i => handle_f2i(thread)?,
            Instruction::F2d => handle_f2d(thread)?,
            Instruction::Ineg => handle_ineg(thread)?,
            Instruction::I2s => handle_i2s(thread)?,
            Instruction::I2c => handle_i2c(thread)?,
            Instruction::I2l => handle_i2l(thread)?,
            Instruction::I2f => handle_i2f(thread)?,
            Instruction::I2d => handle_i2d(thread)?,
            Instruction::I2b => handle_i2b(thread)?,
            Instruction::Istore0 => handle_istore0(thread)?,
            Instruction::Istore1 => handle_istore1(thread)?,
            Instruction::Istore2 => handle_istore2(thread)?,
            Instruction::Istore3 => handle_istore3(thread)?,
            Instruction::Istore(idx) => handle_istore(thread, idx)?,
            Instruction::Isub => handle_isub(thread)?,
            Instruction::Imul => handle_imul(thread)?,
            Instruction::Iinc(index, const_val) => handle_iinc(thread, index, const_val)?,
            Instruction::Ldc(idx) | Instruction::LdcW(idx) | Instruction::Ldc2W(idx) => {
                handle_ldc_ldcw_ldc2w(thread, vm, idx)?
            }
            Instruction::New(idx) => handle_new(thread, vm, idx)?,
            Instruction::Newarray(array_type) => handle_newarray(thread, vm, array_type)?,
            Instruction::Pop => handle_pop(thread)?,
            Instruction::Putfield(idx) => handle_putfield(thread, vm, idx)?,
            Instruction::Putstatic(idx) => handle_putstatic(thread, vm, idx)?,
            Instruction::InvokeInterface(idx, count) => {
                handle_invokeinterface(thread, vm, idx, count)?
            }
            Instruction::InvokeSpecial(idx) => handle_invokespecial(thread, vm, idx)?,
            Instruction::InvokeStatic(idx) => handle_invokestatic(thread, vm, idx)?,
            Instruction::InvokeDynamic(idx) => handle_invokedynamic(thread, vm, idx)?,
            Instruction::Iushr => handle_iushr(thread)?,
            Instruction::Lload0 => handle_lload0(thread)?,
            Instruction::Lload1 => handle_lload1(thread)?,
            Instruction::Lload2 => handle_lload2(thread)?,
            Instruction::Lload3 => handle_lload3(thread)?,
            Instruction::Lload(pos) => handle_lload(thread, pos)?,
            Instruction::Lshl => handle_lshl(thread)?,
            Instruction::Lshr => handle_lshr(thread)?,
            Instruction::Lushr => handle_lushr(thread)?,
            Instruction::Lstore0 => handle_lstore0(thread)?,
            Instruction::Lstore1 => handle_lstore1(thread)?,
            Instruction::Lstore2 => handle_lstore2(thread)?,
            Instruction::Lstore3 => handle_lstore3(thread)?,
            Instruction::Lstore(idx) => handle_lstore(thread, idx)?,
            Instruction::Lsub => handle_lsub(thread)?,
            Instruction::Iastore => handle_iastore(thread, vm)?,
            Instruction::Ishl => handle_ishl(thread)?,
            Instruction::Ishr => handle_ishr(thread)?,
            Instruction::Saload => handle_saload(thread, vm)?,
            Instruction::Sastore => handle_sastore(thread, vm)?,
            Instruction::Sipush(value) => handle_sipush(thread, value)?,
            Instruction::TableSwitch(switch) => handle_tableswitch(thread, switch)?,
            Instruction::Monitorenter => handle_monitorenter(thread)?,
            Instruction::Monitorexit => handle_monitorexit(thread)?,
            Instruction::Return => {
                return Ok(ControlFlow::Break(None));
            }
            Instruction::Dreturn => {
                let ret_value = handle_dreturn(thread)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Ireturn => {
                let ret_value = handle_ireturn(thread)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Areturn => {
                let ret_value = handle_areturn(thread)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Lreturn => {
                let ret_value = handle_lreturn(thread)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Freturn => {
                let ret_value = handle_freturn(thread)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            instruction => unimplemented!("instruction {:?}", instruction),
        }

        if !is_branch {
            thread.stack.cur_java_frame_mut()?.increment_pc(instr_size);
        }
        Ok(ControlFlow::Continue(()))
    }

    //TODO: need to move it probaly to vm, and refactor

    fn prepare_method_args(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        vm: &VirtualMachine,
    ) -> Result<Vec<Value>, JvmError> {
        let mut args_count = vm
            .method_area_read()
            .get_method_descriptor_by_method_id(&method_id)
            .params
            .len();
        if !vm.method_area_read().get_method(&method_id).is_static() {
            args_count += 1;
        }
        // TODO: I saw somewhere a data structure with fixed capacity, that can avoid heap allocation
        let mut args = Vec::with_capacity(args_count);
        for _ in 0..args_count {
            args.push(thread.stack.pop_operand()?);
        }
        args.reverse();
        Ok(args)
    }

    fn pc_in_range(pc: usize, entry: &ExceptionTableEntry) -> bool {
        pc >= entry.start_pc as usize && pc < entry.end_pc as usize
    }

    fn is_exception_caught(
        vm: &VirtualMachine,
        entry: &ExceptionTableEntry,
        method_id: &MethodId,
        java_exception: HeapRef,
    ) -> Result<bool, JvmError> {
        let catch_type = entry.catch_type;

        if catch_type == 0 {
            return Ok(true);
        }

        let exception_class_id = vm.heap_read().get_class_id(java_exception)?;
        let catch_type_sym = vm
            .method_area_read()
            .get_cp_by_method_id(method_id)?
            .get_class_sym(&catch_type, vm.interner())?;

        Ok(vm
            .method_area_read()
            .instance_of(exception_class_id, catch_type_sym))
    }

    fn find_exception_handler(
        vm: &VirtualMachine,
        method_id: &MethodId,
        java_exception: HeapRef,
        thread: &mut JavaThreadState,
    ) -> Result<bool, JvmError> {
        let pc = thread.stack.pc()?;
        let ma = vm.method_area_read();
        let exception_table = ma.get_method(method_id).get_exception_table()?;

        for entry in exception_table.iter() {
            if !Self::pc_in_range(pc, entry) {
                continue;
            }

            if Self::is_exception_caught(vm, entry, method_id, java_exception)? {
                let handler_pc = entry.handler_pc as usize;
                let stack = &mut thread.stack;
                stack.push_operand(Value::Ref(java_exception))?;
                *stack.pc_mut()? = handler_pc;
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn interpret_method(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        vm: &VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let code_ptr = vm.method_area_read().get_method(&method_id).get_code()? as *const [u8];
        loop {
            // SAFETY: code_ptr is valid as long as method exists in method area (always)
            // need to use pointer to avoid borrow checker issues
            let code = unsafe { &*code_ptr };
            let pc = thread.stack.pc()?;
            let instruction = Instruction::new_at(code, pc)?;

            match Self::interpret_instruction(thread, instruction, vm) {
                Ok(flow) => {
                    if let ControlFlow::Break(res) = flow {
                        return Ok(res);
                    }
                }
                Err(e) => {
                    let java_exception = match e {
                        JvmError::JavaException(exception) => {
                            vm.map_rust_error_to_java_exception(thread, exception)
                        }
                        JvmError::JavaExceptionThrown(exception_ref) => Ok(exception_ref),
                        // TODO: this errors are not mapped yet or happened during mapping to java exception
                        e => Err(e),
                    }?;
                    if thread.stack.cur_frame()?.is_native() {
                        thread.stack.pop_native_frame()?;
                    }
                    if !Self::find_exception_handler(vm, &method_id, java_exception, thread)? {
                        thread.stack.pop_java_frame()?;
                        return Err(JvmError::JavaExceptionThrown(java_exception));
                    }
                }
            }
        }
    }

    fn invoke_native_method(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let is_static = {
            let ma = vm.method_area_read();
            ma.get_method(&method_id).is_static()
        };
        let clone_desc = vm.br.clone_desc;
        let object_class_sym = vm.br.java_lang_object_sym;
        let mut method_key = vm
            .method_area_read()
            .build_fully_qualified_native_method_key(&method_id);
        // native instance method of array special handling (for now, only Object.clone)
        if !is_static
            && vm.heap_read().is_array(args[0].as_obj_ref()?)?
            && method_key.name == vm.br.clone_sym
            && method_key.desc == clone_desc
            && method_key.class == Some(object_class_sym)
        {
            method_key.class = None;
        }
        let frame = NativeFrame::new(method_id);
        thread.stack.push_frame(FrameType::NativeFrame(frame))?;
        let native = vm.native_registry.get(&method_key).ok_or(build_exception!(
            UnsatisfiedLinkError,
            vm.pretty_method_not_found_message(&method_id)
        ))?;
        let native_res = match native(vm, thread, args.as_slice()) {
            Ok(res) => res,
            Err(e) => {
                error_log_method!(
                    &method_id,
                    &e,
                    "👹👹👹 Java exception thrown in native method"
                );
                return Err(e);
            }
        };
        thread.stack.pop_native_frame()?;
        Ok(native_res)
    }

    fn invoke_java_method(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let (max_stack, max_locals) = vm
            .method_area_read()
            .get_method(&method_id)
            .get_frame_attributes()?;
        let frame = JavaFrame::new(method_id, max_stack, max_locals, args);
        thread.stack.push_frame(FrameType::JavaFrame(frame))?;
        let method_ret = Self::interpret_method(thread, method_id, vm);
        if let Err(e) = &method_ret {
            error_log_method!(
                &method_id,
                e,
                "👹👹👹 Java exception thrown in interpreted method"
            );
        }
        let method_ret = method_ret?;
        thread.stack.pop_java_frame()?;
        Ok(method_ret)
    }

    fn invoke_method_core(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let is_native = {
            let ma = vm.method_area_read();
            ma.get_method(&method_id).is_native()
        };
        if is_native {
            Self::invoke_native_method(thread, method_id, args, vm)
        } else {
            Self::invoke_java_method(thread, method_id, args, vm)
        }
    }

    fn invoke_method_internal(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &VirtualMachine,
    ) -> Result<(), JvmError> {
        let method_ret = Self::invoke_method_core(thread, method_id, args, vm)?;
        if let Some(ret) = method_ret {
            thread.stack.push_operand(ret)?;
        }
        Ok(())
    }

    fn interface_needs_initialization(
        interface_id: ClassId,
        vm: &VirtualMachine,
    ) -> Result<bool, JvmError> {
        let has_clinit = {
            let ma = vm.method_area_read();
            let interface = ma.get_interface_class(&interface_id)?;
            interface.has_clinit()
        };

        Ok(has_clinit) //TODO: || interface.has_non_constant_static_fields()?
    }

    fn run_clinit_if_exists(
        thread: &mut JavaThreadState,
        class_id: ClassId,
        vm: &VirtualMachine,
    ) -> Result<(), JvmError> {
        let ma = vm.method_area_read();
        if let Some(&clinit_method_id) = ma.get_class_like(&class_id)?.get_clinit_method_id() {
            drop(ma);
            Self::invoke_method_internal(thread, clinit_method_id, vec![], vm)?;
        }

        Ok(())
    }

    pub fn ensure_initialized(
        thread: &mut JavaThreadState,
        class_id: Option<ClassId>,
        vm: &VirtualMachine,
    ) -> Result<(), JvmError> {
        let Some(class_id) = class_id else {
            return Ok(());
        };

        {
            let ma = vm.method_area_read();
            let class = ma.get_class_like(&class_id)?;

            if class.is_initialized_or_initializing() {
                return Ok(());
            }

            class.set_initializing();
        }

        let (is_instance, is_interface) = {
            let ma = vm.method_area_read();
            let jvm_class = ma.get_class(&class_id);
            match jvm_class {
                JvmClass::Instance(_) => (true, false),
                JvmClass::Interface(_) => (false, true),
                _ => (false, false),
            }
        };

        if is_instance {
            let super_id = {
                let ma = vm.method_area_read();
                let inst_class = ma.get_instance_class(&class_id)?;
                inst_class.get_super()
            };
            if let Some(super_id) = super_id {
                Self::ensure_initialized(thread, Some(super_id), vm)?;
            }
            let interfaces = vm
                .method_area_read()
                .get_instance_class(&class_id)?
                .get_interfaces()?
                .clone(); // TODO: avoid clone?
            for interface_id in interfaces {
                if Self::interface_needs_initialization(interface_id, vm)? {
                    Self::ensure_initialized(thread, Some(interface_id), vm)?;
                }
            }

            Self::run_clinit_if_exists(thread, class_id, vm)?;

            let cur_class_name = vm.method_area_read().get_instance_class(&class_id)?.name();

            //TODO: stub
            if vm.interner().resolve(&cur_class_name) == "jdk/internal/access/SharedSecrets" {
                warn!(
                    "TODO: Stub: Setting jdk/internal/access/SharedSecrets javaLangRefAccess to non-null value, to avoid NPEs"
                );
                let ref_access_fk = FieldKey {
                    name: vm.interner().get_or_intern("javaLangRefAccess"),
                    desc: vm
                        .interner()
                        .get_or_intern("Ljdk/internal/access/JavaLangRefAccess;"),
                };
                vm.method_area_read()
                    .get_instance_class(&class_id)?
                    .set_static_field_value(&ref_access_fk, Value::Ref(0))?;
            }
        } else if is_interface {
            let interfaces = vm
                .method_area_read()
                .get_interface_class(&class_id)?
                .get_interfaces()?
                .clone(); // TODO: avoid clone?
            for super_interface_id in interfaces {
                if Self::interface_needs_initialization(super_interface_id, vm)? {
                    Self::ensure_initialized(thread, Some(super_interface_id), vm)?;
                }
            }

            Self::run_clinit_if_exists(thread, class_id, vm)?;
        }

        vm.method_area_read()
            .get_class_like(&class_id)?
            .set_initialized();
        Ok(())
    }

    pub fn invoke_instance_method(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        vm: &VirtualMachine,
        args: Vec<Value>,
    ) -> Result<Option<Value>, JvmError> {
        //TODO: do I need to check that args[0] is not null?
        Self::invoke_method_core(thread, method_id, args, vm)
    }

    pub fn invoke_static_method(
        thread: &mut JavaThreadState,
        method_id: MethodId,
        vm: &VirtualMachine,
        args: Vec<Value>,
    ) -> Result<(), JvmError> {
        let class_id = vm.method_area_read().get_method(&method_id).class_id();
        Self::ensure_initialized(thread, Some(class_id), vm)?;
        Self::invoke_method_internal(thread, method_id, args, vm)?;
        Ok(())
    }
}
