use crate::ast::flag::{RnsClassFlag, RnsMethodFlag};
use crate::ast::{
    ClassDirective, CodeDirective, MethodDirective, RnsInstruction, RnsModule, RnsOperand,
    SuperDirective,
};
use crate::diagnostic::Diagnostic;
use crate::instruction::{INSTRUCTION_SPECS, InstructionOperand};
use crate::parser::error::{
    AccessFlagContext, NumericRejection, OperandErrPosContext, ParseNumeric, ParserError,
    TrailingTokensErrContext, UnexpectedTokenContext,
};
use crate::parser::warning::ParserWarning;
use crate::token::type_hint::{RefTypeHint, TypeHint, TypeHintKind, TypeHintOperandName};
use crate::token::{RnsToken, RnsTokenKind, Span, Spanned};
use std::collections::{BTreeMap, HashMap};
use std::iter::Peekable;
use std::vec::IntoIter;

mod error;
#[cfg(test)]
mod tests;
mod warning;

const JAVA_LANG_OBJECT: &str = "java/lang/Object";

enum TypeHintOrigin {
    Explicit(Spanned<TypeHintKind>),
    Implicit(OperandErrPosContext, TypeHintKind),
}

impl TypeHintOrigin {
    fn kind(&self) -> TypeHintKind {
        match self {
            TypeHintOrigin::Explicit(th) => th.value,
            TypeHintOrigin::Implicit(_, kind) => *kind,
        }
    }

    fn kind_span(&self) -> Option<Span> {
        match self {
            TypeHintOrigin::Explicit(th) => Some(th.span),
            TypeHintOrigin::Implicit(_, _) => None,
        }
    }
}

struct RnsParser {
    tokens: Peekable<IntoIter<RnsToken>>,
    eof_span: Span,
    last_kind: RnsTokenKind,
    last_span: Span,

    diagnostic: Vec<Diagnostic>,

    class_dir_span: Span,
    class_name: Option<TypeHint>,
    access_flags: BTreeMap<RnsClassFlag, Span>,
    method_dirs: Vec<MethodDirective>,

    super_directives: Vec<(Span, TypeHint)>,
    reported_errs: ReportedErrs,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum ErrKind {
    Super = 0,
    CodeStack = 1,
    CodeLocals = 2,
}

#[derive(Default, Clone, Copy)]
struct ReportedErrs(u32);

impl ReportedErrs {
    fn insert(&mut self, kind: ErrKind) {
        self.0 |= 1 << kind as u8;
    }

    fn contains(self, kind: ErrKind) -> bool {
        self.0 & (1 << kind as u8) != 0
    }
}

impl RnsParser {
    fn peek_token(&mut self) -> Option<&RnsToken> {
        self.tokens.peek()
    }

    fn skip_newlines(&mut self) {
        while let Some(RnsToken::Newline(_)) = self.peek_token() {
            self.next_token();
        }
    }

    fn next_token(&mut self) -> RnsToken {
        match self.tokens.next() {
            Some(token) => {
                self.last_span = token.span();
                self.last_kind = token.kind();
                token
            }
            None => {
                self.last_span = self.eof_span;
                self.last_kind = RnsTokenKind::Eof;
                RnsToken::Eof(self.eof_span)
            }
        }
    }

    fn next_until_newline(&mut self) -> Vec<RnsToken> {
        let mut tokens = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if matches!(token, RnsToken::Newline(_) | RnsToken::Eof(_)) {
                break;
            }
            tokens.push(self.next_token());
        }
        tokens
    }

