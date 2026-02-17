use crate::attribute::method::code::{CodeAttributeInfo, StackMapFrame, VerificationTypeInfo};
use crate::attribute::method::{CodeAttribute, ParameterAnnotations};
use crate::attribute::{
    Annotation, ClassAttribute, ElementValue, FieldAttribute, MethodAttribute, SharedAttribute,
    TargetInfo, TypeAnnotation,
};
use crate::bytecode::Instruction;
use crate::constant_pool::ConstantPool;
use crate::flags::{InnerClassFlags, MethodParamFlags};
use common::descriptor::MethodDescriptor;
use common::error::ClassFormatErr;
use common::try_javap_print;
use common::try_javap_print_class_name;
use common::utils::indent_write::Indented;
use itertools::Itertools;
use std::fmt;
use std::fmt::Write as _;

impl MethodAttribute {
    pub(crate) fn javap_fmt(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        descriptor: &MethodDescriptor,
        this: &u16,
        is_static: bool,
    ) -> fmt::Result {
        match self {
            MethodAttribute::Shared(shared) => shared.javap_fmt(ind, cp)?,
            MethodAttribute::Code(code) => code.javap_fmt(ind, cp, descriptor, this, is_static)?,
            MethodAttribute::Exceptions(exc) => {
                writeln!(ind, "Exceptions:")?;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "throws {}",
                        try_javap_print!(
                            ind,
                            exc.iter()
                                .map(|index| cp.get_javap_class_name(index))
                                .collect::<Result<Vec<_>, _>>()
                        )
                        .join(", ")
                    )?;
                    Ok(())
                })?
            }
            MethodAttribute::RuntimeVisibleParameterAnnotations(param_annotations) => {
                Self::fmt_parameter_annotations(
                    ind,
                    cp,
                    "RuntimeVisibleParameterAnnotations:",
                    param_annotations,
                )?
            }
            MethodAttribute::RuntimeInvisibleParameterAnnotations(param_annotations) => {
                Self::fmt_parameter_annotations(
                    ind,
                    cp,
                    "RuntimeInvisibleParameterAnnotations:",
                    param_annotations,
                )?
            }
            MethodAttribute::AnnotationsDefault => unimplemented!(),
            MethodAttribute::MethodParameters(params) => {
                const W_NAME: usize = 32;
                writeln!(ind, "MethodParameters:")?;
                ind.with_indent(|ind| {
                    writeln!(ind, "{:<W_NAME$} Flags", "Name")?;
                    for param in params {
                        let name = if param.name_index == 0 {
                            "<no name>".to_string()
                        } else {
                            try_javap_print!(ind, cp.get_utf8(&param.name_index)).to_string()
                        };
                        write!(ind, "{:<W_NAME$} ", name)?;
                        MethodParamFlags::new(param.access_flags).javap_fmt(ind)?;
                        writeln!(ind)?;
                    }
                    Ok(())
                })?;
            }
        }

        Ok(())
    }

    fn fmt_parameter_annotations(
        ind: &mut Indented,
        cp: &ConstantPool,
        header: &str,
        param_annotations: &[ParameterAnnotations],
    ) -> fmt::Result {
        writeln!(ind, "{header}")?;
        ind.with_indent(|ind| {
            for (param_idx, param) in param_annotations.iter().enumerate() {
                writeln!(ind, "parameter {param_idx}:")?;
                ind.with_indent(|ind| {
                    for (ann_idx, annotation) in param.annotations.iter().enumerate() {
                        writeln!(
                            ind,
                            "{}: #{}({})",
                            ann_idx,
                            annotation.type_index,
                            annotation
                                .element_value_pairs
                                .iter()
                                .map(|pair| format!(
                                    "#{}={}",
                                    pair.element_name_index,
                                    pair.value.get_javap_descriptor()
                                ))
                                .join(",")
                        )?;
                        ind.with_indent(|ind| {
                            let type_name = try_javap_print_class_name!(
                                ind,
                                cp.get_utf8(&annotation.type_index)
                            );
                            writeln!(ind, "{type_name}")?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl CodeAttribute {
    pub(crate) fn javap_fmt(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        method_descriptor: &MethodDescriptor,
        this: &u16,
        is_static: bool,
    ) -> fmt::Result {
        writeln!(ind, "Code: ")?;
        ind.with_indent(|ind| {
            writeln!(
                ind,
                "stack={}, locals={}, args_size={}",
                self.max_stack,
                self.max_locals,
                // +1 for 'this' if not static
                method_descriptor.params.len() + if is_static { 0 } else { 1 }
            )?;

            let mut instructions = Vec::new();
            let mut pc = 0;
            let code_len = self.code.len();
            while pc < code_len {
                let inst = try_javap_print!(ind, Instruction::new_at(&self.code, pc));
                pc += inst.byte_size() as usize;
                instructions.push(inst);
            }
            let mut byte_pos = 0;
            for instruction in instructions {
                writeln!(
                    ind,
                    "{byte_pos:4}: {:<24}",
                    try_javap_print!(
                        ind,
                        instruction.get_javap_instruction_string(cp, byte_pos as i32, this)
                    )
                )?;
                byte_pos += instruction.byte_size();
            }
            if !self.exception_table.is_empty() {
                const W_START: usize = 6;
                const W_LENGTH: usize = 8;
                const W_SLOT: usize = 5;
                writeln!(ind, "Exception table:")?;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$} type",
                        "from", "to", "target"
                    )?;
                    for entry in &self.exception_table {
                        let catch_type = if entry.catch_type == 0 {
                            "any"
                        } else {
                            try_javap_print!(ind, cp.get_class_name(&entry.catch_type))
                        };
                        writeln!(
                            ind,
                            "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {}{}",
                            entry.start_pc,
                            entry.end_pc,
                            entry.handler_pc,
                            if catch_type != "any" { "Class " } else { "" },
                            catch_type
                        )?;
                    }
                    Ok(())
                })?;
            }
            for attr in &self.attributes {
                attr.javap_fmt(ind, cp, this)?;
            }
            Ok(())
        })?;

        Ok(())
    }
}

impl StackMapFrame {
    pub(crate) fn javap_fmt(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        this: &u16,
    ) -> fmt::Result {
        let get_javap_verif_type =
            |locals: &Vec<VerificationTypeInfo>| -> Result<String, ClassFormatErr> {
                locals
                    .iter()
                    .map(|v| v.get_javap_value(cp, this))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|v| v.join(", "))
            };

        write!(ind, "frame_type = ")?;
        match self {
            StackMapFrame::Same { offset_delta } => writeln!(ind, "{offset_delta} /* same */")?,
            StackMapFrame::SameLocals1StackItem {
                offset_delta,
                stack,
            } => {
                writeln!(ind, "{} /* same_locals_1_stack_item */", offset_delta + 64)?;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "stack = [ {} ]",
                        try_javap_print!(ind, stack.get_javap_value(cp, this))
                    )?;
                    Ok(())
                })?;
            }
            StackMapFrame::SameLocals1StackItemExtended {
                offset_delta,
                stack,
            } => {
                writeln!(ind, "247 /* same_locals_1_stack_item_frame_extended */")?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    writeln!(
                        ind,
                        "stack = [ {} ]",
                        try_javap_print!(ind, stack.get_javap_value(cp, this))
                    )?;
                    Ok(())
                })?;
            }
            StackMapFrame::Chop { k, offset_delta } => {
                writeln!(ind, "{} /* chop */", 251 - k)?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    Ok(())
                })?;
            }
            StackMapFrame::SameExtended { offset_delta } => {
                writeln!(ind, "251 /* same_frame_extended */")?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    Ok(())
                })?;
            }
            StackMapFrame::Append {
                k,
                offset_delta,
                locals,
            } => {
                writeln!(ind, "{} /* append */", 251 + k)?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    writeln!(
                        ind,
                        "locals = [{}]",
                        try_javap_print!(ind, get_javap_verif_type(locals))
                    )?;
                    Ok(())
                })?;
            }
            StackMapFrame::Full {
                offset_delta,
                locals,
                stack,
            } => {
                writeln!(ind, "255 /* full_frame */")?;
                ind.with_indent(|ind| {
                    writeln!(ind, "offset_delta = {offset_delta}")?;
                    writeln!(
                        ind,
                        "locals = [{}]",
                        try_javap_print!(ind, get_javap_verif_type(locals))
                    )?;
                    writeln!(
                        ind,
                        "stack = [{}]",
                        try_javap_print!(ind, get_javap_verif_type(stack))
                    )?;
                    Ok(())
                })?;
            }
        }
        Ok(())
    }
}

