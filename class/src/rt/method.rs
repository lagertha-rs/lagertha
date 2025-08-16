use crate::class_file::attribute::method::MethodAttribute;
use crate::class_file::method::MethodInfo;
use crate::rt::access::MethodAccessFlag;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::instruction_set::Instruction;
use crate::JvmError;
use std::fmt;
use std::fmt::Formatter;
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
    pub descriptor: Rc<MethodDescriptorReference>,
    //TODO: not sure right now if method needs to have a direct access to the runtime constant pool
    // but now I use it only for display
    pub const_pool: Rc<RuntimeConstantPool>,
    pub code_context: CodeContext,
}

impl Method {
    pub fn new(
        method_info: &MethodInfo,
        const_pool: Rc<RuntimeConstantPool>,
    ) -> Result<Self, JvmError> {
        let name = const_pool.get_utf8(&method_info.name_index)?.clone();
        let flags = MethodAccessFlag::new(method_info.access_flags);
        let descriptor = const_pool.get_method_descriptor(&method_info.descriptor_index)?;

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
            const_pool,
            name,
            flags,
            descriptor,
            code_context,
        })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}();", self.flags, self.name.replace("/", "."))?;
        writeln!(f, "    descriptor: {}", self.descriptor.raw())?;
        writeln!(
            f,
            "    flags: (0x{:04X}) {}",
            self.flags.get_raw(),
            self.flags
        )?;
        writeln!(f, "    Code:")?;
        writeln!(
            f,
            "      stack={}, locals={}, args_size={}",
            self.code_context.max_stack,
            self.code_context.max_locals,
            self.descriptor.resolved().params.len() //TODO: incorrect, for non static need to add + 1 (this)
        )?;
        let mut byte_pos = 0;
        for instruction in &self.code_context.instructions {
            write!(f, "        {byte_pos}: {instruction:<24}")?;
            match instruction {
                Instruction::Aload0 => {}
                Instruction::Ldc { .. } => {}
                Instruction::Invokespecial { method_index }
                | Instruction::Invokevirtual { method_index } => {
                    let method_ref = self
                        .const_pool
                        .get_methodref(method_index)
                        .map_err(|_| fmt::Error)?;
                    write!(f, "#{method_ref}")?;
                }
                Instruction::Getstatic { .. } => {}
                Instruction::Return => {}
            }
            byte_pos += instruction.byte_size();
            writeln!(f, "")?;
        }
        writeln!(f, "      ")?;
        writeln!(f, "      ")?;
        writeln!(f, "      ")?;

        Ok(())
    }
}
