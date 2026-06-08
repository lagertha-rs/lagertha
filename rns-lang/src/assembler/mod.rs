use crate::assembler::jvm_warning::JvmWarning;
use crate::ast::flag::{RnsClassFlag, RnsMethodFlag};
use crate::ast::{MethodDirective, RnsModule, RnsOperand};
use crate::diagnostic::Diagnostic;
use crate::token::type_hint::TypeHint;
use jclass::ClassFile;
use jclass::flags::ClassFlags;
use jclass::prelude::{
    ClassFileBuilder, CodeAttribute, ConstantPoolBuilder, MethodAttribute, MethodFlags, MethodInfo,
};

mod jvm_warning;

impl RnsModule {
    fn build_class_flags(&mut self) -> ClassFlags {
        let mut res = ClassFlags::new(0);
        for (flag, span) in &self.class_dir.flags {
            match flag {
                RnsClassFlag::Public => res.set_public(),
                RnsClassFlag::Final => res.set_final(),
                RnsClassFlag::Super => res.set_super(),
                RnsClassFlag::Interface => {
                    // TODO: put in method?
                    if !self.class_dir.flags.contains_key(&RnsClassFlag::Abstract) {
                        self.diagnostics.push(
                            JvmWarning::InterfaceFlagWithMissingAbstract {
                                interface_span: *span,
                            }
                            .into(),
                        )
                    }
                    let exclusive_flags = self
                        .class_dir
                        .flags
                        .iter()
                        .filter(|(f, _)| {
                            // TODO: add a method, something like "is_exclusive_with_interface" to RnsFlag
                            matches!(
                                f,
                                RnsClassFlag::Final
                                    | RnsClassFlag::Enum
                                    | RnsClassFlag::Module
                                    | RnsClassFlag::Super
                            )
                        })
                        .map(|(f, s)| (*f, *s))
                        .collect::<Vec<_>>();
                    if !exclusive_flags.is_empty() {
                        self.diagnostics.push(
                            JvmWarning::InterfaceMutuallyExclusive {
                                interface_span: *span,
                                exclusive_flags,
                            }
                            .into(),
                        )
                    }
                    res.set_interface()
                }
                RnsClassFlag::Abstract => res.set_abstract(),
                RnsClassFlag::Enum => res.set_enum(),
                RnsClassFlag::Synthetic => res.set_synthetic(),
                RnsClassFlag::Annotation => res.set_annotation(),
                RnsClassFlag::Module => res.set_module(),
                _ => unimplemented!(),
            }
        }
        res
    }

    fn build_method_flags(&mut self, method_dir: &MethodDirective) -> u16 {
        let mut res = 0;
        for (flag, span) in &method_dir.flags {
            match flag {
                RnsMethodFlag::Public => res |= 0x0001,
                RnsMethodFlag::Private => res |= 0x0002,
                RnsMethodFlag::Protected => res |= 0x0004,
                RnsMethodFlag::Static => res |= 0x0008,
                RnsMethodFlag::Final => res |= 0x0010,
                RnsMethodFlag::Synchronized => res |= 0x0020,
                RnsMethodFlag::Bridge => res |= 0x0040,
                RnsMethodFlag::Varargs => res |= 0x0080,
                RnsMethodFlag::Native => res |= 0x0100,
                RnsMethodFlag::Abstract => res |= 0x0400,
                RnsMethodFlag::Strict => res |= 0x0800,
                RnsMethodFlag::Synthetic => res |= 0x1000,
            }
        }
        res
    }

    fn add_type_hint_to_cp(cp_builder: &mut ConstantPoolBuilder, hint: TypeHint) -> u16 {
        match hint {
            TypeHint::Utf8(_, utf8_value) => cp_builder.add_utf8(&utf8_value.value),
            TypeHint::Class(_, class_name) => cp_builder.add_class(&class_name.value),
            TypeHint::Integer(_, int_value) => cp_builder.add_integer(int_value.value),
            TypeHint::Methodref(r) => {
                cp_builder.add_methodref(&r.class.value, &r.name.value, &r.descriptor.value)
            }
            TypeHint::Fieldref(r) => {
                cp_builder.add_fieldref(&r.class.value, &r.name.value, &r.descriptor.value)
            }
            TypeHint::String(_, string_value) => cp_builder.add_string(&string_value.value),
            TypeHint::CpIndex(_, explicit_idx) => explicit_idx.value, // TODO: warn somewhere?
            _ => unimplemented!(),
        }
    }