impl VerificationTypeInfo {
    pub(crate) fn get_javap_value(
        &self,
        cp: &ConstantPool,
        this: &u16,
    ) -> Result<String, ClassFormatErr> {
        Ok(match self {
            VerificationTypeInfo::Top => "top".to_string(),
            VerificationTypeInfo::Integer => "int".to_string(),
            VerificationTypeInfo::Float => "float".to_string(),
            VerificationTypeInfo::Double => "double".to_string(),
            VerificationTypeInfo::Long => "long".to_string(),
            VerificationTypeInfo::Null => "null".to_string(),
            VerificationTypeInfo::UninitializedThis => "this".to_string(),
            VerificationTypeInfo::Object(cp_index) => {
                cp.get_raw(cp_index)?.get_javap_type_and_value(cp, this)?
            }
            VerificationTypeInfo::Uninitialized(idx) => format!("uninitialized {idx}"),
        })
    }
}

impl CodeAttributeInfo {
    pub(crate) fn javap_fmt(
        &self,
        ind: &mut Indented,
        cp: &ConstantPool,
        this: &u16,
    ) -> fmt::Result {
        match self {
            CodeAttributeInfo::LineNumberTable(table) => {
                writeln!(ind, "LineNumberTable:")?;
                ind.with_indent(|ind| {
                    for entry in table {
                        writeln!(ind, "line {}: {}", entry.line_number, entry.start_pc)?;
                    }
                    Ok(())
                })?;
            }
            CodeAttributeInfo::LocalVariableTable(table) => {
                const W_START: usize = 6;
                const W_LENGTH: usize = 8;
                const W_SLOT: usize = 5;
                const W_NAME: usize = 6;
                writeln!(ind, "LocalVariableTable:")?;
                writeln!(
                    ind,
                    "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$} Signature",
                    "Start", "Length", "Slot", "Name",
                )?;
                for lv in table {
                    let name = try_javap_print!(ind, cp.get_utf8(&lv.name_index));
                    let descriptor = try_javap_print!(ind, cp.get_utf8(&lv.descriptor_index));
                    writeln!(
                        ind,
                        "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$} {}",
                        lv.start_pc, lv.length, lv.index, name, descriptor,
                    )?;
                }
            }
            CodeAttributeInfo::StackMapTable(table) => {
                writeln!(ind, "StackMapTable: number_of_entries = {}", table.len())?;
                ind.with_indent(|ind| {
                    for frame in table {
                        frame.javap_fmt(ind, cp, this)?;
                    }
                    Ok(())
                })?;
            }
            CodeAttributeInfo::LocalVariableTypeTable(table) => {
                writeln!(ind, "LocalVariableTypeTable:")?;
                const W_START: usize = 6;
                const W_LENGTH: usize = 8;
                const W_SLOT: usize = 5;
                const W_NAME: usize = 4;
                ind.with_indent(|ind| {
                    writeln!(
                        ind,
                        "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$}   Signature",
                        "Start", "Length", "Slot", "Name"
                    )?;
                    for lv in table {
                        let name = try_javap_print!(ind, cp.get_utf8(&lv.name_index));
                        let signature = try_javap_print!(ind, cp.get_utf8(&lv.signature_index));
                        writeln!(
                            ind,
                            "{:>W_START$} {:>W_LENGTH$} {:>W_SLOT$}  {:<W_NAME$}   {}",
                            lv.start_pc, lv.length, lv.index, name, signature,
                        )?;
                    }
                    Ok(())
                })?;
            }
            CodeAttributeInfo::RuntimeVisibleTypeAnnotations => unimplemented!(),
            CodeAttributeInfo::RuntimeInvisibleTypeAnnotations => unimplemented!(),
        }
        Ok(())
    }
}

