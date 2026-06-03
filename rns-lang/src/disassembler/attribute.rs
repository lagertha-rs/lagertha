use crate::disassembler::indent_write::Indented;
use crate::disassembler::instruction::fmt_instruction_rns;
use crate::disassembler::{DisasmError, DisasmResult};
use jclass::attribute::{CodeAttribute, MethodAttribute};
use jclass::bytecode::Instruction;
use jclass::constant_pool::ConstantPool;
use std::fmt::Write as _;

pub(crate) fn fmt_method_attribute_rns(
    attribute: &MethodAttribute,
    method_name: &str,
    ind: &mut Indented,
    cp: &ConstantPool,
) -> DisasmResult<()> {
    ind.with_indent(|ind| match attribute {
        MethodAttribute::Code(code) => fmt_code_attribute_rns(code, method_name, ind, cp),
        other => Err(DisasmError::UnsupportedMethodAttribute {
            method: method_name.to_string(),
            attribute: format!("{other:?}"),
        }),
    })
}

fn fmt_code_attribute_rns(
    code: &CodeAttribute,
    method_name: &str,
    ind: &mut Indented,
    cp: &ConstantPool,
) -> DisasmResult<()> {
    writeln!(ind, ".code stack {} locals {}", code.max_stack, code.max_locals)?;
    ind.with_indent(|ind| {
        let mut pc = 0;
        while pc < code.code.len() {
            let inst = Instruction::new_at(&code.code, pc)?;
            fmt_instruction_rns(&inst, ind, cp)?;
            pc += inst.byte_size() as usize;
        }
        Ok(())
    })?;

    if !code.exception_table.is_empty() {
        return Err(DisasmError::UnsupportedExceptionTable {
            method: method_name.to_string(),
            handlers: code.exception_table.len(),
        });
    }

    if !code.attributes.is_empty() {
        let attribute = code
            .attributes
            .iter()
            .map(code_attribute_name)
            .next()
            .expect("non-empty code attributes must have first attribute");
        return Err(DisasmError::UnsupportedCodeAttribute {
            method: method_name.to_string(),
            attribute,
        });
    }

    writeln!(ind, ".end code")?;
    Ok(())
}

fn code_attribute_name(attribute: &jclass::attribute::method::CodeAttributeInfo) -> &'static str {
    match attribute {
        jclass::attribute::method::CodeAttributeInfo::LineNumberTable(_) => "LineNumberTable",
        jclass::attribute::method::CodeAttributeInfo::LocalVariableTable(_) => {
            "LocalVariableTable"
        }
        jclass::attribute::method::CodeAttributeInfo::StackMapTable(_) => "StackMapTable",
        jclass::attribute::method::CodeAttributeInfo::LocalVariableTypeTable(_) => {
            "LocalVariableTypeTable"
        }
        jclass::attribute::method::CodeAttributeInfo::RuntimeVisibleTypeAnnotations => {
            "RuntimeVisibleTypeAnnotations"
        }
        jclass::attribute::method::CodeAttributeInfo::RuntimeInvisibleTypeAnnotations => {
            "RuntimeInvisibleTypeAnnotations"
        }
    }
}
