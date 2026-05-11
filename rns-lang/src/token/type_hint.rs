use crate::token::Span;
use crate::token::span::Spanned;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TypeHintOperandName {
    Utf8Entry,
    U16Literal,
    I32Literal,
    F32Literal,
    I64Literal,
    F64Literal,
    StringLiteral,
    ClassName,
    MethodName,
    MethodDescriptor,
    FieldName,
    FieldDescriptor,
}

impl Display for TypeHintOperandName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8Entry => write!(f, "utf8 value"),
            Self::U16Literal => write!(f, "16-bit unsigned integer"),
            Self::I32Literal => write!(f, "32-bit signed integer"),
            Self::F32Literal => write!(f, "32-bit float"),
            Self::I64Literal => write!(f, "64-bit signed integer"),
            Self::F64Literal => write!(f, "64-bit float"),
            Self::StringLiteral => write!(f, "string literal"),
            Self::ClassName => write!(f, "class name"),
            Self::MethodName => write!(f, "method name"),
            Self::MethodDescriptor => write!(f, "method descriptor"),
            Self::FieldName => write!(f, "field name"),
            Self::FieldDescriptor => write!(f, "field descriptor"),
        }
    }
}

impl TypeHintOperandName {
    pub fn placeholder(&self) -> &'static str {
        match self {
            Self::Utf8Entry => "<utf8_value>",
            Self::U16Literal => "<unsigned_short>",
            Self::I32Literal => "<integer>",
            Self::F32Literal => "<float>",
            Self::I64Literal => "<long>",
            Self::F64Literal => "<double>",
            Self::StringLiteral => "\"<string>\"",
            Self::ClassName => "<class_name>",
            Self::MethodName => "<method_name>",
            Self::MethodDescriptor => "<method_descriptor>",
            Self::FieldName => "<field_name>",
            Self::FieldDescriptor => "<field_descriptor>",
        }
    }
}

pub const TYPE_HINT_AT_CP_IDX: &str = "@cp_idx";
pub const TYPE_HINT_AT_UTF8: &str = "@utf8";
pub const TYPE_HINT_AT_INTEGER: &str = "@int";
pub const TYPE_HINT_AT_STRING: &str = "@string";
pub const TYPE_HINT_AT_CLASS: &str = "@class";
pub const TYPE_HINT_AT_METHODREF: &str = "@methodref";
pub const TYPE_HINT_AT_FIELDREF: &str = "@fieldref";
pub const TYPE_HINT_AT_INTERFACE_METHODREF: &str = "@interface_methodref";
pub const TYPE_HINT_AT_FLOAT: &str = "@float";
pub const TYPE_HINT_AT_LONG: &str = "@long";
pub const TYPE_HINT_AT_DOUBLE: &str = "@double";
pub const TYPE_HINT_AT_NAME_AND_TYPE: &str = "@name_and_type";
pub const TYPE_HINT_AT_METHOD_HANDLE: &str = "@method_handle";
pub const TYPE_HINT_AT_METHOD_TYPE: &str = "@method_type";
pub const TYPE_HINT_AT_DYNAMIC: &str = "@dynamic";
pub const TYPE_HINT_AT_INVOKE_DYNAMIC: &str = "@invoke_dynamic";
pub const TYPE_HINT_AT_MODULE: &str = "@module";
pub const TYPE_HINT_AT_PACKAGE: &str = "@package";

pub const TYPE_HINT_CP_IDX: &str = "cp_idx";
pub const TYPE_HINT_UTF8: &str = "utf8";
pub const TYPE_HINT_INTEGER: &str = "int";
pub const TYPE_HINT_STRING: &str = "string";
pub const TYPE_HINT_CLASS: &str = "class";
pub const TYPE_HINT_METHODREF: &str = "methodref";
pub const TYPE_HINT_FIELDREF: &str = "fieldref";
pub const TYPE_HINT_INTERFACE_METHODREF: &str = "interface_methodref";
pub const TYPE_HINT_FLOAT: &str = "float";
pub const TYPE_HINT_LONG: &str = "long";
pub const TYPE_HINT_DOUBLE: &str = "double";
pub const TYPE_HINT_NAME_AND_TYPE: &str = "name_and_type";
pub const TYPE_HINT_METHOD_HANDLE: &str = "method_handle";
pub const TYPE_HINT_METHOD_TYPE: &str = "method_type";
pub const TYPE_HINT_DYNAMIC: &str = "dynamic";
pub const TYPE_HINT_INVOKE_DYNAMIC: &str = "invoke_dynamic";
pub const TYPE_HINT_MODULE: &str = "module";
pub const TYPE_HINT_PACKAGE: &str = "package";

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TypeHintKind {
    CpIndex,
    Utf8,
    Integer,
    String,
    Class,
    Methodref,
    Fieldref,
    InterfaceMethodref,
    Float,
    Long,
    Double,
    NameAndType,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
}

