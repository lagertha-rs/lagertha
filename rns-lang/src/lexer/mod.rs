use crate::diagnostic::Diagnostic;
use crate::lexer::error::LexerError;
use crate::token::type_hint::TypeHintKind;
use crate::token::{RnsToken, Span, Spanned};

mod error;
#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod tests;

pub fn tokenize(source: &str) -> (Vec<RnsToken>, Vec<Diagnostic>, Span) {
    RnsLexer::new(source).tokenize()
}

struct RnsLexer<'a> {
    source: &'a str,
    byte_pos: usize,
    col_pos: usize,
    line: usize,
}

impl<'a> RnsLexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            byte_pos: 0,
            col_pos: 0,
            line: 0,
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.source[self.byte_pos..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.peek_char() {
            self.byte_pos += c.len_utf8();
            if c == '\n' {
                self.line += 1;
                self.col_pos = 0;
            } else {
                self.col_pos += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespaces_and_comments(&mut self) {
        while let Some(c) = self.peek_char() {
            match c {
                ' ' | '\t' | '\r' => {
                    self.next_char();
                }
                ';' => {
                    self.next_char();
                    while let Some(c2) = self.peek_char() {
                        if c2 != '\n' {
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                }
                _ => break,
            }
        }
    }

    fn is_delimiter(c: char) -> bool {
        c.is_whitespace()
    }

    fn extend_to_delimiter(&mut self, mut start: String) -> String {
        while let Some(c) = self.peek_char() {
            if !Self::is_delimiter(c) {
                start.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        start
    }

    fn read_to_delimiter(&mut self) -> String {
        self.extend_to_delimiter(String::new())
    }

    fn span_for_current_position(&self, byte_start: usize, col_start: usize) -> Span {
        Span {
            byte_start,
            byte_end: self.byte_pos,
            line: self.line,
            col_start,
            col_end: self.col_pos,
        }
    }

    fn span_for_eof(&self, byte_start: usize, col_start: usize) -> Span {
        Span {
            byte_start,
            byte_end: byte_start,
            line: self.line,
            col_start,
            col_end: col_start,
        }
    }

    fn span_for_one_char(&self, byte_start: usize, col_start: usize) -> Span {
        Span {
            byte_start,
            byte_end: byte_start + 1,
            line: self.line,
            col_start,
            col_end: col_start + 1,
        }
    }

    fn read_string(
        &mut self,
        byte_start: usize,
        col_start: usize,
    ) -> Result<Spanned<String>, LexerError> {
        let mut result = String::new();

        self.next_char(); // consume opening quote

        while let Some(c) = self.peek_char() {
            match c {
                '"' => {
                    self.next_char(); // consume closing quote
                    return Ok(Spanned::new(
                        result,
                        self.span_for_current_position(byte_start, col_start),
                    ));
                }
                '\n' | '\r' => break,
                '\\' => {
                    self.next_char(); // consume '\'
                    if let Some(next_char) = self.peek_char() {
                        match next_char {
                            'n' => result.push('\n'),
                            't' => result.push('\t'),
                            'r' => result.push('\r'),
                            '0' => result.push('\0'),
                            'b' => result.push('\x08'), // backspace
                            'f' => result.push('\x0C'), // form feed
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            _ => {
                                return Err(LexerError::InvalidEscape(
                                    Span {
                                        byte_start: self.byte_pos - 1,
                                        byte_end: self.byte_pos + next_char.len_utf8(),
                                        line: self.line,
                                        col_start: self.col_pos - 1,
                                        col_end: self.col_pos + 1,
                                    },
                                    next_char,
                                ));
                            }
                        }
                        self.next_char(); // consume escaped character
                    } else {
                        break;
                    }
                }
                _ => {
                    result.push(c);
                    self.next_char();
                }
            }
        }

        // point to the opening quote for better error highlighting
        Err(LexerError::UnterminatedQuotedIdentifier(
            self.span_for_one_char(byte_start, col_start),
        ))
    }

    fn handle_directive(&mut self) -> Result<RnsToken, LexerError> {
        let byte_start = self.byte_pos;
        let col_start = self.col_pos;

        self.next_char(); // consume '.'

        let directive_name = self.read_to_delimiter();
        let directive_span = self.span_for_current_position(byte_start, col_start);
        if directive_name.is_empty() {
            return Ok(RnsToken::Identifier(Spanned::new(
                ".".to_string(),
                directive_span,
            )));
        }

        RnsToken::from_directive(&directive_name, directive_span).ok_or(
            LexerError::UnknownDirective(directive_span, format!(".{directive_name}")),
        )
    }

    fn handle_type_hint(&mut self) -> Result<RnsToken, LexerError> {
        let byte_start = self.byte_pos;
        let col_start = self.col_pos;

        self.next_char(); // consume '@'

        let type_hint_str = self.read_to_delimiter();
        let type_hint_span = self.span_for_current_position(byte_start, col_start);
        if type_hint_str.is_empty() {
            return Ok(RnsToken::Identifier(Spanned::new(
                "@".to_string(),
                type_hint_span,
            )));
        }

        TypeHintKind::from_str(&type_hint_str)
            .map(|kind| RnsToken::TypeHint(Spanned::new(kind, type_hint_span)))
            .ok_or(LexerError::UnknownTypeHint(
                type_hint_span,
                format!("@{type_hint_str}"),
            ))
    }

    fn next_token(&mut self) -> Result<RnsToken, Diagnostic> {
        self.skip_whitespaces_and_comments();

        let byte_start = self.byte_pos;
        let col_start = self.col_pos;

        let Some(ch) = self.peek_char() else {
            return Ok(RnsToken::Eof(self.span_for_eof(byte_start, col_start)));
        };

        let token = match ch {
            '.' => self.handle_directive()?,
            '"' => RnsToken::Identifier(self.read_string(byte_start, col_start)?),
            '@' => self.handle_type_hint()?,
            '\n' => {
                self.next_char();
                RnsToken::Newline(Span {
                    byte_start,
                    byte_end: self.byte_pos,
                    line: self.line - 1,
                    col_start,
                    col_end: col_start + 1,
                })
            }
            _ => {
                let str = self.read_to_delimiter();
                RnsToken::from_identifier(
                    str,
                    self.span_for_current_position(byte_start, col_start),
                )
            }
        };
        Ok(token)
    }

    fn skip_to_end_of_line(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == '\n' {
                break;
            }
            self.next_char();
        }
    }

    fn tokenize(&mut self) -> (Vec<RnsToken>, Vec<Diagnostic>, Span) {
        let mut tokens = Vec::new();
        let mut diagnostics = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    if let RnsToken::Eof(eof_span) = token {
                        return (tokens, diagnostics, eof_span);
                    }
                    // TODO: can I skip multiple newlines in a row? do I need to keep them all?
                    tokens.push(token);
                }
                Err(e) => {
                    diagnostics.push(e);
                    self.skip_to_end_of_line();
                }
            }
        }
    }
}