    fn parse_access_flags<F: Ord>(
        &mut self,
        ctx: AccessFlagContext,
        convert: fn(&crate::token::flag::RnsFlag) -> Option<F>,
    ) -> BTreeMap<F, Span> {
        let mut flags: BTreeMap<F, (crate::token::flag::RnsFlag, Vec<Span>)> = BTreeMap::new();
        loop {
            match self.peek_token() {
                Some(token) if token.is_access_flag() => {
                    let next_token = self.next_token();
                    let next_token_span = next_token.span();
                    if let RnsToken::AccessFlag(spanned) = next_token {
                        if let Some(flag) = convert(&spanned.value) {
                            flags
                                .entry(flag)
                                .or_insert_with(|| (spanned.value, Vec::new()))
                                .1
                                .push(next_token_span);
                        } else {
                            self.diagnostic
                                .push(ParserError::InvalidAccessFlag(ctx, spanned).into())
                        }
                    }
                }
                _ => break,
            }
        }
        flags
            .into_iter()
            .map(|(k, (rns_flag, spans))| {
                let first_span = spans[0];
                if spans.len() > 1 {
                    self.diagnostic.push(
                        ParserWarning::DuplicateAccessFlag {
                            ctx,
                            flag: rns_flag,
                            spans,
                        }
                        .into(),
                    );
                }
                (k, first_span)
            })
            .collect()
    }

    fn parse_class_access_flags(&mut self) -> BTreeMap<RnsClassFlag, Span> {
        self.parse_access_flags(AccessFlagContext::Class, |f| f.as_class_flag())
    }

    fn parse_method_access_flags(&mut self) -> BTreeMap<RnsMethodFlag, Span> {
        self.parse_access_flags(AccessFlagContext::Method, |f| f.as_method_flag())
    }

    fn try_next_identifier(&mut self) -> Result<Spanned<String>, RnsToken> {
        // don't consume to not break trailing tokens check
        if let Some(next) = self.peek_token()
            && matches!(next, RnsToken::Eof(_) | RnsToken::Newline(_))
        {
            return Err(next.clone());
        }
        let token = self.next_token();
        match token {
            RnsToken::Identifier(spanned) => Ok(spanned),
            RnsToken::Eof(_) | RnsToken::Newline(_) => Err(token),
            identifier_like => {
                self.diagnostic
                    .push(ParserWarning::ReservedLikeIdentifierTodoName.into());
                Ok(Spanned::new(
                    identifier_like.token_name().to_string(),
                    identifier_like.span(),
                ))
            }
        }
    }

    fn try_next_numeric<T: ParseNumeric>(&mut self) -> Result<Spanned<T>, NumericRejection> {
        // Don't consume EOF/newline to not break trailing tokens check
        if let Some(next) = self.peek_token()
            && matches!(next, RnsToken::Eof(_) | RnsToken::Newline(_))
        {
            return Err(NumericRejection::Missing(next.clone()));
        }
        let token = self.next_token();
        match token {
            RnsToken::Identifier(ref spanned) => {
                let raw = &spanned.value;
                T::parse_and_classify(raw, spanned).map(|value| Spanned::new(value, spanned.span))
            }
            RnsToken::Eof(_) | RnsToken::Newline(_) => Err(NumericRejection::Missing(token)),
            _ => {
                let spanned = Spanned::new(token.as_identifier().to_string(), token.span());
                Err(NumericRejection::NotNumeric(spanned))
            }
        }
    }

    fn parse_identifier(
        &mut self,
        err_ctx: OperandErrPosContext,
    ) -> Result<Spanned<String>, Diagnostic> {
        let prev_token_span = self.last_span;
        self.try_next_identifier().map_err(|token| {
            ParserError::IdentifierOrHintExpected(prev_token_span, token, err_ctx).into()
        })
    }

    fn parse_hint_identifier(
        &mut self,
        origin: &TypeHintOrigin,
        operand: TypeHintOperandName,
    ) -> Result<Spanned<String>, ParserError> {
        let after_span = self.last_span;
        self.try_next_identifier().map_err(|_| match origin {
            TypeHintOrigin::Explicit(th) => ParserError::MissingTypeHintOperand {
                type_hint: th.clone(),
                operand,
                after_span,
            },
            TypeHintOrigin::Implicit(ctx, kind) => ParserError::MissingImplicitTypeHintOperand {
                err_ctx: ctx.clone(),
                implicit_kind: *kind,
                operand,
                after_span,
            },
        })
    }