impl TypeHintKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            TYPE_HINT_CP_IDX => Some(Self::CpIndex),
            TYPE_HINT_UTF8 => Some(Self::Utf8),
            TYPE_HINT_INTEGER => Some(Self::Integer),
            TYPE_HINT_STRING => Some(Self::String),
            TYPE_HINT_CLASS => Some(Self::Class),
            TYPE_HINT_METHODREF => Some(Self::Methodref),
            TYPE_HINT_FIELDREF => Some(Self::Fieldref),
            TYPE_HINT_INTERFACE_METHODREF => Some(Self::InterfaceMethodref),
            TYPE_HINT_FLOAT => Some(Self::Float),
            TYPE_HINT_LONG => Some(Self::Long),
            TYPE_HINT_DOUBLE => Some(Self::Double),
            TYPE_HINT_NAME_AND_TYPE => Some(Self::NameAndType),
            TYPE_HINT_METHOD_HANDLE => Some(Self::MethodHandle),
            TYPE_HINT_METHOD_TYPE => Some(Self::MethodType),
            TYPE_HINT_DYNAMIC => Some(Self::Dynamic),
            TYPE_HINT_INVOKE_DYNAMIC => Some(Self::InvokeDynamic),
            TYPE_HINT_MODULE => Some(Self::Module),
            TYPE_HINT_PACKAGE => Some(Self::Package),
            _ => None,
        }
    }

    pub fn operands_count(&self) -> usize {
        match self {
            Self::CpIndex
            | Self::Utf8
            | Self::Integer
            | Self::Long
            | Self::Float
            | Self::Double
            | Self::String
            | Self::Class => 1,
            Self::Methodref | Self::Fieldref => 3,
            _ => unimplemented!(),
        }
    }

    pub fn expected_argument_types(&self) -> &'static [&'static str] {
        match self {
            Self::CpIndex => &["unsigned 16-bit integer"],
            Self::Utf8 => &["identifier"],
            Self::Integer => &["integer"],
            Self::Long => &["integer"],
            Self::Float => &["float"],
            Self::Double => &["double"],
            Self::String => &["string literal"],
            Self::Class => &["identifier"],
            Self::Methodref => &[
                "identifier (class name)",
                "identifier (method name)",
                "identifier (method descriptor)",
            ],
            _ => unimplemented!(),
        }
    }

    pub fn operand_names(&self) -> &'static [TypeHintOperandName] {
        match self {
            Self::CpIndex => &[TypeHintOperandName::U16Literal],
            Self::Utf8 => &[TypeHintOperandName::Utf8Entry],
            Self::Integer => &[TypeHintOperandName::I32Literal],
            Self::Float => &[TypeHintOperandName::F32Literal],
            Self::Long => &[TypeHintOperandName::I64Literal],
            Self::Double => &[TypeHintOperandName::F64Literal],
            Self::String => &[TypeHintOperandName::StringLiteral],
            Self::Class => &[TypeHintOperandName::ClassName],
            Self::Methodref => &[
                TypeHintOperandName::ClassName,
                TypeHintOperandName::MethodName,
                TypeHintOperandName::MethodDescriptor,
            ],
            Self::Fieldref => &[
                TypeHintOperandName::ClassName,
                TypeHintOperandName::FieldName,
                TypeHintOperandName::FieldDescriptor,
            ],
            _ => unimplemented!(),
        }
    }

    pub fn context_label(&self) -> &'static str {
        match self {
            Self::CpIndex => "forces explicit constant pool index",
            Self::Utf8 => "forces explicit utf8 constant pool type",
            Self::Integer => "forces explicit integer constant pool type",
            Self::Float => "forces explicit float constant pool type",
            Self::Long => "forces explicit long constant pool type",
            Self::Double => "forces explicit double constant pool type",
            Self::String => "forces explicit string constant pool type",
            Self::Class => "forces explicit class constant pool type",
            Self::Methodref => "forces explicit method reference constant pool type",
            Self::Fieldref => "forces explicit field reference constant pool type",
            _ => unimplemented!(),
        }
    }

    pub fn example(&self) -> &'static str {
        match self {
            Self::CpIndex => "@cp_idx 12",
            Self::Utf8 => "@utf8 HelloWorld",
            Self::Integer => "@int 42",
            Self::Float => "@float 3.14",
            Self::Long => "@long 100000",
            Self::Double => "@double 3.14",
            Self::String => "@string \"Hello, World!\"",
            Self::Class => "@class java/lang/Object",
            Self::Methodref => "@methodref java/io/PrintStream println (Ljava/lang/String;)V",
            Self::Fieldref => "@fieldref java/lang/System out Ljava/io/PrintStream;",
            _ => unimplemented!(),
        }
    }

    pub fn token_name(&self) -> &'static str {
        match self {
            Self::CpIndex => TYPE_HINT_AT_CP_IDX,
            Self::Utf8 => TYPE_HINT_AT_UTF8,
            Self::Integer => TYPE_HINT_AT_INTEGER,
            Self::String => TYPE_HINT_AT_STRING,
            Self::Class => TYPE_HINT_AT_CLASS,
            Self::Methodref => TYPE_HINT_AT_METHODREF,
            Self::Fieldref => TYPE_HINT_AT_FIELDREF,
            Self::InterfaceMethodref => TYPE_HINT_AT_INTERFACE_METHODREF,
            Self::Float => TYPE_HINT_AT_FLOAT,
            Self::Long => TYPE_HINT_AT_LONG,
            Self::Double => TYPE_HINT_AT_DOUBLE,
            Self::NameAndType => TYPE_HINT_AT_NAME_AND_TYPE,
            Self::MethodHandle => TYPE_HINT_AT_METHOD_HANDLE,
            Self::MethodType => TYPE_HINT_AT_METHOD_TYPE,
            Self::Dynamic => TYPE_HINT_AT_DYNAMIC,
            Self::InvokeDynamic => TYPE_HINT_AT_INVOKE_DYNAMIC,
            Self::Module => TYPE_HINT_AT_MODULE,
            Self::Package => TYPE_HINT_AT_PACKAGE,
        }
    }

    pub fn numeric_kind(&self) -> &'static str {
        match self {
            Self::CpIndex | Self::Integer | Self::Long => "integer",
            Self::Float | Self::Double => "float",
            _ => unreachable!("numeric_kind() called on non-numeric type hint"),
        }
    }

    pub fn bit_width(&self) -> &'static str {
        match self {
            Self::CpIndex => "16-bit",
            Self::Integer | Self::Float => "32-bit",
            Self::Long | Self::Double => "64-bit",
            _ => unreachable!("bit_width() called on non-numeric type hint"),
        }
    }

    pub fn range_description(&self) -> String {
        match self {
            Self::CpIndex => format!(
                "{} integer range is {} to {}",
                self.bit_width(),
                u16::MIN,
                u16::MAX
            ),
            Self::Integer => format!(
                "{} integer range is {} to {}",
                self.bit_width(),
                i32::MIN,
                i32::MAX
            ),
            Self::Long => format!(
                "{} integer range is {} to {}",
                self.bit_width(),
                i64::MIN,
                i64::MAX
            ),
            Self::Float => format!(
                "{} float range is {:.7e} to {:.7e}",
                self.bit_width(),
                f32::MIN,
                f32::MAX
            ),
            Self::Double => format!(
                "{} float range is {:.15e} to {:.15e}",
                self.bit_width(),
                f64::MIN,
                f64::MAX
            ),
            _ => unreachable!("range_description() called on non-numeric type hint"),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::CpIndex => TYPE_HINT_CP_IDX,
            Self::Utf8 => TYPE_HINT_UTF8,
            Self::Integer => TYPE_HINT_INTEGER,
            Self::String => TYPE_HINT_STRING,
            Self::Class => TYPE_HINT_CLASS,
            Self::Methodref => TYPE_HINT_METHODREF,
            Self::Fieldref => TYPE_HINT_FIELDREF,
            Self::InterfaceMethodref => TYPE_HINT_INTERFACE_METHODREF,
            Self::Float => TYPE_HINT_FLOAT,
            Self::Long => TYPE_HINT_LONG,
            Self::Double => TYPE_HINT_DOUBLE,
            Self::NameAndType => TYPE_HINT_NAME_AND_TYPE,
            Self::MethodHandle => TYPE_HINT_METHOD_HANDLE,
            Self::MethodType => TYPE_HINT_METHOD_TYPE,
            Self::Dynamic => TYPE_HINT_DYNAMIC,
            Self::InvokeDynamic => TYPE_HINT_INVOKE_DYNAMIC,
            Self::Module => TYPE_HINT_MODULE,
            Self::Package => TYPE_HINT_PACKAGE,
        }
    }
}

