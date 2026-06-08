pub mod assembler;
pub mod ast;
pub mod diagnostic;
mod disassembler;
pub mod instruction;
pub mod lexer;
pub mod parser;
pub mod suggestion;
pub mod token;

pub use disassembler::{DisasmError, disassemble_bytes};

pub const ERROR_DOCS_BASE_URL: &str = "https://rune.lagertha-vm.com/errors/";
