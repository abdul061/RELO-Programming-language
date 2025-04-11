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

use crate::{
    compiler::Compiler,
    interpreter::{Interpreter, Visitable},
    lexer::{Lexer, TokenKind},
    parser::Parser,
    vm::Vm,
};

use color_eyre::eyre::Result;

const USE_COMPILER: bool = true;

fn main() -> Result<()> {
    color_eyre::install()?;
    let program = r#"
START

    VARIABLE i IS 1000000000;
    LOOP 1 Start i End{

    }DONE;
    DISPLAY i;
END

    "#;

    let tokens = Lexer::tokenize_str(program)?;
    let token_kinds = tokens
        .into_iter()
        .map(|tok| tok.kind)
        .collect::<Vec<TokenKind>>();
    let parser = Parser::new(&token_kinds);
    let ast = parser.parse().unwrap();
    if USE_COMPILER {
        // Bytecode interpreter
        let compiler = Compiler::new();
        let bytecode_program = compiler.compile_program(&ast);
        let mut vm = Vm::new();
        vm.interpret(&bytecode_program);
    } else {
        // Tree-walk interpreter
        let mut interpreter = Interpreter::new();
        ast.visit(&mut interpreter).unwrap();
    }
    Ok(())
}
