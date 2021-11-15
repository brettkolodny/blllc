use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
  // TYPES
  INT(u32),
  STR(String),
  IDENT(String),
  // SYMBOLS
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  LBRACKET,
  RBRACKET,
  ADD,
  EOF,
  SUB,
  MUL,
  DIV,
  MOD,
  BAND,
  BOR,
  BXOR,
  LT,
  GT,
  LTOE,
  GTOE,
  EQ,
  NEQ,
  SLT,
  SGT,
  SLTOE,
  SGTOE,
  BNOT,
  AT,
  DAT,
  // KEY WORDS
  DEF,
  INVALID,
}

#[derive(Clone, Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub row: u32,
  pub col: u32,
}

impl Token {
  pub fn new(token_type: TokenType, row: u32, col: u32) -> Self {
    Token {
      token_type,
      row,
      col,
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.token_type)
  }
}
