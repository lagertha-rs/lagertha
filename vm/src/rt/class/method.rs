use crate::class_file::attribute::annotation::Annotation;
use crate::class_file::attribute::code::{
    CodeAttributeInfo, LineNumberEntry, LocalVariableEntry, StackMapFrame,
};
use crate::class_file::attribute::method::{CodeAttribute, MethodAttribute};
use crate::class_file::method::MethodInfo;
use crate::rt::class::access::MethodAccessFlag;
use crate::rt::class::instruction_set::Instruction;
use crate::rt::class::LoadingError;
use crate::rt::constant_pool::reference::MethodDescriptorReference;
use crate::rt::constant_pool::RuntimeConstantPool;
use std::cell::OnceCell;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;

///https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
#[derive(Debug)]
pub struct CodeContext {
    max_stack: u16,
    max_locals: u16,
    instructions: Vec<Instruction>,
    // TODO: Create a dedicated struct? (now struct from class_file)
    line_numbers: Option<Vec<LineNumberEntry>>,
    // TODO: Create a dedicated struct? (now struct from class_file)
    local_variables: Option<Vec<LocalVariableEntry>>,
    stack_map_table: Option<Vec<StackMapFrame>>,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.6
#[derive(Debug)]
pub struct Method {
    pub name: Rc<String>,
    pub flags: MethodAccessFlag,
    pub descriptor: Rc<MethodDescriptorReference>,
    //TODO: not sure right now if method needs to have a direct access to the runtime constant pool
    // but now I use it only for display
    pub cp: Rc<RuntimeConstantPool>,
    pub code_context: Option<CodeContext>,
    pub signature: Option<Rc<String>>,
    pub rt_visible_annotations: Option<Vec<Annotation>>,
}

impl Method {
    pub fn new(method_info: MethodInfo, cp: Rc<RuntimeConstantPool>) -> Result<Self, LoadingError> {
        let name = cp.get_utf8(&method_info.name_index)?.clone();
        let flags = MethodAccessFlag::new(method_info.access_flags);
        let descriptor = cp.get_method_descriptor(&method_info.descriptor_index)?;

        let code_ctx = OnceCell::<CodeContext>::new();
        let signature = OnceCell::<Rc<String>>::new();
        let rt_vis_ann = OnceCell::<Vec<Annotation>>::new();
        let exceptions = OnceCell::<Vec<u16>>::new();

        for attr in method_info.attributes {
            match attr {
                MethodAttribute::Code(code) => code_ctx
                    .set(CodeContext::try_from(code)?)
                    .map_err(|_| LoadingError::DuplicatedCodeAttr)?,
                MethodAttribute::Signature(idx) => signature
                    .set(cp.get_utf8(&idx)?.clone())
                    .map_err(|_| LoadingError::DuplicatedSignatureAttr)?,
                MethodAttribute::RuntimeVisibleAnnotations(v) => rt_vis_ann
                    .set(v)
                    .map_err(|_| LoadingError::DuplicatedRuntimeVisibleAnnotationsAttr)?,
                MethodAttribute::Exceptions(v) => exceptions
                    .set(v)
                    .map_err(|_| LoadingError::DuplicatedExceptionAttribute)?,
                MethodAttribute::Unknown { name_index, .. } => {
                    unimplemented!("Unknown method attr: {}", name_index)
                }
            }
        }

        let code_context = match (flags.is_native(), code_ctx.into_inner()) {
            (true, Some(_)) => return Err(LoadingError::CodeAttrIsAmbiguousForNative),
            (true, None) => None,
            (false, Some(c)) => Some(c),
            (false, None) => return Err(LoadingError::MissingCodeAttr),
        };

        Ok(Method {
            name,
            flags,
            descriptor,
            cp,
            code_context,
            signature: signature.into_inner(),
            rt_visible_annotations: rt_vis_ann.into_inner(),
        })
    }
}

impl TryFrom<CodeAttribute> for CodeContext {
    type Error = LoadingError;

    fn try_from(code: CodeAttribute) -> Result<Self, Self::Error> {
        let mut all_line_numbers: Option<Vec<LineNumberEntry>> = None;
        let mut all_local_vars: Option<Vec<LocalVariableEntry>> = None;
        let mut stack_map_table = OnceCell::<Vec<StackMapFrame>>::new();

        for code_attr in code.attributes {
            match code_attr {
                CodeAttributeInfo::LineNumberTable(v) => {
                    if let Some(cur) = &mut all_line_numbers {
                        cur.extend(v);
                    } else {
                        all_line_numbers = Some(v);
                    }
                }
                CodeAttributeInfo::LocalVariableTable(v) => {
                    if let Some(cur) = &mut all_local_vars {
                        cur.extend(v);
                    } else {
                        all_local_vars = Some(v);
                    }
                    // TODO: JVMS §4.7.13: ensure no more than one entry per *local variable* across tables.
                }
                CodeAttributeInfo::StackMapTable(table) => stack_map_table
                    .set(table)
                    .map_err(|_| LoadingError::DuplicatedStackMapTable)?,
                CodeAttributeInfo::Unknown { name_index, .. } => {
                    unimplemented!("Unknown code attr {}", name_index);
                }
            }
        }

        Ok(CodeContext {
            max_stack: code.max_stack,
            max_locals: code.max_locals,
            instructions: Instruction::new_instruction_set(code.code)?,
            line_numbers: all_line_numbers,
            local_variables: all_local_vars,
            stack_map_table: stack_map_table.take(),
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
        //TODO: Move code part to the CodeContext Display
        if let Some(code) = &self.code_context {
            writeln!(f, "    Code:")?;
            writeln!(
                f,
                "      stack={}, locals={}, args_size={}",
                code.max_stack,
                code.max_locals,
                self.descriptor.resolved().params.len() //TODO: incorrect, for non static need to add + 1 (this)
            )?;
            let mut byte_pos = 0;
            for instruction in &code.instructions {
                write!(f, "        {byte_pos}: {instruction:<24}")?;
                match instruction {
                    Instruction::Ldc(index) => {
                        write!(f, "// {}", self.cp.get(index).map_err(Into::into)?)?
                    }
                    Instruction::Invokespecial(index)
                    | Instruction::Invokevirtual(index)
                    | Instruction::Getstatic(index) => {
                        write!(f, "// {}", self.cp.get(index).map_err(Into::into)?)?;
                    }
                    _ => {}
                }
                byte_pos += instruction.byte_size();
                writeln!(f)?;
            }

            if let Some(ln_table) = &code.line_numbers {
                writeln!(f, "      LineNumberTable:")?;
                for line_number in ln_table {
                    writeln!(
                        f,
                        "        line {}: {}",
                        line_number.line_number, line_number.start_pc
                    )?;
                }
            }

            //TODO: clean up
            if let Some(lv_table) = &code.local_variables {
                writeln!(f, "      LocalVariableTable:")?;
                const W_START: usize = 13;
                const W_LENGTH: usize = 8;
                const W_SLOT: usize = 5;
                const W_NAME: usize = 4;

                writeln!(
                    f,
                    "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$}   {}",
                    "Start", "Length", "Slot", "Name", "Signature",
                )?;
                for lv in lv_table {
                    let name = self.cp.get_utf8(&lv.name_index).map_err(Into::into)?;
                    let sig = self.cp.get_utf8(&lv.descriptor_index).map_err(Into::into)?;
                    writeln!(
                        f,
                        "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$}   {}",
                        lv.start_pc, lv.length, lv.index, name, sig,
                    )?;
                }
            }
        }

        Ok(())
    }
}
