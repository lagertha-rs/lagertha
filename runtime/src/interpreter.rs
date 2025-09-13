use crate::heap::Heap;
use crate::method_area::MethodArea;
use crate::native::JNIEnv;
use crate::rt::class::class::{Class, InitState};
use crate::rt::constant_pool::RuntimeConstant;
use crate::rt::method::StaticMethodType;
use crate::stack::{Frame, FrameStack};
use crate::string_pool::StringPool;
use crate::{JvmError, MethodKey, VmConfig};
use common::instruction::Instruction;
use common::jtype::{ObjectRef, Value};
use std::sync::Arc;
use tracing_log::log::debug;

pub struct Interpreter {
    method_area: Arc<MethodArea>,
    frame_stack: FrameStack,
    native_stack: (),
    pc: (),
    jni_env: JNIEnv,
    heap: Heap,
    string_pool: StringPool,
}

impl Interpreter {
    pub fn new(vm_config: &VmConfig, method_area: Arc<MethodArea>) -> Self {
        let thread_stack = FrameStack::new(vm_config);
        let jni_env = JNIEnv::new();
        let heap = Heap::new();
        let string_pool = StringPool::new();
        Self {
            method_area,
            frame_stack: thread_stack,
            native_stack: (),
            pc: (),
            jni_env,
            heap,
            string_pool,
        }
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

                self.run_static_method(class, initializer)?;

                // TODO: need to be placed in better place
                if class.name()? == "java/lang/System" {
                    let init = class.get_static_method("initPhase1", "()V")?;
                    self.run_static_method(class, init)?;
                }

                class.set_state(InitState::Initialized);
                debug!("Class {} initialized", class.name()?);
            }
        }
        Ok(())
    }

    fn interpret_instruction(&mut self, instruction: &Instruction) -> Result<(), JvmError> {
        debug!("Executing instruction: {:?}", instruction);
        match instruction {
            Instruction::Iconst0 => {
                self.frame_stack.cur_frame_push_operand(Value::Int(0))?;
            }
            Instruction::Putstatic(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let field_ref = cp.get_fieldref(idx)?;
                let class = self.method_area.get_class(field_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let field_nat = field_ref.name_and_type()?;
                class.set_static_field(field_nat, self.frame_stack.cur_frame_pop_operand()?)?;
            }
            Instruction::Getstatic(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let field_ref = cp.get_fieldref(idx)?;
                let class = self.method_area.get_class(field_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let field_nat = field_ref.name_and_type()?;
                let value = class.get_static_field_value(field_nat)?;
                self.frame_stack.cur_frame_push_operand(value)?;
            }
            Instruction::InvokeStatic(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let method_ref = cp.get_methodref(idx)?;
                let class = self.method_area.get_class(method_ref.class()?.name()?)?;
                self.ensure_initialized(Some(&class))?;
                let method = class.get_static_method_by_nat(method_ref)?;
                self.run_static_method(&class, method)?;
            }
            Instruction::AconstNull => {
                self.frame_stack
                    .cur_frame_push_operand(Value::Object(ObjectRef::Null))?;
            }
            Instruction::Ldc(idx) => {
                let cp = self.frame_stack.cur_frame_cp()?;
                let raw = cp.get(idx)?;
                match raw {
                    RuntimeConstant::String(data) => {
                        let string_addr =
                            self.string_pool.get_or_new(&mut self.heap, data.value()?);
                        self.frame_stack
                            .cur_frame_push_operand(Value::Object(ObjectRef::Ref(string_addr)))?;
                    }
                    _ => unimplemented!("Ldc for constant {:?}", raw),
                }
            }
            Instruction::Return => {
                // TODO: does nothing right now, since I don't have return values in methods
                // and the instructions are executed one by one in a loop
            }
            unimp => unimplemented!("Instruction {:?} not implemented", unimp),
        }
        Ok(())
    }

    fn run_static_method(
        &mut self,
        class: &Arc<Class>,
        method: &StaticMethodType,
    ) -> Result<(), JvmError> {
        match method {
            StaticMethodType::Java(method) => {
                debug!(
                    "Running static method {}{} of class {}",
                    method.name(),
                    method.descriptor().raw(),
                    class.name()?
                );
                let frame = Frame::new(class.cp().clone(), method.max_locals(), method.max_stack());

                self.frame_stack.push_frame(frame)?;

                let instructions = method.instructions();
                for instruction in instructions {
                    self.interpret_instruction(instruction)?;
                }

                // TODO: delete, since I don't have return in clinit and tests for it
                // just to be sure that no operands are left in the stack before popping the frame
                assert!(self.frame_stack.cur_frame_pop_operand().is_err());
                self.frame_stack.pop_frame()?;
                Ok(())
            }
            StaticMethodType::Native(method) => {
                debug!(
                    "Running native method {}{} of class {}",
                    method.name(),
                    method.descriptor().raw(),
                    class.name()?
                );
                // TODO: pass args, native stack?
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
                method(&mut self.jni_env, &[]);
                Ok(())
            }
        }
    }

    //TODO: redisign start method (maybe return Value, maybe take args)
    pub fn start(&mut self, data: Vec<u8>) -> Result<(), JvmError> {
        let main_class = self.method_area.add_class(data)?;
        let main_method = main_class
            .find_main_method()
            .ok_or(JvmError::NoMainClassFound(main_class.name()?.to_string()))?;
        debug!("Found main method in class {}", main_class.name()?);
        self.ensure_initialized(Some(&main_class))?;
        let instructions = main_method.instructions();
        let frame = Frame::new(
            main_class.cp().clone(),
            main_method.max_locals(),
            main_method.max_stack(),
        );
        self.frame_stack.push_frame(frame)?;

        debug!("Executing main method...");

        for instruction in instructions {
            self.interpret_instruction(instruction)?;
        }

        self.frame_stack.pop_frame()?;

        //TODO: delete, since I don't have return in main and tests for it
        // just to be sure that stack is empty
        assert!(self.frame_stack.pop_frame().is_err());

        Ok(())
    }
}
