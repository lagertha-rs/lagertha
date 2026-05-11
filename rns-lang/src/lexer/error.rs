use crate::diagnostic::{
    DiagnosticLabel, DiagnosticTier, ERR_CODE_INVALID_ESCAPE, ERR_CODE_INVALID_TYPE_HINT,
    ERR_CODE_UNCLOSED_IDENT, ERR_CODE_UNKNOWN_DIR, IntoDiagnostic, docs_note,
};
use crate::suggestion;
use crate::token::Span;
use std::borrow::Cow;

//TODO: same error code for all lexer, try to categorize later if needed

#[derive(Debug, Eq, PartialEq, Clone)]
pub(super) enum LexerError {
    UnknownDirective(Span, String),
    UnterminatedQuotedIdentifier(Span),
    UnknownTypeHint(Span, String),
    InvalidEscape(Span, char),
}

impl IntoDiagnostic for LexerError {
    fn code(&self) -> &'static str {
        match self {
            LexerError::UnterminatedQuotedIdentifier(_) => ERR_CODE_UNCLOSED_IDENT,
            LexerError::UnknownDirective(_, _) => ERR_CODE_UNKNOWN_DIR,
            LexerError::InvalidEscape(_, _) => ERR_CODE_INVALID_ESCAPE,
            LexerError::UnknownTypeHint(_, _) => ERR_CODE_INVALID_TYPE_HINT,
        }
    }

    fn asm_msg(&self) -> Cow<'static, str> {
        match self {
            LexerError::UnknownDirective(_, _) => "unknown directive".into(),
            LexerError::UnterminatedQuotedIdentifier(_) => "unterminated quoted identifier".into(),
            LexerError::InvalidEscape(_, _) => "invalid escape sequence".into(),
            LexerError::UnknownTypeHint(_, _) => "unknown type hint".into(),
        }
    }

    fn lsp_msg(&self) -> Cow<'static, str> {
        match self {
            LexerError::UnknownDirective(_, name) => format!("unknown directive '{name}'").into(),
            LexerError::UnknownTypeHint(_, name) => format!("unknown type hint '{name}'").into(),
            _ => self.asm_msg(),
        }
    }

    fn note(&self) -> Option<Cow<'static, str>> {
        Some(docs_note(self.code()))
    }

    fn labels(&self) -> Vec<DiagnosticLabel> {
        match self {
            LexerError::UnknownTypeHint(_, name) => {
                vec![DiagnosticLabel::at(
                    self.primary_location().as_range(),
                    format!("'{name}' is not a recognized type hint"),
                )]
            }
            LexerError::UnknownDirective(_, name) => {
                vec![DiagnosticLabel::at(
                    self.primary_location().as_range(),
                    format!("'{name}' is not a recognized directive"),
                )]
            }
            LexerError::UnterminatedQuotedIdentifier(_) => {
                vec![DiagnosticLabel::at(
                    self.primary_location().as_range(),
                    "this identifier is not terminated",
                )]
            }
            LexerError::InvalidEscape(_, c) if *c == '\n' => vec![DiagnosticLabel::at(
                self.primary_location().as_range(),
                "newline characters cannot be escaped",
            )],
            LexerError::InvalidEscape(_, c) if *c == '\r' => vec![DiagnosticLabel::at(
                self.primary_location().as_range(),
                "carriage return characters cannot be escaped",
            )],
            LexerError::InvalidEscape(_, c) => vec![DiagnosticLabel::at(
                self.primary_location().as_range(),
                format!("'\\{}' is not a valid escape sequence", c.escape_default()),
            )],
        }
    }

    fn help(&self) -> Option<Cow<'static, str>> {
        match self {
            LexerError::UnknownDirective(_, name) => suggestion::closest_directive(name)
                .map(|closest| format!("Did you mean '{}'?", closest).into()),
            LexerError::UnknownTypeHint(_, name) => {
                let bare = name.strip_prefix('@').unwrap_or(name);
                suggestion::closest_type_hint(bare)
                    .map(|closest| format!("Did you mean '@{}'?", closest).into())
            }
            LexerError::InvalidEscape(_, c) if *c == '\n' || *c == '\r' => Some(
                "Multiline identifiers are not supported yet, but are planned for the future."
                    .into(),
            ),
            LexerError::InvalidEscape(_, _) => None,
            LexerError::UnterminatedQuotedIdentifier(_) => {
                Some("Close the identifier with a '\"' on the same line.".into())
            }
        }
    }

    fn primary_location(&self) -> Span {
        match self {
            LexerError::UnknownDirective(span, _)
            | LexerError::UnterminatedQuotedIdentifier(span)
            | LexerError::InvalidEscape(span, _)
            | LexerError::UnknownTypeHint(span, _) => *span,
        }
    }

    fn tier(&self) -> DiagnosticTier {
        DiagnosticTier::SyntaxError
    }
}
