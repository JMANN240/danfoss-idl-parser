use pest_derive::Parser;

pub mod ast;
pub mod codegen;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct IDLParser;