    fn parse_type_hint_u16(
        &mut self,
        type_hint: Spanned<TypeHintKind>,
    ) -> Result<Spanned<u16>, ParserError> {
        let after_span = self.last_span;
        self.try_next_numeric::<u16>()
            .map_err(|rejection| match rejection {
                NumericRejection::Missing(_) => ParserError::MissingTypeHintOperand {
                    type_hint,
                    operand: TypeHintOperandName::U16Literal,
                    after_span,
                },
                other => ParserError::TypeHintExpectsNumericOperand {
                    type_hint,
                    rejection: other,
                },
            })
    }

    fn parse_type_hint_i32(
        &mut self,
        type_hint: Spanned<TypeHintKind>,
    ) -> Result<Spanned<i32>, ParserError> {
        let after_span = self.last_span;
        self.try_next_numeric::<i32>()
            .map_err(|rejection| match rejection {
                NumericRejection::Missing(_) => ParserError::MissingTypeHintOperand {
                    type_hint,
                    operand: TypeHintOperandName::I32Literal,
                    after_span,
                },
                other => ParserError::TypeHintExpectsNumericOperand {
                    type_hint,
                    rejection: other,
                },
            })
    }

    fn parse_type_hint_i64(
        &mut self,
        type_hint: Spanned<TypeHintKind>,
    ) -> Result<Spanned<i64>, ParserError> {
        let after_span = self.last_span;
        self.try_next_numeric::<i64>()
            .map_err(|rejection| match rejection {
                NumericRejection::Missing(_) => ParserError::MissingTypeHintOperand {
                    type_hint,
                    operand: TypeHintOperandName::I64Literal,
                    after_span,
                },
                other => ParserError::TypeHintExpectsNumericOperand {
                    type_hint,
                    rejection: other,
                },
            })
    }

    fn parse_type_hint_f32(
        &mut self,
        type_hint: Spanned<TypeHintKind>,
    ) -> Result<Spanned<f32>, ParserError> {
        let after_span = self.last_span;
        self.try_next_numeric::<f32>()
            .map_err(|rejection| match rejection {
                NumericRejection::Missing(_) => ParserError::MissingTypeHintOperand {
                    type_hint,
                    operand: TypeHintOperandName::F32Literal,
                    after_span,
                },
                other => ParserError::TypeHintExpectsNumericOperand {
                    type_hint,
                    rejection: other,
                },
            })
    }

    fn parse_type_hint_f64(
        &mut self,
        type_hint: Spanned<TypeHintKind>,
    ) -> Result<Spanned<f64>, ParserError> {
        let after_span = self.last_span;
        self.try_next_numeric::<f64>()
            .map_err(|rejection| match rejection {
                NumericRejection::Missing(_) => ParserError::MissingTypeHintOperand {
                    type_hint,
                    operand: TypeHintOperandName::F64Literal,
                    after_span,
                },
                other => ParserError::TypeHintExpectsNumericOperand {
                    type_hint,
                    rejection: other,
                },
            })
    }

    fn resolve_identifier_type_hint(
        &mut self,
        origin: &TypeHintOrigin,
    ) -> Result<TypeHint, Diagnostic> {
        let kind = origin.kind();
        let kind_span = origin.kind_span();
        match kind {
            TypeHintKind::Utf8 => Ok(TypeHint::Utf8(
                kind_span,
                self.parse_hint_identifier(origin, TypeHintOperandName::Utf8Entry)?,
            )),
            TypeHintKind::String => Ok(TypeHint::String(
                kind_span,
                self.parse_hint_identifier(origin, TypeHintOperandName::StringLiteral)?,
            )),
            TypeHintKind::Class => Ok(TypeHint::Class(
                kind_span,
                self.parse_hint_identifier(origin, TypeHintOperandName::ClassName)?,
            )),
            TypeHintKind::Methodref => {
                let class = self.parse_hint_identifier(origin, TypeHintOperandName::ClassName)?;
                let name = self.parse_hint_identifier(origin, TypeHintOperandName::MethodName)?;
                let descriptor =
                    self.parse_hint_identifier(origin, TypeHintOperandName::MethodDescriptor)?;
                Ok(TypeHint::Methodref(Box::new(RefTypeHint {
                    hint_span: kind_span,
                    class,
                    name,
                    descriptor,
                })))
            }
            TypeHintKind::Fieldref => {
                let class = self.parse_hint_identifier(origin, TypeHintOperandName::ClassName)?;
                let name = self.parse_hint_identifier(origin, TypeHintOperandName::FieldName)?;
                let descriptor =
                    self.parse_hint_identifier(origin, TypeHintOperandName::FieldDescriptor)?;
                Ok(TypeHint::Fieldref(Box::new(RefTypeHint {
                    hint_span: kind_span,
                    class,
                    name,
                    descriptor,
                })))
            }
            _ => unimplemented!(),
        }
    }

