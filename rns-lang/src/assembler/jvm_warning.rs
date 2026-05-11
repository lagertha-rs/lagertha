use crate::ast::flag::RnsClassFlag;
use crate::diagnostic::{
    DiagnosticLabel, DiagnosticTier, IntoDiagnostic, JVMS_CODE_1, jvms_docs_note,
};
use crate::token::{RnsFlag, Span};
use std::borrow::Cow;

#[derive(Debug)]
pub(super) enum JvmWarning {
    InterfaceFlagWithMissingAbstract {
        interface_span: Span,
    },
    InterfaceMutuallyExclusive {
        interface_span: Span,
        exclusive_flags: Vec<(RnsClassFlag, Span)>,
    },
}

impl IntoDiagnostic for JvmWarning {
    fn code(&self) -> &'static str {
        match self {
            JvmWarning::InterfaceFlagWithMissingAbstract { .. }
            | JvmWarning::InterfaceMutuallyExclusive { .. } => JVMS_CODE_1,
        }
    }

    fn asm_msg(&self) -> Cow<'static, str> {
        match self {
            JvmWarning::InterfaceFlagWithMissingAbstract { .. } => {
                "interface must also be declared as 'abstract'".into()
            }
            JvmWarning::InterfaceMutuallyExclusive { .. } => {
                "interface cannot be declared with mutually exclusive flags".into()
            }
        }
    }

    fn primary_location(&self) -> Span {
        match self {
            JvmWarning::InterfaceFlagWithMissingAbstract { interface_span } => *interface_span,
            JvmWarning::InterfaceMutuallyExclusive { interface_span, .. } => *interface_span,
        }
    }

    fn note(&self) -> Option<Cow<'static, str>> {
        Some(jvms_docs_note(self.code()))
    }

    fn help(&self) -> Option<Cow<'static, str>> {
        match self {
            JvmWarning::InterfaceFlagWithMissingAbstract { .. } => {
                Some("Add the 'abstract' access flag to the class definition.".into())
            }
            JvmWarning::InterfaceMutuallyExclusive {
                exclusive_flags, ..
            } => {
                let flags_list = exclusive_flags
                    .iter()
                    .map(|(flag, _)| format!("'{}'", flag.jvm_spec_name()))
                    .collect::<Vec<_>>()
                    .join(", ");
                Some(format!("Consider removing: {}.", flags_list).into())
            }
        }
    }

    fn labels(&self) -> Vec<DiagnosticLabel> {
        match self {
            JvmWarning::InterfaceFlagWithMissingAbstract { interface_span } => {
                vec![DiagnosticLabel::at(
                    interface_span.as_range(),
                    format!(
                        "'{}' requires '{}'",
                        RnsFlag::Interface.jvm_spec_name(),
                        RnsFlag::Abstract.jvm_spec_name()
                    ),
                )]
            }
            JvmWarning::InterfaceMutuallyExclusive {
                interface_span,
                exclusive_flags,
            } => {
                let mut labels = vec![DiagnosticLabel::context(
                    interface_span.as_range(),
                    format!("'{}' is declared here", RnsFlag::Interface.jvm_spec_name()),
                )];
                for (flag, span) in exclusive_flags {
                    labels.push(DiagnosticLabel::at(
                        span.as_range(),
                        format!("'{}' is exclusive", flag.jvm_spec_name()),
                    ));
                }
                labels
            }
        }
    }

    fn tier(&self) -> DiagnosticTier {
        DiagnosticTier::JvmSpecWarn
    }
}
