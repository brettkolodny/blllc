use crate::lexer::Lexer;
use crate::token::{TokenType, TokenType::*};

fn test(expected: Vec<TokenType>, input: &str) {
  let mut lexer = Lexer::new(input);

  for i in 0..expected.len() {
    assert_eq!(expected[i], lexer.next().token_type)
  }
}

#[test]
fn basic() {
  let expected = vec![LBRACE, LPAREN, RPAREN, RBRACE, EOF];
  let input = "{()}";
  test(expected, input);
}

#[test]
fn basic_comment() {
  let expected = vec![LBRACE, LPAREN, RPAREN, RBRACE, EOF];
  let input = ";; this is a comment\n{()}";
  test(expected, input);
}

#[test]
fn basic_multi_char() {
  let expected = vec![
    LPAREN, LT, GT, SLT, SGT, RPAREN, EQ, NEQ, LBRACE, LTOE, GTOE, SLTOE, SGTOE, RBRACE, EOF,
  ];
  let input = "(< > S< S>) = != {<= >= S<= S>=}";
  test(expected, input);
}

#[test]
fn basic_arith() {
  let expected = vec![ADD, SUB, MUL, DIV, MOD, BAND, BOR, BXOR, BNOT, EOF];
  let input = "+ - * / % & | ^ ~";
  test(expected, input);
}

#[test]
fn basic_int() {
  let expected = vec![INT(42), INT(100), INT(0xaa)];
  let input = "42 100 0xaa";
  test(expected, input);
}

#[test]
fn basic_string() {
  let expected = vec![
    STR(String::from("foo-bar")),
    STR(String::from("こんにちは世界")),
    STR(String::from("hello")),
    LPAREN,
    STR(String::from("hello world")),
    RPAREN,
    STR(String::from("bread")),
  ];
  let input = "'foo-bar 'こんにちは世界 'hello (\"hello world\") 'bread";
  test(expected, input);
}

#[test]
fn basic_identitiy() {
  let expected = vec![IDENT(String::from("foo")), IDENT(String::from("foo-bar"))];
  let input = "foo foo-bar";
  test(expected, input);
}

#[test]
fn basic_contract() {
  let epxected = vec![
    LPAREN,
    IDENT(String::from("seq")),
    LPAREN,
    DEF,
    STR(String::from("scratch")),
    INT(0x00),
    RPAREN,
    LPAREN,
    DEF,
    STR(String::from("identity")),
    INT(0xac37eebb),
    RPAREN,
    LPAREN,
    DEF,
    STR(String::from("function")),
    LPAREN,
    IDENT(String::from("function-hash")),
    IDENT(String::from("code-body")),
    RPAREN,
    LPAREN,
    IDENT(String::from("when")),
    LPAREN,
    EQ,
    LPAREN,
    IDENT(String::from("div")),
    LPAREN,
    IDENT(String::from("calldataload")),
    INT(0x00),
    RPAREN,
    LPAREN,
    IDENT(String::from("exp")),
    INT(2),
    INT(224),
    RPAREN,
    RPAREN,
    IDENT(String::from("function-hash")),
    RPAREN,
    IDENT(String::from("code-body")),
    RPAREN,
    RPAREN,
    LPAREN,
    IDENT(String::from("returnlll")),
    LPAREN,
    IDENT(String::from("function")),
    IDENT(String::from("identity")),
    LPAREN,
    IDENT(String::from("seq")),
    LPAREN,
    IDENT(String::from("mstore")),
    IDENT(String::from("scratch")),
    LPAREN,
    IDENT(String::from("calldataload")),
    INT(0x04),
    RPAREN,
    RPAREN,
    LPAREN,
    IDENT(String::from("return")),
    IDENT(String::from("scratch")),
    INT(32),
    RPAREN,
    RPAREN,
    RPAREN,
    RPAREN,
    RPAREN,
    EOF,
  ];
  let input = "(seq
    (def 'scratch 0x00)
    (def 'identity 0xac37eebb)
    (def 'function (function-hash code-body)
      (when (= (div (calldataload 0x00) (exp 2 224)) function-hash)
        code-body))
    (returnlll
      (function identity
        (seq
          (mstore scratch (calldataload 0x04))
          (return scratch 32)))))";

  test(epxected, input);
}
