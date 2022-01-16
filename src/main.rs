mod token;
mod lexer;
mod ast;
mod parser;
mod compiler;

#[cfg(test)]
mod tests;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use clap::{App, Arg};
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let matches = App::new("Brett's Lovely Little Language Compiler")
        .version("0.1.0")
        .author("Brett Kolodny <brettkolodny@gmail.com>")
        .arg(
            Arg::with_name("input")
                .help("Input file")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .get_matches();

    if let Some(input) = matches.value_of("input") {
        let path = Path::new(input);
        let file_str = read_to_string(path).expect(&format!("Could not open file at {}", &input));

        let lexer = Lexer::new(&file_str);
        let mut parser = Parser::new(lexer);

        if let Ok(ast) = parser.parse_program() {
            let compiler = Compiler::new(ast);
            let byte_code = compiler.compile().expect("Compilation error");
            
            println!("{}", byte_code);
        } else {
            std::process::exit(1);
        }
    } else {
        std::process::exit(1);
    }
}
