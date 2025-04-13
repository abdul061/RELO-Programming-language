#![feature(let_chains)]
#![feature(variant_count)]
#![feature(inline_const_pat)]
#![feature(internal_output_capture)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(incomplete_features)]
#![allow(clippy::enum_variant_names)]

mod compiler;
mod interpreter;
mod lexer;
mod parser;
mod vm;

use std::env;
use std::fs;
use std::path::PathBuf;
use color_eyre::eyre::{eyre, Result};

use crate::{
    compiler::Compiler,
    interpreter::{Interpreter, Visitable},
    lexer::{Lexer, TokenKind},
    parser::Parser,
    vm::Vm,
};

const USE_COMPILER: bool = true;

fn main() -> Result<()> {
    color_eyre::install()?; 

    // Get the base filename (without extension)
    let args: Vec<String> = env::args().collect();
    let file_stem = args.get(1).ok_or_else(|| eyre!("Please provide a .relo file name (without extension)"))?;

    // Add the `.relo` extension automatically
    let file_path = PathBuf::from(format!("{file_stem}.relo"));

    if !file_path.exists() {
        return Err(eyre!("File {} does not exist", file_path.display()));
    }

    println!("🦀 Running file: {}\n", file_path.display());

    let program = fs::read_to_string(&file_path)
        .map_err(|e| eyre!("Failed to read {}: {}", file_path.display(), e))?;

    let tokens = Lexer::tokenize_str(&program)?;
    let token_kinds = tokens.into_iter().map(|tok| tok.kind).collect::<Vec<TokenKind>>();

    let parser = Parser::new(&token_kinds);
    let ast = parser.parse().unwrap();

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