    fn build_super_class(&mut self, cp_builder: &mut ConstantPoolBuilder) -> Option<u16> {
        let name = self.super_dir.take()?.name;
        Some(Self::add_type_hint_to_cp(cp_builder, name))
    }

    fn build_method_directive(
        &mut self,
        cp_builder: &mut ConstantPoolBuilder,
        method_dir: MethodDirective,
    ) -> MethodInfo {
        let access_flags = MethodFlags::new(self.build_method_flags(&method_dir));
        let name_index = Self::add_type_hint_to_cp(cp_builder, method_dir.name.unwrap());
        let descriptor_index =
            Self::add_type_hint_to_cp(cp_builder, method_dir.descriptor.unwrap());
        let code_dir = method_dir.code_dir.unwrap(); // TODO: handle missing code directive
        let mut code = Vec::new();
        for ins in code_dir.instructions {
            let opcode = ins.spec.opcode;
            code.push(opcode as u8);
            match ins.operand {
                None => {}
                Some(RnsOperand::CpRef(hint)) => {
                    let cp_index = Self::add_type_hint_to_cp(cp_builder, hint);
                    match opcode.operand_size() {
                        1 => code.push(cp_index as u8),
                        2 => code.extend(cp_index.to_be_bytes()),
                        _ => unimplemented!(),
                    }
                }
                Some(RnsOperand::Byte(v)) => code.push(v.value),
                Some(RnsOperand::Label(label)) => {
                    // TODO: resolve label to branch offset
                    let target_pc = code_dir.labels.get(&label.value).expect("unknown label"); // TODO: proper error
                    let current_pc = (code.len() - 1) as u32; // pc of opcode
                    let offset = (*target_pc as i32) - (current_pc as i32);
                    match opcode.operand_size() {
                        2 => code.extend((offset as i16).to_be_bytes()),
                        4 => code.extend(offset.to_be_bytes()),
                        _ => unimplemented!(),
                    }
                }
            }
        }
        let code_attribute = MethodAttribute::Code(CodeAttribute {
            max_stack: code_dir.max_stack,
            max_locals: code_dir.max_locals,
            code,
            exception_table: vec![],
            attributes: vec![],
        });
        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes: vec![code_attribute],
        }
    }

    pub fn into_bytes(self) -> (Option<Vec<u8>>, Vec<Diagnostic>) {
        let (class_file, diagnostics) = self.into_class_file();
        let bytes = class_file.map(|cf| cf.to_bytes());
        (bytes, diagnostics)
    }

    // TODO: why tuple but not Result?
    pub fn into_class_file(mut self) -> (Option<ClassFile>, Vec<Diagnostic>) {
        let mut cp_builder = ConstantPoolBuilder::new();
        let super_cp_id = self.build_super_class(&mut cp_builder);
        let class_flags = self.build_class_flags();

        let this_cp_id = self
            .class_dir
            .name
            .take()
            .map(|name| Self::add_type_hint_to_cp(&mut cp_builder, name));

        let rns_methods = std::mem::take(&mut self.methods);
        let methods = rns_methods
            .into_iter()
            .map(|method_dir| self.build_method_directive(&mut cp_builder, method_dir))
            .collect();

        // TODO: only when at least one code is actually present
        cp_builder.add_utf8("Code");

        let class_file = ClassFileBuilder::new(0, 69, cp_builder.build()) // TODO: allow specifying version in jasm
            .access_flags(class_flags) // TODO: set access flags based on parsed flags
            .this_class(this_cp_id)
            .super_class(super_cp_id)
            .methods(methods)
            .build();

        (class_file, self.diagnostics)
    }
}
