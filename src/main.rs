mod lexer;
mod token;

#[cfg(test)]
mod tests;

use std::fs::read_to_string;
use std::path::Path;
use clap::{App, Arg};
use crate::lexer::Lexer;
use crate::token::TokenType;

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

        let mut lexer = Lexer::new(&file_str);
        let mut token = lexer.next();
        while token.token_type != TokenType::EOF {
            println!("{:?}", token);
            token = lexer.next();
        }
    }
}