impl SharedAttribute {
    fn fmt_annotations(
        ind: &mut Indented,
        cp: &ConstantPool,
        annotations: &[Annotation],
    ) -> fmt::Result {
        for (i, annotation) in annotations.iter().enumerate() {
            writeln!(
                ind,
                "{i}: #{}({})",
                annotation.type_index,
                annotation
                    .element_value_pairs
                    .iter()
                    .map(|pair| format!(
                        "#{}={}",
                        pair.element_name_index,
                        pair.value.get_javap_descriptor()
                    ))
                    .join(",")
            )?;
            ind.with_indent(|ind| {
                write!(
                    ind,
                    "{}",
                    try_javap_print_class_name!(ind, cp.get_utf8(&annotation.type_index))
                )?;
                if !annotation.element_value_pairs.is_empty() {
                    writeln!(ind, "(")?;
                    for param in &annotation.element_value_pairs {
                        ind.with_indent(|ind| {
                            writeln!(
                                ind,
                                "{}={}",
                                try_javap_print!(ind, cp.get_utf8(&param.element_name_index)),
                                try_javap_print!(ind, param.value.get_javap_value(cp))
                            )?;
                            Ok(())
                        })?;
                    }
                    writeln!(ind, ")")?;
                } else {
                    writeln!(ind)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }

    pub(crate) fn javap_fmt(&self, ind: &mut Indented, cp: &ConstantPool) -> fmt::Result {
        match self {
            SharedAttribute::Synthetic => unimplemented!(),
            SharedAttribute::Deprecated => writeln!(ind, "Deprecated: true")?,
            SharedAttribute::Signature(index) => writeln!(
                ind,
                "Signature: #{:<26} // {}",
                index,
                try_javap_print!(ind, cp.get_utf8(index)),
            )?,
            SharedAttribute::RuntimeVisibleAnnotations(annotations) => {
                writeln!(ind, "RuntimeVisibleAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_annotations(ind, cp, annotations))?
            }
            SharedAttribute::RuntimeInvisibleAnnotations(annotations) => {
                writeln!(ind, "RuntimeInvisibleAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_annotations(ind, cp, annotations))?
            }
            SharedAttribute::RuntimeVisibleTypeAnnotations(annotations) => {
                writeln!(ind, "RuntimeVisibleTypeAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_type_annotations(ind, cp, annotations))?
            }
            SharedAttribute::RuntimeInvisibleTypeAnnotations(annotations) => {
                writeln!(ind, "RuntimeInvisibleTypeAnnotations:")?;
                ind.with_indent(|ind| Self::fmt_type_annotations(ind, cp, annotations))?
            }
        }

        Ok(())
    }

    fn fmt_type_annotations(
        ind: &mut Indented,
        cp: &ConstantPool,
        annotations: &[TypeAnnotation],
    ) -> fmt::Result {
        for (i, annotation) in annotations.iter().enumerate() {
            // Format target info
            let target_str = match &annotation.target_info {
                TargetInfo::TypeParameter {
                    type_parameter_index,
                } => format!("CLASS_TYPE_PARAMETER, param_index={type_parameter_index}"),
                TargetInfo::Supertype { supertype_index } => {
                    format!("CLASS_EXTENDS, type_index={supertype_index}")
                }
                TargetInfo::TypeParameterBound {
                    type_parameter_index,
                    bound_index,
                } => format!(
                    "CLASS_TYPE_PARAMETER_BOUND, param_index={type_parameter_index}, bound_index={bound_index}"
                ),
                TargetInfo::Empty => "EMPTY".to_string(),
                TargetInfo::MethodFormalParameter {
                    formal_parameter_index,
                } => format!("METHOD_FORMAL_PARAMETER, param_index={formal_parameter_index}"),
                TargetInfo::Throws { throws_type_index } => {
                    format!("THROWS, type_index={throws_type_index}")
                }
                TargetInfo::LocalVar { localvar_table } => {
                    let entries = localvar_table
                        .iter()
                        .map(|e| {
                            format!(
                                "start_pc={}, length={}, index={}",
                                e.start_pc, e.length, e.index
                            )
                        })
                        .join("; ");
                    format!("LOCAL_VARIABLE, {{{entries}}}")
                }
                TargetInfo::Catch {
                    exception_table_index,
                } => format!("EXCEPTION_PARAMETER, exception_index={exception_table_index}"),
                TargetInfo::Offset { offset } => format!("OFFSET, offset={offset}"),
                TargetInfo::TypeArgument {
                    offset,
                    type_argument_index,
                } => format!(
                    "TYPE_ARGUMENT, offset={offset}, type_argument_index={type_argument_index}"
                ),
            };

            // Format element value pairs
            let pairs_str = if annotation.element_value_pairs.is_empty() {
                String::new()
            } else {
                annotation
                    .element_value_pairs
                    .iter()
                    .map(|pair| {
                        format!(
                            "#{}={}",
                            pair.element_name_index,
                            pair.value.get_javap_descriptor()
                        )
                    })
                    .join(",")
            };

            writeln!(
                ind,
                "{i}: #{}({pairs_str}): {target_str}",
                annotation.type_index
            )?;
            ind.with_indent(|ind| {
                let type_name =
                    try_javap_print_class_name!(ind, cp.get_utf8(&annotation.type_index));
                writeln!(ind, "{type_name}")?;
                Ok(())
            })?;
        }
        Ok(())
    }
}

impl ClassAttribute {
    pub(crate) fn javap_fmt(&self, ind: &mut Indented, cp: &ConstantPool) -> fmt::Result {
        match self {
            ClassAttribute::Shared(shared) => shared.javap_fmt(ind, cp)?,
            ClassAttribute::SourceFile(idx) => {
                writeln!(
                    ind,
                    "SourceFile: \"{}\"",
                    try_javap_print!(ind, cp.get_utf8(idx))
                )?;
            }
            ClassAttribute::InnerClasses(inner) => {
                writeln!(ind, "InnerClasses:")?;
                ind.with_indent(|ind| {
                    for entry in inner {
                        let inner_class =
                            try_javap_print!(ind, cp.get_raw(&entry.inner_class_info_index));

                        // Truly anonymous class
                        if entry.outer_class_info_index == 0 && entry.inner_name_index == 0 {
                            let inner_access_flags =
                                InnerClassFlags::new(entry.inner_class_access_flags);
                            let flags_str = inner_access_flags.javap_java_like_prefix();
                            let left_part = if flags_str.is_empty() {
                                format!("#{};", entry.inner_class_info_index)
                            } else {
                                format!("{} #{};", flags_str, entry.inner_class_info_index)
                            };
                            writeln!(
                                ind,
                                "{:<43} // {}",
                                left_part,
                                try_javap_print!(ind, inner_class.get_javap_type_and_value(cp, &0)),
                            )?;
                        }
                        // Local/member class
                        else if entry.outer_class_info_index == 0 {
                            writeln!(
                                ind,
                                "{:<43} // {}={}",
                                format!(
                                    "#{}= #{};",
                                    entry.inner_name_index, entry.inner_class_info_index
                                ),
                                try_javap_print!(ind, cp.get_utf8(&entry.inner_name_index)),
                                try_javap_print!(ind, inner_class.get_javap_type_and_value(cp, &0)),
                            )?;
                        }
                        // Regular inner class
                        else {
                            let inner_access_flags =
                                InnerClassFlags::new(entry.inner_class_access_flags);
                            let outer_class =
                                try_javap_print!(ind, cp.get_raw(&entry.outer_class_info_index));
                            writeln!(
                                ind,
                                "{:<43} // {}={} of {}",
                                format!(
                                    "{} #{}= #{} of #{};",
                                    inner_access_flags.javap_java_like_prefix(),
                                    entry.inner_name_index,
                                    entry.inner_class_info_index,
                                    entry.outer_class_info_index
                                ),
                                try_javap_print!(ind, cp.get_utf8(&entry.inner_name_index)),
                                try_javap_print!(ind, inner_class.get_javap_type_and_value(cp, &0)),
                                try_javap_print!(ind, outer_class.get_javap_type_and_value(cp, &0))
                            )?;
                        }
                    }
                    Ok(())
                })?;
            }
            ClassAttribute::EnclosingMethod(class_idx, method_idx) => {
                let method = if *method_idx == 0 {
                    ""
                } else {
                    try_javap_print!(ind, cp.get_method_or_field_name_by_nat_idx(method_idx))
                };
                writeln!(
                    ind,
                    "{:<24} // {}{}{}",
                    format!("EnclosingMethod: #{}.#{}", class_idx, method_idx),
                    try_javap_print!(ind, cp.get_javap_class_name(class_idx)),
                    if method.is_empty() { "" } else { "." },
                    method
                )?;
            }
            ClassAttribute::SourceDebugExtension => unimplemented!(),
            ClassAttribute::BootstrapMethods(bootstrap_methods) => {
                writeln!(ind, "BootstrapMethods:")?;
                ind.with_indent(|ind| {
                    for (i, method) in bootstrap_methods.iter().enumerate() {
                        let method_handle =
                            try_javap_print!(ind, cp.get_raw(&method.bootstrap_method_idx));
                        writeln!(
                            ind,
                            "{}: #{} {}",
                            i,
                            method.bootstrap_method_idx,
                            try_javap_print!(ind, method_handle.get_javap_type_and_value(cp, &0))
                        )?;
                        ind.with_indent(|ind| {
                            writeln!(ind, "Method arguments:")?;
                            ind.with_indent(|ind| {
                                for arg in &method.bootstrap_arguments {
                                    let argument = try_javap_print!(ind, cp.get_raw(arg));
                                    writeln!(
                                        ind,
                                        "#{} {}",
                                        arg,
                                        try_javap_print!(ind, argument.get_javap_value(cp, &0))
                                    )?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }
            ClassAttribute::Module => unimplemented!(),
            ClassAttribute::ModulePackages => unimplemented!(),
            ClassAttribute::ModuleMainClass => unimplemented!(),
            ClassAttribute::NestHost(idx) => {
                let constant = try_javap_print!(ind, cp.get_raw(idx));
                writeln!(
                    ind,
                    "NestHost: {}",
                    try_javap_print!(ind, constant.get_javap_type_and_value(cp, &0))
                )?;
            }
            ClassAttribute::NestMembers(members) => {
                writeln!(ind, "NestMembers:")?;
                ind.with_indent(|ind| {
                    for member in members {
                        writeln!(ind, "{}", try_javap_print!(ind, cp.get_class_name(member)))?;
                    }
                    Ok(())
                })?;
            }
            ClassAttribute::Record => unimplemented!(),
            ClassAttribute::PermittedSubclasses(classes) => {
                writeln!(ind, "PermittedSubclasses:")?;
                ind.with_indent(|ind| {
                    for class in classes {
                        writeln!(ind, "{}", try_javap_print!(ind, cp.get_class_name(class)))?;
                    }
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}

impl FieldAttribute {
    pub(crate) fn javap_fmt(&self, ind: &mut Indented, cp: &ConstantPool) -> fmt::Result {
        match self {
            FieldAttribute::Shared(shared) => shared.javap_fmt(ind, cp)?,
            FieldAttribute::ConstantValue(val) => {
                let constant = try_javap_print!(ind, cp.get_raw(val));
                writeln!(
                    ind,
                    "ConstantValue: {}",
                    try_javap_print!(ind, constant.get_javap_type_and_value(cp, &0))
                )?;
            }
        }

        Ok(())
    }
}

impl ElementValue {
    pub fn get_javap_descriptor(&self) -> String {
        match self {
            ElementValue::Boolean(v) => format!("Z#{}", v),
            ElementValue::String(v) => format!("s#{}", v),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn get_javap_value(&self, cp: &ConstantPool) -> Result<String, ClassFormatErr> {
        Ok(match self {
            ElementValue::Boolean(idx) => match cp.get_integer(idx)? {
                0 => "false".to_string(),
                1 => "true".to_string(),
                _ => unimplemented!(),
            },
            ElementValue::String(idx) => format!("\"{}\"", cp.get_utf8(idx)?),
            _ => unimplemented!(),
        })
    }
}