    fn resolve_explicit_type_hint(
        &mut self,
        th: Spanned<TypeHintKind>,
    ) -> Result<TypeHint, Diagnostic> {
        let th_for_trailing = th.clone();
        let res = match th.value {
            TypeHintKind::CpIndex => Ok(TypeHint::CpIndex(th.span, self.parse_type_hint_u16(th)?)),
            TypeHintKind::Integer => Ok(TypeHint::Integer(
                Some(th.span),
                self.parse_type_hint_i32(th)?,
            )),
            TypeHintKind::Long => Ok(TypeHint::Long(Some(th.span), self.parse_type_hint_i64(th)?)),
            TypeHintKind::Float => Ok(TypeHint::Float(
                Some(th.span),
                self.parse_type_hint_f32(th)?,
            )),
            TypeHintKind::Double => Ok(TypeHint::Double(
                Some(th.span),
                self.parse_type_hint_f64(th)?,
            )),
            _ => {
                let origin = TypeHintOrigin::Explicit(th);
                self.resolve_identifier_type_hint(&origin)
            }
        };
        self.verify_trailing_tokens(TrailingTokensErrContext::TypeHint(th_for_trailing));
        res
    }

    fn parse_operand_or_type_hint(
        &mut self,
        err_ctx: OperandErrPosContext,
        implicit_kind: TypeHintKind,
    ) -> Result<TypeHint, Diagnostic> {
        if let Some(RnsToken::TypeHint(th)) = self.peek_token() {
            let th = th.clone();
            self.next_token();
            self.resolve_explicit_type_hint(th)
        } else {
            let origin = TypeHintOrigin::Implicit(err_ctx, implicit_kind);
            self.resolve_identifier_type_hint(&origin)
        }
    }

    // Doesn't need to recover, just check for trailing tokens and report error if they exist
    fn verify_trailing_tokens(&mut self, context: TrailingTokensErrContext) {
        let end_of_previous_token = self.last_span.byte_end;
        let trailing_tokens = self.next_until_newline();
        if !trailing_tokens.is_empty() {
            self.diagnostic.push(
                ParserError::TrailingTokens(end_of_previous_token, trailing_tokens, context).into(),
            );
        }
    }