impl Display for TypeHintKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_name())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefTypeHint {
    pub hint_span: Option<Span>,
    pub class: Spanned<String>,
    pub name: Spanned<String>,
    pub descriptor: Spanned<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeHint {
    CpIndex(Span, Spanned<u16>),
    Utf8(Option<Span>, Spanned<String>),
    Integer(Option<Span>, Spanned<i32>),
    String(Option<Span>, Spanned<String>),
    Class(Option<Span>, Spanned<String>),
    Methodref(Box<RefTypeHint>),
    Fieldref(Box<RefTypeHint>),
    InterfaceMethodref,
    Float(Option<Span>, Spanned<f32>),
    Long(Option<Span>, Spanned<i64>),
    Double(Option<Span>, Spanned<f64>),
    NameAndType,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
}

impl TypeHint {
    pub fn token_name_with_value(&self) -> String {
        match self {
            Self::CpIndex(_, value) => format!("{} {}", TYPE_HINT_AT_CP_IDX, value.value),
            Self::Utf8(_, value) => format!("{} {}", TYPE_HINT_AT_UTF8, value.value),
            Self::Integer(_, value) => format!("{} {}", TYPE_HINT_AT_INTEGER, value.value),
            Self::String(_, value) => format!("{} {}", TYPE_HINT_AT_STRING, value.value),
            Self::Class(_, value) => format!("{} {}", TYPE_HINT_AT_CLASS, value.value),
            Self::Methodref(r) => format!(
                "{} class: {}, name: {}, descriptor: {}",
                TYPE_HINT_AT_METHODREF, r.class.value, r.name.value, r.descriptor.value
            ),
            Self::Fieldref(r) => format!(
                "{} class: {}, name: {}, descriptor: {}",
                TYPE_HINT_AT_FIELDREF, r.class.value, r.name.value, r.descriptor.value
            ),
            Self::Long(_, value) => format!("{} {}", TYPE_HINT_AT_LONG, value.value),
            Self::Float(_, value) => format!("{} {}", TYPE_HINT_AT_FLOAT, value.value),
            Self::Double(_, value) => format!("{} {}", TYPE_HINT_AT_DOUBLE, value.value),
            _ => unimplemented!(),
        }
    }

    pub fn value(&self) -> String {
        match self {
            Self::CpIndex(_, value) => value.value.to_string(),
            Self::Integer(_, value) => value.value.to_string(),
            Self::Long(_, value) => format!("{} {}", TYPE_HINT_AT_LONG, value.value),
            Self::Float(_, value) => format!("{} {}", TYPE_HINT_AT_FLOAT, value.value),
            Self::Double(_, value) => format!("{} {}", TYPE_HINT_AT_DOUBLE, value.value),
            Self::Utf8(_, value) | Self::String(_, value) | Self::Class(_, value) => {
                value.value.to_string()
            }
            Self::Methodref(r) | Self::Fieldref(r) => {
                format!("{} {} {}", r.class.value, r.name.value, r.descriptor.value)
            }
            _ => unimplemented!(),
        }
    }
}
