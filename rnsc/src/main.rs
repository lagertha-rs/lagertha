use clap::{Parser, Subcommand};
use rns::diagnostic::DiagnosticTier;
use rns::lexer;
use rns::parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rnsc", about = "Java assembler and disassembler")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Asm {
        file: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Dis {
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Asm { file, output }) => assemble(&file, output.as_ref()),
        Some(Command::Dis { file }) => disassemble(&file),
        None => {
            if let Some(file) = cli.file {
                assemble(&file, None);
            } else {
                eprintln!("Usage: rnsc <file.rns> or rnsc asm <file.rns> or rnsc dis <file.class>");
                std::process::exit(1);
            }
        }
    }
}

fn assemble(path: &PathBuf, output: Option<&PathBuf>) {
    let filename = path.to_string_lossy().to_string();
    let contents = std::fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        std::process::exit(1);
    });

    let (tokens, eof_span) = {
        let (tokens, diagnostics, eof_span) = lexer::tokenize(&contents);
        let mut has_error = false;
        for diag in diagnostics {
            if diag.tier == DiagnosticTier::SyntaxError {
                has_error = true
            }
            diag.print(&filename, &contents);
        }
        if has_error {
            std::process::exit(1);
        }
        (tokens, eof_span)
    };

    let rns_module = {
        let mut module = match parser::parse(tokens, eof_span) {
            Ok(module) => module,
            Err(errors) => {
                for err in errors {
                    err.print(&filename, &contents);
                }
                std::process::exit(1);
            }
        };

        let mut has_error = false;
        let diagnostics = std::mem::take(&mut module.diagnostics);
        for diag in diagnostics {
            if diag.tier == DiagnosticTier::SyntaxError {
                has_error = true;
            }
            diag.print(&filename, &contents);
        }
        if has_error {
            std::process::exit(1);
        }

        module
    };

    let (class, diagnostics) = rns_module.into_class_file();

    let mut has_error = false;
    for diag in diagnostics {
        if diag.tier == DiagnosticTier::SyntaxError {
            has_error = true;
        }
        diag.print(&filename, &contents);
    }

    if has_error {
        std::process::exit(1);
    }

    if let Some(class) = class {
        let bytes = class.to_bytes();
        let output_path = output
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| filename.replace(".rns", ".class"));
        std::fs::write(output_path, bytes).expect("Failed to write output file");
    } else {
        std::process::exit(1);
    }
}

fn disassemble(path: &PathBuf) {
    let bytes = std::fs::read(path).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", path.display(), err);
        std::process::exit(1);
    });

    let class_file = jclass::ClassFile::try_from(bytes).unwrap_or_else(|err| {
        eprintln!("Error parsing class file {}: {}", path.display(), err);
        std::process::exit(1);
    });

    let rns_text = rns::disassemble(&class_file).unwrap_or_else(|err| {
        eprintln!("Error disassembling class file {}: {}", path.display(), err);
        std::process::exit(1);
    });
    print!("{}", rns_text);
}