    fn parse_instruction(&mut self) -> Result<RnsInstruction, Diagnostic> {
        let raw_instruction = self.parse_identifier(OperandErrPosContext::InstructionName)?;
        let instruction_spec = *INSTRUCTION_SPECS
            .get(raw_instruction.value.as_str())
            .ok_or_else(|| ParserError::UnknownInstruction(raw_instruction.clone()))?;
        let instruction = match instruction_spec.operand {
            InstructionOperand::None => {
                RnsInstruction::new_without_operand(raw_instruction.span, instruction_spec)
            }
            InstructionOperand::MethodRef => {
                let operand = self.parse_operand_or_type_hint(
                    OperandErrPosContext::InstructionOperand(instruction_spec),
                    TypeHintKind::Methodref,
                )?;
                RnsInstruction::new(
                    raw_instruction.span,
                    instruction_spec,
                    RnsOperand::CpRef(operand),
                )
            }
            InstructionOperand::FieldRef => {
                let operand = self.parse_operand_or_type_hint(
                    OperandErrPosContext::InstructionOperand(instruction_spec),
                    TypeHintKind::Fieldref,
                )?;
                RnsInstruction::new(
                    raw_instruction.span,
                    instruction_spec,
                    RnsOperand::CpRef(operand),
                )
            }
            InstructionOperand::TypeHint => {
                if let Some(RnsToken::TypeHint(th)) = self.peek_token() {
                    let th = th.clone();
                    self.next_token();
                    let operand = self.resolve_explicit_type_hint(th)?;
                    RnsInstruction::new(
                        raw_instruction.span,
                        instruction_spec,
                        RnsOperand::CpRef(operand),
                    )
                } else {
                    todo!("error: ldc requires an explicit type hint (e.g. @string, @int, @float)")
                }
            }
            InstructionOperand::Byte => {
                let value = self.try_next_numeric::<u8>().unwrap(); // TODO: proper error handling
                RnsInstruction::new(
                    raw_instruction.span,
                    instruction_spec,
                    RnsOperand::Byte(value),
                )
            }
            InstructionOperand::Label => {
                let label = self
                    .parse_identifier(OperandErrPosContext::InstructionOperand(instruction_spec))?;
                RnsInstruction::new(
                    raw_instruction.span,
                    instruction_spec,
                    RnsOperand::Label(label),
                )
            }
        };

        // TODO: verify no trailing tokens
        Ok(instruction)
    }

    /* TODO: doesn't handle errors. In the closest future I want to calculate stack and locals and make it optional.
       TODO: When it will be explicitly specified but don't match my calculations, it should warn.
    */
    fn parse_code_header(&mut self) -> (u16, u16) {
        let mut stack = None;
        let mut locals = None;

        while let Some(RnsToken::Identifier(_)) = self.peek_token() {
            let identifier_token = self.next_token();
            match identifier_token {
                RnsToken::Identifier(ref name) if name.value == "stack" => {
                    stack = Some(self.try_next_numeric().unwrap().value);
                }
                RnsToken::Identifier(ref name) if name.value == "locals" => {
                    locals = Some(self.try_next_numeric().unwrap().value);
                }
                other => self
                    .diagnostic
                    .push(ParserError::UnknownCodeDirectiveAttribute(other).into()),
            }
        }

        if !matches!(self.peek_token(), Some(RnsToken::Newline(_))) {
            panic!("Unexpected tokens after code directive header");
        }

        (stack.unwrap(), locals.unwrap())
    }

    fn parse_code_directive(&mut self) -> Option<CodeDirective> {
        let dir_span = self.next_token().span(); // consume .code token
        let mut labels: HashMap<String, u32> = HashMap::new();
        let mut instructions = Vec::new();
        let mut cur_pc = 0u32;

        let (stack, locals) = self.parse_code_header();

        while let Some(token) = self.peek_token() {
            match token {
                RnsToken::Newline(_) => {
                    self.next_token();
                }
                RnsToken::Label(_) => {
                    if let RnsToken::Label(label) = self.next_token() {
                        // TODO: check trailing tokens after label
                        labels.insert(label.value, cur_pc);
                        self.skip_newlines();
                        if matches!(self.peek_token(), Some(RnsToken::DotCodeEnd(_))) {
                            todo!("Special error for label should be followed by instruction")
                        }
                        let instruction = self.parse_instruction().unwrap();
                        cur_pc += instruction.spec.opcode.pc_size().unwrap() as u32;
                        instructions.push(instruction);
                    }
                }
                RnsToken::DotCodeEnd(_) | RnsToken::Eof(_) => {
                    self.next_token(); // consume .code_end or EOF
                    break;
                }
                _ => match self.parse_instruction() {
                    Ok(instruction) => {
                        cur_pc += instruction.spec.opcode.pc_size().unwrap() as u32;
                        instructions.push(instruction);
                    }
                    Err(e) => {
                        self.anchor(&[RnsTokenKind::Newline]);
                        self.diagnostic.push(e);
                    }
                },
            }
        }

        Some(CodeDirective {
            dir_span,
            instructions,
            labels,
            max_stack: stack,
            max_locals: locals,
        })
    }

