use super::*;

/*
mod internal_error {
    use super::*;
    use crate::parser::ParserError;

    #[test]
    fn test_missing_eof() {
        let tokens = vec![JasmToken {
            kind: JasmTokenKind::Identifier("Test".to_string()),
            span: Span::new(0, 4),
        }];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert!(matches!(err, ParserError::Internal(_)));
    }

    #[test]
    fn test_eof_isnt_the_last_token() {
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(0, 0),
            },
            JasmToken {
                kind: JasmTokenKind::Identifier("Test".to_string()),
                span: Span::new(0, 4),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert!(matches!(err, ParserError::Internal(_)));
    }
}

mod class_directive_expected {
    use super::*;
    use crate::parser::ParserError;
    use rstest::rstest;

    #[rstest]
    #[case::dot_super(JasmTokenKind::DotSuper, Span::new(0, 6))]
    #[case::dot_method(JasmTokenKind::DotMethod, Span::new(0, 7))]
    #[case::dot_code(JasmTokenKind::DotCode, Span::new(0, 5))]
    #[case::dot_end(JasmTokenKind::DotEnd, Span::new(0, 4))]
    #[case::public(JasmTokenKind::Public, Span::new(0, 6))]
    #[case::static_kw(JasmTokenKind::Static, Span::new(0, 6))]
    #[case::identifier(JasmTokenKind::Identifier("HelloWorld".to_string()), Span::new(0, 10))]
    #[case::integer(JasmTokenKind::Integer(42), Span::new(0, 2))]
    #[case::string_literal(JasmTokenKind::StringLiteral("hello".to_string()), Span::new(0, 7))]
    fn test_non_class_token_returns_error(#[case] token_kind: JasmTokenKind, #[case] span: Span) {
        let tokens = vec![
            JasmToken {
                kind: token_kind.clone(),
                span: span.clone(),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(100, 100),
            },
        ];

        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(err, ParserError::ClassDirectiveExpected(span, token_kind));
    }

    #[test]
    fn test_skips_leading_newlines_before_error() {
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(0, 1),
            },
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(1, 2),
            },
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(2, 3),
            },
            JasmToken {
                kind: JasmTokenKind::DotSuper,
                span: Span::new(3, 9),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(9, 9),
            },
        ];

        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::ClassDirectiveExpected(Span::new(3, 9), JasmTokenKind::DotSuper)
        );
    }

    #[test]
    fn test_only_newlines_then_eof() {
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(0, 1),
            },
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(1, 2),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(2, 2),
            },
        ];

        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(err, ParserError::EmptyFile(Span::new(2, 2)));
    }

    #[test]
    fn test_eof_as_first_token() {
        let tokens = vec![JasmToken {
            kind: JasmTokenKind::Eof,
            span: Span::new(0, 0),
        }];

        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(err, ParserError::EmptyFile(Span::new(0, 0)));
    }

    #[test]
    fn test_single_newline_before_non_class_token() {
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(0, 1),
            },
            JasmToken {
                kind: JasmTokenKind::Public,
                span: Span::new(1, 7),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(7, 7),
            },
        ];

        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::ClassDirectiveExpected(Span::new(1, 7), JasmTokenKind::Public)
        );
    }

    #[test]
    fn test_error_span_preserves_offset() {
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::Identifier("main".to_string()),
                span: Span::new(42, 46),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(46, 46),
            },
        ];

        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::ClassDirectiveExpected(
                Span::new(42, 46),
                JasmTokenKind::Identifier("main".to_string())
            )
        );
    }
}

mod class_name_expected {
    use super::*;
    use crate::parser::ParserError;
    use rstest::rstest;

    /// Helper: builds a token stream of [.class, ...access_flags, name_token, Eof]
    fn make_tokens_with_flags_and_name(
        access_flags: &[JasmTokenKind],
        name_token: JasmToken,
    ) -> Vec<JasmToken> {
        // ".class" at 0..6
        let mut tokens = vec![JasmToken {
            kind: JasmTokenKind::DotClass,
            span: Span::new(0, 6),
        }];
        // access flags start after ".class " (pos 7)
        let mut pos = 7;
        for flag in access_flags {
            let len = match flag {
                JasmTokenKind::Public => 6,
                JasmTokenKind::Static => 6,
                _ => panic!("unexpected access flag"),
            };
            tokens.push(JasmToken {
                kind: flag.clone(),
                span: Span::new(pos, pos + len),
            });
            pos += len + 1; // +1 for space
        }
        tokens.push(name_token);
        tokens.push(JasmToken {
            kind: JasmTokenKind::Eof,
            span: Span::new(200, 200),
        });
        tokens
    }

    #[rstest]
    #[case::dot_class(JasmTokenKind::DotClass, Span::new(14, 20))]
    #[case::dot_super(JasmTokenKind::DotSuper, Span::new(14, 20))]
    #[case::dot_method(JasmTokenKind::DotMethod, Span::new(14, 21))]
    #[case::dot_code(JasmTokenKind::DotCode, Span::new(14, 19))]
    #[case::dot_end(JasmTokenKind::DotEnd, Span::new(14, 18))]
    #[case::integer(JasmTokenKind::Integer(42), Span::new(14, 16))]
    fn test_non_identifier_after_class_with_flags(
        #[case] token_kind: JasmTokenKind,
        #[case] span: Span,
    ) {
        let name_token = JasmToken {
            kind: token_kind.clone(),
            span,
        };
        // .class public <token>
        let tokens = make_tokens_with_flags_and_name(&[JasmTokenKind::Public], name_token);
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(span, token_kind, IdentifierContext::ClassDirective)
        );
    }

    #[test]
    fn test_eof_after_class_with_flags() {
        // .class public static<EOF>
        // ".class" 0..6, "public" 7..13, "static" 14..20
        // access_flag_end = 20 (last_span.end after consuming "static")
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Public,
                span: Span::new(7, 13),
            },
            JasmToken {
                kind: JasmTokenKind::Static,
                span: Span::new(14, 20),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(20, 20),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(Span::new(20, 20), JasmTokenKind::Eof)
        );
    }

    #[test]
    fn test_newline_after_class_with_flags() {
        // .class public\n
        // ".class" 0..6, "public" 7..13, "\n" 13..14
        // access_flag_end = 13 (last_span.end after consuming "public"; no static)
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Public,
                span: Span::new(7, 13),
            },
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(13, 14),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(14, 14),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(Span::new(13, 13), JasmTokenKind::Newline)
        );
    }

    #[test]
    fn test_eof_after_class_no_flags() {
        // .class<EOF>
        // ".class" 0..6
        // access_flag_end = 6 (last_span.end after consuming ".class", no flags consumed)
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(6, 6),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(Span::new(6, 6), JasmTokenKind::Eof)
        );
    }

    #[test]
    fn test_newline_after_class_no_flags() {
        // .class\n
        // ".class" 0..6, "\n" 6..7
        // access_flag_end = 6 (no flags consumed)
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(6, 7),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(7, 7),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(Span::new(6, 6), JasmTokenKind::Newline)
        );
    }

    #[test]
    fn test_directive_after_class_no_flags() {
        // .class .super
        // ".class" 0..6, ".super" 7..13
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::DotSuper,
                span: Span::new(7, 13),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(13, 13),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(Span::new(7, 13), JasmTokenKind::DotSuper)
        );
    }

    #[test]
    fn test_integer_after_class_no_flags() {
        // .class 42
        // ".class" 0..6, "42" 7..9
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Integer(42),
                span: Span::new(7, 9),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(9, 9),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(Span::new(7, 9), JasmTokenKind::Integer(42))
        );
    }
}

mod class_directive_trailing_tokens {
    use super::*;
    use crate::parser::ParserError;
    use rstest::rstest;

    /// Helper: builds [.class, ...access_flags, Identifier(class_name), ...trailing, Newline, Eof]
    fn make_tokens_with_trailing(
        access_flags: &[JasmTokenKind],
        class_name: &str,
        trailing: Vec<JasmToken>,
    ) -> Vec<JasmToken> {
        // ".class" at 0..6
        let mut tokens = vec![JasmToken {
            kind: JasmTokenKind::DotClass,
            span: Span::new(0, 6),
        }];
        let mut pos = 7;
        for flag in access_flags {
            let len = match flag {
                JasmTokenKind::Public => 6,
                JasmTokenKind::Static => 6,
                _ => panic!("unexpected access flag"),
            };
            tokens.push(JasmToken {
                kind: flag.clone(),
                span: Span::new(pos, pos + len),
            });
            pos += len + 1;
        }
        // class name identifier
        let name_len = class_name.len();
        tokens.push(JasmToken {
            kind: JasmTokenKind::Identifier(class_name.to_string()),
            span: Span::new(pos, pos + name_len),
        });
        pos += name_len + 1;
        // trailing tokens (use the spans from the provided tokens, but shift them)
        for t in &trailing {
            tokens.push(t.clone());
        }
        // newline + eof at the end
        tokens.push(JasmToken {
            kind: JasmTokenKind::Newline,
            span: Span::new(200, 201),
        });
        tokens.push(JasmToken {
            kind: JasmTokenKind::Eof,
            span: Span::new(201, 201),
        });
        tokens
    }

    #[rstest]
    #[case::dot_class(JasmTokenKind::DotClass, Span::new(24, 30))]
    #[case::dot_super(JasmTokenKind::DotSuper, Span::new(24, 30))]
    #[case::dot_method(JasmTokenKind::DotMethod, Span::new(24, 31))]
    #[case::dot_code(JasmTokenKind::DotCode, Span::new(24, 29))]
    #[case::dot_end(JasmTokenKind::DotEnd, Span::new(24, 28))]
    #[case::public(JasmTokenKind::Public, Span::new(24, 30))]
    #[case::static_kw(JasmTokenKind::Static, Span::new(24, 30))]
    #[case::identifier(JasmTokenKind::Identifier("extra".to_string()), Span::new(24, 29))]
    #[case::integer(JasmTokenKind::Integer(42), Span::new(24, 26))]
    #[case::string_literal(JasmTokenKind::StringLiteral("hello".to_string()), Span::new(24, 31))]
    #[case::open_paren(JasmTokenKind::OpenParen, Span::new(24, 25))]
    #[case::close_paren(JasmTokenKind::CloseParen, Span::new(24, 25))]
    #[case::open_bracket(JasmTokenKind::OpenBracket, Span::new(24, 25))]
    fn test_single_trailing_token(#[case] token_kind: JasmTokenKind, #[case] span: Span) {
        // .class public HelloWorld <trailing_token>\n<EOF>
        let trailing_token = JasmToken {
            kind: token_kind,
            span,
        };
        let trailing = vec![trailing_token.clone()];
        let tokens =
            make_tokens_with_trailing(&[JasmTokenKind::Public], "HelloWorld", trailing.clone());
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::TrailingTokens(trailing, "HelloWorld".to_string())
        );
    }

    #[test]
    fn test_trailing_without_access_flags() {
        // .class HelloWorld .super
        // ".class" 0..6, "HelloWorld" 7..17, ".super" 18..24
        let trailing = vec![JasmToken {
            kind: JasmTokenKind::DotSuper,
            span: Span::new(18, 24),
        }];
        let tokens = make_tokens_with_trailing(&[], "HelloWorld", trailing.clone());
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::TrailingTokens(trailing, "HelloWorld".to_string())
        );
    }

    #[test]
    fn test_multiple_trailing_tokens() {
        // .class public HelloWorld extra tokens here\n<EOF>
        let trailing = vec![
            JasmToken {
                kind: JasmTokenKind::Identifier("extra".to_string()),
                span: Span::new(24, 29),
            },
            JasmToken {
                kind: JasmTokenKind::Identifier("tokens".to_string()),
                span: Span::new(30, 36),
            },
            JasmToken {
                kind: JasmTokenKind::Identifier("here".to_string()),
                span: Span::new(37, 41),
            },
        ];
        let tokens =
            make_tokens_with_trailing(&[JasmTokenKind::Public], "HelloWorld", trailing.clone());
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::TrailingTokens(trailing, "HelloWorld".to_string())
        );
    }

    #[test]
    fn test_trailing_span_covers_all_tokens() {
        // Span should go from first trailing token start to last trailing token end
        let trailing = vec![
            JasmToken {
                kind: JasmTokenKind::Identifier("extra".to_string()),
                span: Span::new(24, 29),
            },
            JasmToken {
                kind: JasmTokenKind::Integer(42),
                span: Span::new(30, 32),
            },
        ];
        let err = ParserError::TrailingTokens(trailing, "Test".to_string());
        assert_eq!(err.as_range(), Some(24..32));
    }

    #[test]
    fn test_trailing_single_token_span() {
        let trailing = vec![JasmToken {
            kind: JasmTokenKind::DotSuper,
            span: Span::new(18, 24),
        }];
        let err = ParserError::TrailingTokens(trailing, "Test".to_string());
        assert_eq!(err.as_range(), Some(18..24));
    }

    #[test]
    fn test_no_trailing_tokens_is_ok() {
        // .class public HelloWorld\n<EOF>
        // No trailing tokens → no ClassDirectiveTrailingTokens error
        // (parser continues past trailing check; may panic on todo!() further down)
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Public,
                span: Span::new(7, 13),
            },
            JasmToken {
                kind: JasmTokenKind::Identifier("HelloWorld".to_string()),
                span: Span::new(14, 24),
            },
            JasmToken {
                kind: JasmTokenKind::Newline,
                span: Span::new(24, 25),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(25, 25),
            },
        ];
        let result = std::panic::catch_unwind(|| JasmParser::parse(tokens));
        match result {
            Ok(Err(ParserError::TrailingTokens(..))) => {
                panic!("Should not produce ClassDirectiveTrailingTokens when no trailing tokens")
            }
            _ => {}
        }
    }

    #[test]
    fn test_eof_immediately_after_class_name_is_ok() {
        // .class public HelloWorld<EOF>
        // No newline before EOF → no trailing tokens
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Public,
                span: Span::new(7, 13),
            },
            JasmToken {
                kind: JasmTokenKind::Identifier("HelloWorld".to_string()),
                span: Span::new(14, 24),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(24, 24),
            },
        ];
        let result = std::panic::catch_unwind(|| JasmParser::parse(tokens));
        match result {
            Ok(Err(ParserError::TrailingTokens(..))) => {
                panic!(
                    "Should not produce ClassDirectiveTrailingTokens when EOF follows class name"
                )
            }
            _ => {}
        }
    }

    #[test]
    fn test_trailing_with_multiple_flags() {
        // .class public static HelloWorld .super\n<EOF>
        // ".class" 0..6, "public" 7..13, "static" 14..20, "HelloWorld" 21..31, ".super" 32..38
        let trailing = vec![JasmToken {
            kind: JasmTokenKind::DotSuper,
            span: Span::new(32, 38),
        }];
        let tokens = make_tokens_with_trailing(
            &[JasmTokenKind::Public, JasmTokenKind::Static],
            "HelloWorld",
            trailing.clone(),
        );
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::TrailingTokens(trailing, "HelloWorld".to_string())
        );
    }

    #[test]
    fn test_class_name_preserved_in_error() {
        // Verify the class name string is correctly captured
        let trailing = vec![JasmToken {
            kind: JasmTokenKind::Integer(99),
            span: Span::new(30, 32),
        }];
        let tokens =
            make_tokens_with_trailing(&[JasmTokenKind::Public], "com/myapp/Main", trailing.clone());
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::TrailingTokens(trailing, "com/myapp/Main".to_string())
        );
    }
}

mod string_literal_as_class_name {
    use super::*;
    use crate::parser::ParserError;

    #[test]
    fn test_string_literal_after_class_with_flags() {
        // .class public "HelloWorld"
        // ".class" 0..6, "public" 7..13, "\"HelloWorld\"" 14..26
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Public,
                span: Span::new(7, 13),
            },
            JasmToken {
                kind: JasmTokenKind::StringLiteral("HelloWorld".to_string()),
                span: Span::new(14, 26),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(26, 26),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(
                Span::new(14, 26),
                JasmTokenKind::StringLiteral("HelloWorld".to_string())
            )
        );
    }

    #[test]
    fn test_string_literal_after_class_no_flags() {
        // .class "HelloWorld"
        // ".class" 0..6, "\"HelloWorld\"" 7..19
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::StringLiteral("HelloWorld".to_string()),
                span: Span::new(7, 19),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(19, 19),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(
                Span::new(7, 19),
                JasmTokenKind::StringLiteral("HelloWorld".to_string())
            )
        );
    }

    #[test]
    fn test_string_literal_after_class_with_multiple_flags() {
        // .class public static "HelloWorld"
        // ".class" 0..6, "public" 7..13, "static" 14..20, "\"HelloWorld\"" 21..33
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::Public,
                span: Span::new(7, 13),
            },
            JasmToken {
                kind: JasmTokenKind::Static,
                span: Span::new(14, 20),
            },
            JasmToken {
                kind: JasmTokenKind::StringLiteral("HelloWorld".to_string()),
                span: Span::new(21, 33),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(33, 33),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(
                Span::new(21, 33),
                JasmTokenKind::StringLiteral("HelloWorld".to_string())
            )
        );
    }

    #[test]
    fn test_string_literal_span_preserves_offset() {
        // String literal at an arbitrary offset
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(50, 56),
            },
            JasmToken {
                kind: JasmTokenKind::StringLiteral("com/myapp/Main".to_string()),
                span: Span::new(57, 73),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(73, 73),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(
                Span::new(57, 73),
                JasmTokenKind::StringLiteral("com/myapp/Main".to_string())
            )
        );
    }

    #[test]
    fn test_empty_string_literal_as_class_name() {
        // .class ""
        // ".class" 0..6, "\"\"" 7..9
        let tokens = vec![
            JasmToken {
                kind: JasmTokenKind::DotClass,
                span: Span::new(0, 6),
            },
            JasmToken {
                kind: JasmTokenKind::StringLiteral("".to_string()),
                span: Span::new(7, 9),
            },
            JasmToken {
                kind: JasmTokenKind::Eof,
                span: Span::new(9, 9),
            },
        ];
        let err = JasmParser::parse(tokens).unwrap_err();
        assert_eq!(
            err,
            ParserError::IdentifierExpected(
                Span::new(7, 9),
                JasmTokenKind::StringLiteral("".to_string())
            )
        );
    }
}

mod parser_error_messages {
    use super::*;
    use crate::parser::ParserError;

    #[test]
    fn test_empty_file_span() {
        let err = ParserError::EmptyFile(Span::new(5, 5));
        assert_eq!(err.as_range(), Some(5..5));
    }

    #[test]
    fn test_span_returns_correct_span() {
        let err = ParserError::ClassDirectiveExpected(Span::new(10, 15), JasmTokenKind::DotMethod);
        assert_eq!(err.as_range(), Some(10..15));
    }

    #[test]
    fn test_internal_error_has_no_range() {
        let err = ParserError::Internal("bug".to_string());
        assert_eq!(err.as_range(), None);
    }

    #[test]
    fn test_class_name_expected_span() {
        let err = ParserError::IdentifierExpected(Span::new(14, 20), JasmTokenKind::Integer(42));
        assert_eq!(err.as_range(), Some(14..20));
    }

    #[test]
    fn test_class_name_expected_zero_width_span() {
        let err = ParserError::IdentifierExpected(Span::new(6, 6), JasmTokenKind::Eof);
        assert_eq!(err.as_range(), Some(6..6));
    }

    #[test]
    fn test_string_literal_as_class_name_span() {
        let err = ParserError::IdentifierExpected(
            Span::new(14, 21),
            JasmTokenKind::StringLiteral("Hello".to_string()),
        );
        assert_eq!(err.as_range(), Some(14..21));
    }

    #[test]
    fn test_trailing_tokens_span() {
        let trailing = vec![
            JasmToken {
                kind: JasmTokenKind::Identifier("extra".to_string()),
                span: Span::new(24, 29),
            },
            JasmToken {
                kind: JasmTokenKind::Integer(42),
                span: Span::new(30, 32),
            },
        ];
        let err = ParserError::TrailingTokens(trailing, "HelloWorld".to_string());
        assert_eq!(err.as_range(), Some(24..32));
    }
}

 */
