use crate::error::JvmError;
use crate::heap::HeapRef;
use crate::interpreter::handlers::*;
use crate::interpreter::return_handlers::*;
use crate::keys::{ClassId, FieldKey};
use crate::rt::{ClassLike, JvmClass};
use crate::vm::Value;
use crate::vm::stack::{FrameType, JavaFrame, NativeFrame};
use crate::{
    MethodId, ThreadId, VirtualMachine, build_exception, debug_log_instruction, error_log_method,
};
use common::instruction::Instruction;
use jclass::attribute::method::ExceptionTableEntry;
use std::ops::ControlFlow;
use tracing_log::log::warn;

mod handlers;
mod return_handlers;

pub struct Interpreter;

impl Interpreter {
    fn interpret_instruction(
        thread_id: ThreadId,
        instruction: Instruction,
        vm: &mut VirtualMachine,
    ) -> Result<ControlFlow<Option<Value>>, JvmError> {
        let is_branch = instruction.is_branch();
        let instr_size = instruction.byte_size();

        //debug_log_instruction!(&instruction, &thread_id);

        match instruction {
            Instruction::Athrow => handle_athrow(thread_id, vm)?,
            Instruction::Aaload => handle_aaload(thread_id, vm)?,
            Instruction::Aastore => handle_aastore(thread_id, vm)?,
            Instruction::Bastore => handle_bastore(thread_id, vm)?,
            Instruction::Iaload => handle_iaload(thread_id, vm)?,
            Instruction::Caload => handle_caload(thread_id, vm)?,
            Instruction::Baload => handle_baload(thread_id, vm)?,
            Instruction::Checkcast(_idx) => handle_checkcast(thread_id, vm)?,
            Instruction::AconstNull => handle_aconst_null(thread_id, vm)?,
            Instruction::Aload0 => handle_aload0(thread_id, vm)?,
            Instruction::Aload1 => handle_aload1(thread_id, vm)?,
            Instruction::Aload2 => handle_aload2(thread_id, vm)?,
            Instruction::Aload3 => handle_aload3(thread_id, vm)?,
            Instruction::Aload(pos) => handle_aload(thread_id, vm, pos)?,
            Instruction::Anewarray(idx) => handle_anewarray(thread_id, vm, idx)?,
            Instruction::ArrayLength => handle_arraylength(thread_id, vm)?,
            Instruction::Astore0 => handle_astore0(thread_id, vm)?,
            Instruction::Astore1 => handle_astore1(thread_id, vm)?,
            Instruction::Astore2 => handle_astore2(thread_id, vm)?,
            Instruction::Astore3 => handle_astore3(thread_id, vm)?,
            Instruction::Astore(pos) => handle_astore(thread_id, vm, pos)?,
            Instruction::Bipush(value) => handle_bipush(thread_id, vm, value)?,
            Instruction::Castore => handle_castore(thread_id, vm)?,
            Instruction::Dadd => handle_dadd(thread_id, vm)?,
            Instruction::Ddiv => handle_ddiv(thread_id, vm)?,
            Instruction::Dcmpl => handle_dcmpl(thread_id, vm)?,
            Instruction::Dcmpg => handle_dcmpg(thread_id, vm)?,
            Instruction::Dconst0 => handle_dconst0(thread_id, vm)?,
            Instruction::Dconst1 => handle_dconst1(thread_id, vm)?,
            Instruction::Dload0 => handle_dload0(thread_id, vm)?,
            Instruction::Dload1 => handle_dload1(thread_id, vm)?,
            Instruction::Dload2 => handle_dload2(thread_id, vm)?,
            Instruction::Dload3 => handle_dload3(thread_id, vm)?,
            Instruction::Dload(n) => handle_dload(thread_id, vm, n)?,
            Instruction::Dmul => handle_dmul(thread_id, vm)?,
            Instruction::Dstore(n) => handle_dstore(thread_id, vm, n)?,
            Instruction::Dup => handle_dup(thread_id, vm)?,
            Instruction::Dup2 => handle_dup2(thread_id, vm)?,
            Instruction::DupX1 => handle_dup_x1(thread_id, vm)?,
            Instruction::Fcmpl => handle_fcmpl(thread_id, vm)?,
            Instruction::Fcmpg => handle_fcmpg(thread_id, vm)?,
            Instruction::Fconst0 => handle_fconst0(thread_id, vm)?,
            Instruction::Fconst1 => handle_fconst1(thread_id, vm)?,
            Instruction::Fload0 => handle_fload0(thread_id, vm)?,
            Instruction::Fload1 => handle_fload1(thread_id, vm)?,
            Instruction::Fload2 => handle_fload2(thread_id, vm)?,
            Instruction::Fload3 => handle_fload3(thread_id, vm)?,
            Instruction::Fload(n) => handle_fload(thread_id, vm, n)?,
            Instruction::Fstore0 => handle_fstore0(thread_id, vm)?,
            Instruction::Fstore1 => handle_fstore1(thread_id, vm)?,
            Instruction::Fstore2 => handle_fstore2(thread_id, vm)?,
            Instruction::Fstore3 => handle_fstore3(thread_id, vm)?,
            Instruction::Fstore(n) => handle_fstore(thread_id, vm, n)?,
            Instruction::Getfield(idx) => handle_getfield(thread_id, vm, idx)?,
            Instruction::Getstatic(idx) => handle_getstatic(thread_id, vm, idx)?,
            Instruction::Goto(offset) => handle_goto(thread_id, vm, offset)?,
            Instruction::Iadd => handle_iadd(thread_id, vm)?,
            Instruction::Iconst0 => handle_iconst0(thread_id, vm)?,
            Instruction::Iconst1 => handle_iconst1(thread_id, vm)?,
            Instruction::Iconst2 => handle_iconst2(thread_id, vm)?,
            Instruction::Iconst3 => handle_iconst3(thread_id, vm)?,
            Instruction::Iconst4 => handle_iconst4(thread_id, vm)?,
            Instruction::Iconst5 => handle_iconst5(thread_id, vm)?,
            Instruction::IconstM1 => handle_iconst_m1(thread_id, vm)?,
            Instruction::Idiv => handle_idiv(thread_id, vm)?,
            Instruction::IfEq(offset) => handle_ifeq(thread_id, vm, offset, instr_size)?,
            Instruction::IfGe(offset) => handle_ifge(thread_id, vm, offset, instr_size)?,
            Instruction::IfGt(offset) => handle_ifgt(thread_id, vm, offset, instr_size)?,
            Instruction::Lcmp => handle_lcmp(thread_id, vm)?,
            Instruction::Lconst0 => handle_lconst0(thread_id, vm)?,
            Instruction::Lconst1 => handle_lconst1(thread_id, vm)?,
            Instruction::Lookupswitch(switch) => handle_lookupswitch(thread_id, vm, switch)?,
            Instruction::Ifnull(offset) => handle_ifnull(thread_id, vm, offset, instr_size)?,
            Instruction::IfIcmplt(offset) => handle_ificmplt(thread_id, vm, offset, instr_size)?,
            Instruction::IfLe(offset) => handle_ifle(thread_id, vm, offset, instr_size)?,
            Instruction::IfLt(offset) => handle_iflt(thread_id, vm, offset, instr_size)?,
            Instruction::IfAcmpEq(offset) => handle_ifacmpeq(thread_id, vm, offset, instr_size)?,
            Instruction::IfAcmpNe(offset) => handle_ifacmpne(thread_id, vm, offset, instr_size)?,
            Instruction::IfIcmpne(offset) => handle_ificmpne(thread_id, vm, offset, instr_size)?,
            Instruction::IfIcmpge(offset) => handle_ificmpge(thread_id, vm, offset, instr_size)?,
            Instruction::IfIcmpgt(offset) => handle_ificmpgt(thread_id, vm, offset, instr_size)?,
            Instruction::IfIcmpeq(offset) => handle_ificmpeq(thread_id, vm, offset, instr_size)?,
            Instruction::IfIcmple(offset) => handle_ificmple(thread_id, vm, offset, instr_size)?,
            Instruction::Ifnonnull(offset) => handle_ifnonnull(thread_id, vm, offset, instr_size)?,
            Instruction::IfNe(offset) => handle_ifne(thread_id, vm, offset, instr_size)?,
            Instruction::Iload0 => handle_iload0(thread_id, vm)?,
            Instruction::Iload1 => handle_iload1(thread_id, vm)?,
            Instruction::Iload2 => handle_iload2(thread_id, vm)?,
            Instruction::Iload3 => handle_iload3(thread_id, vm)?,
            Instruction::Iload(pos) => handle_iload(thread_id, vm, pos)?,
            Instruction::InvokeVirtual(idx) => handle_invokevirtual(thread_id, vm, idx)?,
            Instruction::Instanceof(idx) => handle_instanceof(thread_id, vm, idx)?,
            Instruction::Fmul => handle_fmul(thread_id, vm)?,
            Instruction::Fdiv => handle_fdiv(thread_id, vm)?,
            Instruction::Irem => handle_irem(thread_id, vm)?,
            Instruction::Ladd => handle_ladd(thread_id, vm)?,
            Instruction::Ldiv => handle_ldiv(thread_id, vm)?,
            Instruction::Lmul => handle_lmul(thread_id, vm)?,
            Instruction::Lrem => handle_lrem(thread_id, vm)?,
            Instruction::Land => handle_land(thread_id, vm)?,
            Instruction::Lor => handle_lor(thread_id, vm)?,
            Instruction::Lxor => handle_lxor(thread_id, vm)?,
            Instruction::Iand => handle_iand(thread_id, vm)?,
            Instruction::Ior => handle_ior(thread_id, vm)?,
            Instruction::Ixor => handle_ixor(thread_id, vm)?,
            Instruction::L2i => handle_l2i(thread_id, vm)?,
            Instruction::L2f => handle_l2f(thread_id, vm)?,
            Instruction::D2i => handle_d2i(thread_id, vm)?,
            Instruction::D2l => handle_d2l(thread_id, vm)?,
            Instruction::F2i => handle_f2i(thread_id, vm)?,
            Instruction::F2d => handle_f2d(thread_id, vm)?,
            Instruction::Ineg => handle_ineg(thread_id, vm)?,
            Instruction::I2s => handle_i2s(thread_id, vm)?,
            Instruction::I2c => handle_i2c(thread_id, vm)?,
            Instruction::I2l => handle_i2l(thread_id, vm)?,
            Instruction::I2f => handle_i2f(thread_id, vm)?,
            Instruction::I2d => handle_i2d(thread_id, vm)?,
            Instruction::I2b => handle_i2b(thread_id, vm)?,
            Instruction::Istore0 => handle_istore0(thread_id, vm)?,
            Instruction::Istore1 => handle_istore1(thread_id, vm)?,
            Instruction::Istore2 => handle_istore2(thread_id, vm)?,
            Instruction::Istore3 => handle_istore3(thread_id, vm)?,
            Instruction::Istore(idx) => handle_istore(thread_id, vm, idx)?,
            Instruction::Isub => handle_isub(thread_id, vm)?,
            Instruction::Imul => handle_imul(thread_id, vm)?,
            Instruction::Iinc(index, const_val) => handle_iinc(thread_id, vm, index, const_val)?,
            Instruction::Ldc(idx) | Instruction::LdcW(idx) | Instruction::Ldc2W(idx) => {
                handle_ldc_ldcw_ldc2w(thread_id, vm, idx)?
            }
            Instruction::New(idx) => handle_new(thread_id, vm, idx)?,
            Instruction::Newarray(array_type) => handle_newarray(thread_id, vm, array_type)?,
            Instruction::Pop => handle_pop(thread_id, vm)?,
            Instruction::Putfield(idx) => handle_putfield(thread_id, vm, idx)?,
            Instruction::Putstatic(idx) => handle_putstatic(thread_id, vm, idx)?,
            Instruction::InvokeInterface(idx, count) => {
                handle_invokeinterface(thread_id, vm, idx, count)?
            }
            Instruction::InvokeSpecial(idx) => handle_invokespecial(thread_id, vm, idx)?,
            Instruction::InvokeStatic(idx) => handle_invokestatic(thread_id, vm, idx)?,
            Instruction::InvokeDynamic(idx) => handle_invokedynamic(thread_id, vm, idx)?,
            Instruction::Iushr => handle_iushr(thread_id, vm)?,
            Instruction::Lload0 => handle_lload0(thread_id, vm)?,
            Instruction::Lload1 => handle_lload1(thread_id, vm)?,
            Instruction::Lload2 => handle_lload2(thread_id, vm)?,
            Instruction::Lload3 => handle_lload3(thread_id, vm)?,
            Instruction::Lload(pos) => handle_lload(thread_id, vm, pos)?,
            Instruction::Lshl => handle_lshl(thread_id, vm)?,
            Instruction::Lshr => handle_lshr(thread_id, vm)?,
            Instruction::Lushr => handle_lushr(thread_id, vm)?,
            Instruction::Lstore0 => handle_lstore0(thread_id, vm)?,
            Instruction::Lstore1 => handle_lstore1(thread_id, vm)?,
            Instruction::Lstore2 => handle_lstore2(thread_id, vm)?,
            Instruction::Lstore3 => handle_lstore3(thread_id, vm)?,
            Instruction::Lstore(idx) => handle_lstore(thread_id, vm, idx)?,
            Instruction::Lsub => handle_lsub(thread_id, vm)?,
            Instruction::Iastore => handle_iastore(thread_id, vm)?,
            Instruction::Ishl => handle_ishl(thread_id, vm)?,
            Instruction::Ishr => handle_ishr(thread_id, vm)?,
            Instruction::Saload => handle_saload(thread_id, vm)?,
            Instruction::Sastore => handle_sastore(thread_id, vm)?,
            Instruction::Sipush(value) => handle_sipush(thread_id, vm, value)?,
            Instruction::TableSwitch(switch) => handle_tableswitch(thread_id, vm, switch)?,
            Instruction::Monitorenter => handle_monitorenter(thread_id, vm)?,
            Instruction::Monitorexit => handle_monitorexit(thread_id, vm)?,
            Instruction::Return => {
                return Ok(ControlFlow::Break(None));
            }
            Instruction::Dreturn => {
                let ret_value = handle_dreturn(thread_id, vm)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Ireturn => {
                let ret_value = handle_ireturn(thread_id, vm)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Areturn => {
                let ret_value = handle_areturn(thread_id, vm)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Lreturn => {
                let ret_value = handle_lreturn(thread_id, vm)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            Instruction::Freturn => {
                let ret_value = handle_freturn(thread_id, vm)?;
                return Ok(ControlFlow::Break(Some(ret_value)));
            }
            instruction => unimplemented!("instruction {:?}", instruction),
        }

        if !is_branch {
            vm.get_stack_mut(&thread_id)?
                .cur_java_frame_mut()?
                .increment_pc(instr_size);
        }
        Ok(ControlFlow::Continue(()))
    }

    //TODO: need to move it probaly to vm, and refactor

    fn prepare_method_args(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
    ) -> Result<Vec<Value>, JvmError> {
        let mut args_count = vm
            .method_area
            .get_method_descriptor_by_method_id(&method_id)
            .params
            .len();
        if !vm.method_area.get_method(&method_id).is_static() {
            args_count += 1;
        }
        // TODO: I saw somewhere a data structure with fixed capacity, that can avoid heap allocation
        let mut args = Vec::with_capacity(args_count);
        for _ in 0..args_count {
            args.push(vm.get_stack_mut(&thread_id)?.pop_operand()?);
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

        let exception_class_id = vm.heap.get_class_id(java_exception)?;
        let catch_type_sym = vm
            .method_area
            .get_cp_by_method_id(method_id)?
            .get_class_sym(&catch_type, vm.interner())?;

        Ok(vm
            .method_area
            .instance_of(exception_class_id, catch_type_sym))
    }

    fn find_exception_handler(
        vm: &mut VirtualMachine,
        method_id: &MethodId,
        java_exception: HeapRef,
        thread_id: &ThreadId,
    ) -> Result<bool, JvmError> {
        let pc = vm.get_stack_mut(thread_id)?.pc()?;
        let exception_table = vm.method_area.get_method(method_id).get_exception_table()?;

        for entry in exception_table.iter() {
            if !Self::pc_in_range(pc, entry) {
                continue;
            }

            if Self::is_exception_caught(vm, entry, method_id, java_exception)? {
                let handler_pc = entry.handler_pc as usize;
                let stack = vm.get_stack_mut(thread_id)?;
                stack.push_operand(Value::Ref(java_exception))?;
                *stack.pc_mut()? = handler_pc;
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn interpret_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let code_ptr = vm.method_area.get_method(&method_id).get_code()? as *const [u8];
        loop {
            // SAFETY: code_ptr is valid as long as method exists in method area (always)
            // need to use pointer to avoid borrow checker issues
            let code = unsafe { &*code_ptr };
            let pc = vm.get_stack_mut(&thread_id)?.pc()?;
            let instruction = Instruction::new_at(code, pc)?;

            match Self::interpret_instruction(thread_id, instruction, vm) {
                Ok(flow) => {
                    if let ControlFlow::Break(res) = flow {
                        return Ok(res);
                    }
                }
                Err(e) => {
                    let java_exception = match e {
                        JvmError::JavaException(exception) => {
                            vm.map_rust_error_to_java_exception(thread_id, exception)
                        }
                        JvmError::JavaExceptionThrown(exception_ref) => Ok(exception_ref),
                        // TODO: this errors are not mapped yet or happened during mapping to java exception
                        e => Err(e),
                    }?;
                    if vm.get_stack(&thread_id)?.cur_frame()?.is_native() {
                        vm.get_stack_mut(&thread_id)?.pop_native_frame()?;
                    }
                    if !Self::find_exception_handler(vm, &method_id, java_exception, &thread_id)? {
                        vm.get_stack_mut(&thread_id)?.pop_java_frame()?;
                        return Err(JvmError::JavaExceptionThrown(java_exception));
                    }
                }
            }
        }
    }

    fn invoke_native_method(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let method = vm.method_area.get_method(&method_id);
        let clone_desc = vm.br.clone_desc;
        let object_class_sym = vm.br.java_lang_object_sym;
        let mut method_key = vm
            .method_area
            .build_fully_qualified_native_method_key(&method_id);
        // native instance method of array special handling (for now, only Object.clone)
        if !method.is_static()
            && vm.heap.is_array(args[0].as_obj_ref()?)?
            && method_key.name == vm.br.clone_sym
            && method_key.desc == clone_desc
            && method_key.class == Some(object_class_sym)
        {
            method_key.class = None;
        }
        let frame = NativeFrame::new(method_id);
        vm.get_stack_mut(&thread_id)?
            .push_frame(FrameType::NativeFrame(frame))?;
        let native = *vm.native_registry.get(&method_key).ok_or(build_exception!(
            UnsatisfiedLinkError,
            vm.pretty_method_not_found_message(&method_id)
        ))?;
        let native_res = match native(vm, thread_id, args.as_slice()) {
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
        vm.get_stack_mut(&thread_id)?.pop_native_frame()?;
        Ok(native_res)
    }

    fn invoke_java_method(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let (max_stack, max_locals) = vm
            .method_area
            .get_method(&method_id)
            .get_frame_attributes()?;
        let frame = JavaFrame::new(method_id, max_stack, max_locals, args);
        vm.get_stack_mut(&thread_id)?
            .push_frame(FrameType::JavaFrame(frame))?;
        let method_ret = Self::interpret_method(thread_id, method_id, vm);
        if let Err(e) = &method_ret {
            error_log_method!(
                &method_id,
                e,
                "👹👹👹 Java exception thrown in interpreted method"
            );
        }
        let method_ret = method_ret?;
        vm.get_stack_mut(&thread_id)?.pop_java_frame()?;
        Ok(method_ret)
    }

    fn invoke_method_core(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let method = vm.method_area.get_method(&method_id);
        if method.is_native() {
            Self::invoke_native_method(thread_id, method_id, args, vm)
        } else {
            Self::invoke_java_method(thread_id, method_id, args, vm)
        }
    }

    fn invoke_method_internal(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let method_ret = Self::invoke_method_core(thread_id, method_id, args, vm)?;
        if let Some(ret) = method_ret {
            vm.get_stack_mut(&thread_id)?.push_operand(ret)?;
        }
        Ok(())
    }

    fn interface_needs_initialization(
        interface_id: ClassId,
        vm: &VirtualMachine,
    ) -> Result<bool, JvmError> {
        let interface = vm.method_area.get_interface_class(&interface_id)?;

        Ok(interface.has_clinit()) //TODO: || interface.has_non_constant_static_fields()?
    }

    fn run_clinit_if_exists(
        thread_id: ThreadId,
        class_id: ClassId,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        if let Some(&clinit_method_id) = vm
            .method_area
            .get_class_like(&class_id)?
            .get_clinit_method_id()
        {
            Self::invoke_method_internal(thread_id, clinit_method_id, vec![], vm)?;
        }

        Ok(())
    }

    pub fn ensure_initialized(
        thread_id: ThreadId,
        class_id: Option<ClassId>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let Some(class_id) = class_id else {
            return Ok(());
        };

        {
            let class = vm.method_area.get_class_like(&class_id)?;

            if class.is_initialized_or_initializing() {
                return Ok(());
            }

            class.set_initializing();
        }

        match vm.method_area.get_class(&class_id) {
            JvmClass::Instance(inst) => {
                if let Some(super_id) = inst.get_super() {
                    Self::ensure_initialized(thread_id, Some(super_id), vm)?;
                }
                for interface_id in vm
                    .method_area
                    .get_instance_class(&class_id)?
                    .get_interfaces()?
                    .clone()
                {
                    if Self::interface_needs_initialization(interface_id, vm)? {
                        Self::ensure_initialized(thread_id, Some(interface_id), vm)?;
                    }
                }

                Self::run_clinit_if_exists(thread_id, class_id, vm)?;

                let cur_class_name = vm.method_area.get_instance_class(&class_id)?.name();

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
                    vm.method_area
                        .get_instance_class(&class_id)?
                        .set_static_field_value(&ref_access_fk, Value::Ref(0))?;
                }
            }
            JvmClass::Interface(interface) => {
                for super_interface_id in interface.get_interfaces()?.clone() {
                    if Self::interface_needs_initialization(super_interface_id, vm)? {
                        Self::ensure_initialized(thread_id, Some(super_interface_id), vm)?;
                    }
                }

                Self::run_clinit_if_exists(thread_id, class_id, vm)?;
            }
            _ => {}
        }

        vm.method_area.get_class_like(&class_id)?.set_initialized();
        Ok(())
    }

    pub fn invoke_instance_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
        args: Vec<Value>,
    ) -> Result<Option<Value>, JvmError> {
        //TODO: do I need to check that args[0] is not null?
        Self::invoke_method_core(thread_id, method_id, args, vm)
    }

    pub fn invoke_static_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
        args: Vec<Value>,
    ) -> Result<(), JvmError> {
        let class_id = vm.method_area.get_method(&method_id).class_id();
        Self::ensure_initialized(thread_id, Some(class_id), vm)?;
        Self::invoke_method_internal(thread_id, method_id, args, vm)?;
        Ok(())
    }
}
