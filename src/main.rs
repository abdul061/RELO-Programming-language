#![feature(let_chains)]
#![feature(variant_count)]
#![feature(inline_const_pat)]
#![feature(internal_output_capture)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(incomplete_features)]
#![allow(clippy::enum_variant_names)]

// Declare all your internal modules
mod compiler;
mod interpreter;
mod lexer;
mod parser;
mod vm;

use std::fs;
use std::path::Path;
use color_eyre::eyre::*;


use crate::{
    compiler::Compiler,
    interpreter::{Interpreter, Visitable},
    lexer::{Lexer, TokenKind},
    parser::Parser,
    vm::Vm,
};



const USE_COMPILER: bool = true;

fn main() -> Result<()> {
    // Enables better error traces
    color_eyre::install()?; 

    // ✅ Path to your .relo file (make sure this exists)
    let file_path = Path::new("testdata/porogram.relo");

    // ✅ Read source code from file
    let program = fs::read_to_string(file_path)
    .map_err(|e| eyre!("Failed to read {}: {}", file_path.display(), e))?;

    // ✅ Tokenize the source
    let tokens = Lexer::tokenize_str(&program)?;

    // ✅ Convert tokens to kinds for parsing
    let token_kinds = tokens
        .into_iter()
        .map(|tok| tok.kind)
        .collect::<Vec<TokenKind>>();

    // ✅ Parse to AST
    let parser = Parser::new(&token_kinds);
    let ast = parser.parse().unwrap();

    // ✅ Either compile and run, or interpret
    if USE_COMPILER {
        let compiler = Compiler::new();
        let bytecode_program = compiler.compile_program(&ast);
        let mut vm = Vm::new();
        vm.interpret(&bytecode_program);
    } else {
        let mut interpreter = Interpreter::new();
        ast.visit(&mut interpreter).unwrap();
    }

    Ok(())
}
