use super::*;
use insta::with_settings;
use rstest::rstest;
use std::io::Write;
use std::path::{Path, PathBuf};
use tabwriter::TabWriter;

const SNAPSHOT_PATH: &str = "snapshots";

fn format_tokens(tokens: &[RnsToken], source: &str) -> String {
    let mut tw = TabWriter::new(vec![]);

    // Header
    writeln!(tw, "KIND\t| SPAN\t| LSP\t| TEXT").unwrap();
    writeln!(tw, "----\t| ----\t| ---\t| ----").unwrap();

    for token in tokens {
        let kind_str = match &token {
            RnsToken::Identifier(s) => format!("Identifier({:?})", s.value),
            RnsToken::Label(s) => format!("Label({:?})", s.value),
            RnsToken::DotCode(_) => "DotCode".to_string(),
            RnsToken::DotCodeEnd(_) => "DotCodeEnd".to_string(),
            RnsToken::DotClass(_) => "DotClass".to_string(),
            RnsToken::DotClassEnd(_) => "DotClassEnd".to_string(),
            RnsToken::DotMethod(_) => "DotMethod".to_string(),
            RnsToken::DotMethodEnd(_) => "DotMethodEnd".to_string(),
            RnsToken::DotSuper(_) => "DotSuper".to_string(),
            RnsToken::Newline(_) => "Newline".to_string(),
            RnsToken::Eof(_) => unreachable!("Eof token should not be in the token list"),
            RnsToken::AccessFlag(spanned) => format!("AccessFlag({})", spanned.value.token_name()),
            RnsToken::DotAnnotation(_) => "DotAnnotation".to_string(),
            RnsToken::TypeHint(spanned) => format!("TypeHint({})", spanned.value.token_name()),
        };

        let span = token.span();
        let span_str = format!("{}..{}", span.byte_start, span.byte_end);
        let lsp_str = format!("{}:{}..{}", span.line, span.col_start, span.col_end);
        let text = &source[span.byte_start..span.byte_end];
        // Escape newlines and other control characters for display
        let text_display = text
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");

        writeln!(
            tw,
            "{}\t| {}\t| {}\t| {}",
            kind_str, span_str, lsp_str, text_display
        )
        .unwrap();
    }

    tw.flush().unwrap();
    let tokens_table = String::from_utf8(tw.into_inner().unwrap()).unwrap();

    // Combine source and tokens table
    format!(
        "----- SOURCE -----\n{}\n----- TOKENS -----\n{}",
        source.trim_end(),
        tokens_table.trim_end()
    )
}

fn to_snapshot_name(path: &Path) -> String {
    let marker = Path::new("test_data/unit/lexer");
    let components = path.components().collect::<Vec<_>>();

    let marker_parts = marker.components().collect::<Vec<_>>();
    let idx = components
        .windows(marker_parts.len())
        .position(|window| window == marker_parts)
        .expect("Marker path not found in the given path");

    let after = &components[idx + marker_parts.len()..];
    let mut new_path = PathBuf::new();
    for c in after {
        new_path.push(c);
    }

    new_path.set_extension("");

    new_path
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("-")
}

#[rstest]
fn success_cases(
    #[base_dir = "test_data/unit/lexer/"]
    #[files("**/*.rns")]
    path: PathBuf,
) {
    let source = std::fs::read_to_string(&path).expect("Unable to read file");
    let (tokens, diagnostics, _eof_span) = super::tokenize(&source);
    assert!(
        diagnostics.is_empty(),
        "Lexer should succeed for success test cases, but got diagnostics: {:?}",
        diagnostics
    );

    // Cross-validate: text extracted via byte offsets must match text extracted via line/col
    let lines: Vec<&str> = source.split('\n').collect();
    for token in &tokens {
        let span = token.span();
        let byte_text = &source[span.byte_start..span.byte_end];

        // For tokens that don't span multiple lines, verify line/col points to the same text
        if !byte_text.contains('\n') && span.byte_start != span.byte_end {
            assert!(
                span.line < lines.len(),
                "Token {:?} has line {} but source only has {} lines",
                token,
                span.line,
                lines.len()
            );
            let line_content = lines[span.line];
            let line_chars: Vec<char> = line_content.chars().collect();
            assert!(
                span.col_start <= span.col_end && span.col_end <= line_chars.len(),
                "Token {:?} has col_start={}, col_end={} but line {} has {} chars: {:?}",
                token,
                span.col_start,
                span.col_end,
                span.line,
                line_chars.len(),
                line_content
            );
            let col_text: String = line_chars[span.col_start..span.col_end].iter().collect();
            assert_eq!(
                byte_text,
                col_text,
                "Byte span {}..{} gives {:?} but line {}:col {}..{} gives {:?}",
                span.byte_start,
                span.byte_end,
                byte_text,
                span.line,
                span.col_start,
                span.col_end,
                col_text
            );
        }
    }

    let formatted = format_tokens(&tokens, &source);

    with_settings!(
        {
            snapshot_path => SNAPSHOT_PATH,
            prepend_module_to_snapshot => false,
        },
        {
            insta::assert_snapshot!(to_snapshot_name(&path), formatted);
        }
    );
}