    fn parse_method(&mut self) -> Result<MethodDirective, Diagnostic> {
        let dot_method = self.next_token(); // consume .method token
        let access_flags = self.parse_method_access_flags();
        let mut method = MethodDirective::new(dot_method.span(), access_flags);
        method.name = self
            .parse_operand_or_type_hint(OperandErrPosContext::MethodName, TypeHintKind::Utf8)
            .map_err(|e| self.diagnostic.push(e))
            .ok();
        // when there is no method name, it doesn't make sense to expect anything else
        if method.name.is_some() {
            method.descriptor = self
                .parse_operand_or_type_hint(
                    OperandErrPosContext::MethodDescriptor,
                    TypeHintKind::Utf8,
                )
                .map_err(|e| self.diagnostic.push(e))
                .ok();
        }
        // trailing tokens can be only when descriptor is present
        if method.descriptor.is_some() {
            self.verify_trailing_tokens(TrailingTokensErrContext::Method);
        }

        while let Some(token) = self.tokens.peek() {
            match token {
                RnsToken::Newline(_) => {
                    self.next_token();
                }
                RnsToken::DotCode(_) => {
                    if let Some(code_dir) = &method.code_dir {
                        self.diagnostic.push(
                            ParserError::MultipleCodeBlocks {
                                method_name: method.name.clone(),
                                method_span: method.dir_span,
                                first_code_span: code_dir.dir_span,
                                duplicate: token.span(),
                            }
                            .into(),
                        );
                        // TODO: decide final strategy
                        self.anchor(&[RnsTokenKind::DotCodeEnd]);
                    } else {
                        method.code_dir = self.parse_code_directive();
                    }
                }
                RnsToken::DotMethodEnd(_) => {
                    self.next_token(); // consume .method_end
                    break;
                }
                // TODO: decide strategy, allow not closed?
                RnsToken::Eof(_) => break,
                _ => {
                    let unexpected_error = ParserError::UnexpectedToken(
                        UnexpectedTokenContext::MethodBody,
                        self.next_token(),
                    );
                    self.diagnostic.push(unexpected_error.into());
                    // TODO: test unknown_token .super/.method etc.
                    self.anchor(&[RnsTokenKind::DotCode]);
                }
            }
        }

        Ok(MethodDirective {
            dir_span: method.dir_span,
            name: method.name,
            descriptor: method.descriptor,
            flags: method.flags,
            code_dir: method.code_dir,
        })
    }

    fn parse_super_directive(&mut self) {
        let super_token = self.next_token(); // consume .super token

        match self.parse_operand_or_type_hint(OperandErrPosContext::SuperName, TypeHintKind::Class)
        {
            Ok(super_name) => self.super_directives.push((super_token.span(), super_name)),
            Err(e) => {
                self.reported_errs.insert(ErrKind::Super);
                self.diagnostic.push(e);
            }
        }
        self.verify_trailing_tokens(TrailingTokensErrContext::Super)
    }

    fn anchor_class_directive(&mut self) -> Result<Span, Diagnostic> {
        self.skip_newlines();
        let next_token = self.next_token();

        if matches!(next_token, RnsToken::Eof(_)) {
            return Err(ParserError::EmptyFile(next_token.span()).into());
        }

        if matches!(next_token, RnsToken::DotClass(_)) {
            return Ok(next_token.span());
        }

        let error: Diagnostic =
            ParserError::UnexpectedToken(UnexpectedTokenContext::BeforeClassDefinition, next_token)
                .into();

        // first token is not `.class` — try to recover by finding the next `.class`
        if !self.anchor(&[RnsTokenKind::DotClass]) {
            // can't recover - fail here
            return Err(error);
        }

        // recovered - report error and continue parsing
        self.diagnostic.push(error);
        Ok(self.next_token().span())
    }

