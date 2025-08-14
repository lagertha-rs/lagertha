use crate::access::class::MethodAccessFlag;
use crate::instruction_set::Instruction;
use crate::runtime_constant_pool::RuntimeConstantPool;
use crate::JvmError;
use class_file::attribute::method::MethodAttribute;
use class_file::descriptor::MethodDescriptor;
use class_file::method::MethodInfo;
use std::rc::Rc;

///https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.6

#[derive(Debug)]
pub struct CodeContext {
    max_stack: u16,
    max_locals: u16,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Method {
    pub name: Rc<String>,
    pub flags: MethodAccessFlag,
    pub descriptor: Rc<MethodDescriptor>,
    pub code_context: CodeContext,
}

impl Method {
    pub fn new(
        method_info: &MethodInfo,
        const_pool: &RuntimeConstantPool,
    ) -> Result<Self, JvmError> {
        let name = const_pool.get_utf8(method_info.name_index)?.clone();
        let flags = MethodAccessFlag(method_info.access_flags);
        let descriptor = const_pool.get_method_descriptor(method_info.descriptor_index)?;

        let mut code_context = None;
        for attr in &method_info.attributes {
            match attr {
                MethodAttribute::Code {
                    max_stack,
                    max_locals,
                    code,
                    ..
                } => {
                    if code_context.is_some() {
                        Err(JvmError::TypeError)?
                    }
                    code_context = Some(CodeContext {
                        max_locals: *max_locals,
                        max_stack: *max_stack,
                        instructions: Instruction::new_instruction_set(code)?,
                    })
                }
                MethodAttribute::Unknown { .. } => {}
            }
        }
        let code_context = code_context.ok_or(JvmError::TypeError)?; //TODO: Error
        Ok(Method {
            name,
            flags,
            descriptor,
            code_context,
        })
    }
}
