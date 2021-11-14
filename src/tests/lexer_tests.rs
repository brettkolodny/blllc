use crate::lexer::Lexer;
use crate::token::TokenType::*;

#[test]
fn basic() {
  let expected = [LBRACE, LPAREN, RPAREN, RBRACE, EOF];
  let mut lexer = Lexer::new("{()}");

  for i in 0..expected.len() {
    assert_eq!(expected[i], lexer.next().token_type)
  }
}

#[test]
fn basic_comment() {
  let expected = [LBRACE, LPAREN, RPAREN, RBRACE, EOF];
  let input = ";; this is a comment\n{()}";
  let mut lexer = Lexer::new(&input);

  for i in 0..expected.len() {
    assert_eq!(expected[i], lexer.next().token_type)
  }
}

#[test]
fn basic_multi_char() {
  let expected = [
    LPAREN, LT, GT, SLT, SGT, RPAREN, EQ, NEQ, LBRACE, LTOE, GTOE, SLTOE, SGTOE, RBRACE, EOF,
  ];
  let input = "(< > S< S>) = != {<= >= S<= S>=}";
  let mut lexer = Lexer::new(&input);

  for i in 0..expected.len() {
    assert_eq!(expected[i], lexer.next().token_type)
  }
}

#[test]
fn basic_arith() {
  let expected = [ADD, SUB, MUL, DIV, MOD, BAND, BOR, BXOR, BNOT, EOF];
  let input = "+ - * / % & | ^ ~";
  let mut lexer = Lexer::new(&input);

  for i in 0..expected.len() {
    assert_eq!(expected[i], lexer.next().token_type)
  }
}

#[test]
fn basic_int() {
  let expected = [INT(42), INT(100), INT(0xaa)];
  let input = "42 100 0xaa";
  let mut lexer = Lexer::new(&input);

  for i in 0..expected.len() {
    assert_eq!(expected[i], lexer.next().token_type)
  }
}

#[test]
fn basic_string() {
  let expected = [
    STR(String::from("foo-bar")),
    STR(String::from("こんにちは世界")),
    STR(String::from("hello")),
    LPAREN,
    STR(String::from("hello world")),
    RPAREN,
    STR(String::from("bread")),
  ];
  let input = "'foo-bar 'こんにちは世界 'hello (\"hello world\") 'bread";
  let mut lexer = Lexer::new(&input);

  for i in 0..expected.len() {
    assert_eq!(expected[i], lexer.next().token_type)
  }
}