    fn parse_class(&mut self) -> Result<(), Diagnostic> {
        self.class_dir_span = self.anchor_class_directive()?;
        self.access_flags = self.parse_class_access_flags();

        self.class_name = self
            .parse_operand_or_type_hint(OperandErrPosContext::ClassName, TypeHintKind::Class)
            .map_err(|e| self.diagnostic.push(e))
            .ok();

        self.verify_trailing_tokens(TrailingTokensErrContext::Class);

        while let Some(token) = self.tokens.peek() {
            match token {
                RnsToken::Newline(_) => {
                    self.next_token();
                }
                RnsToken::DotMethod(_) => {
                    let method_dir = self.parse_method()?;
                    self.method_dirs.push(method_dir);
                }
                RnsToken::DotSuper(_) => self.parse_super_directive(),
                RnsToken::DotClassEnd(_) => {
                    self.next_token(); // consume .class_end
                    self.verify_trailing_tokens(TrailingTokensErrContext::ClassEnd);
                    break;
                }
                RnsToken::Eof(_) => break,
                _ => {
                    let unexpected_error = ParserError::UnexpectedToken(
                        UnexpectedTokenContext::ClassBody,
                        self.next_token(),
                    );
                    self.diagnostic.push(unexpected_error.into());
                    // TODO: test unknown_token .super/.method etc.
                    self.anchor(&[RnsTokenKind::DotMethod, RnsTokenKind::DotSuper]);
                }
            }
        }

        self.skip_newlines();
        let token_after_class = self.next_token();
        if !matches!(token_after_class, RnsToken::Eof(_)) {
            self.diagnostic.push(
                ParserError::UnexpectedToken(
                    UnexpectedTokenContext::AfterClassDefinition,
                    token_after_class,
                )
                .into(),
            );
        }

        Ok(())
    }

    fn anchor(&mut self, recovery_token: &[RnsTokenKind]) -> bool {
        while let Some(token) = self.tokens.peek() {
            if recovery_token.iter().any(|kind| token.matches_kind(*kind)) {
                return true;
            }
            self.next_token();
        }
        false
    }

    fn take_super_directive(&mut self) -> Option<SuperDirective> {
        let super_directives = std::mem::take(&mut self.super_directives);
        match super_directives.len() {
            0 => {
                // If no class name is defined, doesn't make sense to report missing superclass
                // If problem with super is already report, don't report missing superclass to avoid duplicate errors
                if self.class_name.is_some() && !self.reported_errs.contains(ErrKind::Super) {
                    self.diagnostic.push(
                        ParserWarning::MissingSuperClass {
                            class_name: self.class_name.clone(),
                            class_dir_pos: self.class_dir_span,
                            default: JAVA_LANG_OBJECT,
                        }
                        .into(),
                    );
                }
                Some(SuperDirective {
                    dir_span: None,
                    name: TypeHint::Class(
                        None,
                        Spanned::new(JAVA_LANG_OBJECT.to_string(), Span::default()),
                    ),
                })
            }
            1 => {
                let (dir_span, name) = super_directives.into_iter().next().unwrap();
                Some(SuperDirective {
                    dir_span: Some(dir_span),
                    name,
                })
            }
            _ => {
                self.diagnostic
                    .push(ParserError::MultipleSuperDefinitions(super_directives).into());
                None
            }
        }
    }
}

pub fn parse(tokens: Vec<RnsToken>, eof_span: Span) -> Result<RnsModule, Vec<Diagnostic>> {
    let mut instance = RnsParser {
        tokens: tokens.into_iter().peekable(),
        eof_span,
        last_span: Span::default(),
        last_kind: RnsTokenKind::Eof,
        diagnostic: Vec::new(),

        class_dir_span: Span::default(),
        class_name: None,
        method_dirs: Vec::new(),
        super_directives: Vec::new(),
        reported_errs: ReportedErrs::default(),
        access_flags: Default::default(),
    };

    if let Err(e) = instance.parse_class() {
        instance.diagnostic.push(e);
        return Err(instance.diagnostic);
    }
    let super_dir = instance.take_super_directive();
    let class_dir = ClassDirective {
        dir_span: instance.class_dir_span,
        name: instance.class_name.take(),
        flags: instance.access_flags,
    };
    Ok(RnsModule {
        class_dir,
        super_dir,
        diagnostics: instance.diagnostic,
        methods: instance.method_dirs,
    })
}
