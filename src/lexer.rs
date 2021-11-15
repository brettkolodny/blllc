use crate::token::{Token, TokenType};
use regex::Regex;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
  position: Peekable<Chars<'a>>,
  row: u32,
  col: u32,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Lexer {
      position: input.chars().peekable(),
      row: 1,
      col: 0,
    }
  }

  pub fn next(&mut self) -> Token {
    let character = self.position.next();
    self.col += 1;

    if let Some(c) = character {
      match c {
        '(' => Token::new(TokenType::LPAREN, self.row, self.col),
        ')' => Token::new(TokenType::RPAREN, self.row, self.col),
        '{' => Token::new(TokenType::LBRACE, self.row, self.col),
        '}' => Token::new(TokenType::RBRACE, self.row, self.col),
        '[' => Token::new(TokenType::LBRACKET, self.row, self.col),
        ']' => Token::new(TokenType::RBRACKET, self.row, self.col),
        '+' => Token::new(TokenType::ADD, self.row, self.col),
        '-' => Token::new(TokenType::SUB, self.row, self.col),
        '*' => Token::new(TokenType::MUL, self.row, self.col),
        '/' => Token::new(TokenType::DIV, self.row, self.col),
        '%' => Token::new(TokenType::MOD, self.row, self.col),
        '&' => Token::new(TokenType::BAND, self.row, self.col),
        '|' => Token::new(TokenType::BOR, self.row, self.col),
        '^' => Token::new(TokenType::BXOR, self.row, self.col),
        '~' => Token::new(TokenType::BNOT, self.row, self.col),
        '=' => Token::new(TokenType::EQ, self.row, self.col),
        ';' => {
          self.next_line();
          self.next()
        }
        '\'' | '"' => self.lex_string(c),
        '>' | '<' | 'S' | '!' | '@' => self.lex_multi_char(c),
        ' ' => self.next(),
        '\n' => {
          self.row += 1;
          self.col = 0;
          self.next()
        }
        c if c.is_ascii_digit() => self.lex_int(c),
        c if c.is_ascii_alphabetic() => {
          let word = self.read_word(c);
          match word.as_str() {
            "def" => Token::new(TokenType::DEF, self.row, self.col),
            _ => Token::new(TokenType::IDENT(word), self.row, self.col)
          }
        }
        _ => Token::new(TokenType::EOF, self.row, self.col),
      }
    } else {
      Token::new(TokenType::EOF, self.row, self.col)
    }
  }

  fn lex_string(&mut self, curr_char: char) -> Token {
    if curr_char == '\'' {
      let mut string = self.read_word(curr_char);
      let single_quote_pattern = Regex::new(r"^'(\w|-)+$").unwrap();

      if single_quote_pattern.is_match(&string) {
        string = String::from(string.trim_start_matches("'"));
        return Token::new(TokenType::STR(string), self.row, self.col);
      } else {
        return Token::new(TokenType::INVALID, self.row, self.col);
      }
    } else {
      let string = String::from(
        self
          .read_double_quote_string(curr_char)
          .trim_start_matches("\"")
          .trim_end_matches("\""),
      );

      return Token::new(TokenType::STR(string), self.row, self.col);
    }
  }

  fn lex_int(&mut self, curr_char: char) -> Token {
    let word = self.read_word(curr_char);

    let int = word.parse::<u32>();

    if let Ok(i) = int {
      Token::new(TokenType::INT(i), self.row, self.col)
    } else {
      if word.starts_with("0x") {
        let stripped_string = word.trim_start_matches("0x");
        let hex_int = u32::from_str_radix(stripped_string, 16);

        if let Ok(i) = hex_int {
          Token::new(TokenType::INT(i), self.row, self.col)
        } else {
          Token::new(TokenType::INVALID, self.row, self.col)
        }
      } else {
        Token::new(TokenType::INVALID, self.row, self.col)
      }
    }
  }

  fn lex_multi_char(&mut self, curr_char: char) -> Token {
    let word = self.read_word(curr_char);

    println!("{}", &word);
    match word.as_str() {
      ">=" => Token::new(TokenType::GTOE, self.row, self.col),
      "<=" => Token::new(TokenType::LTOE, self.row, self.col),
      "<" => Token::new(TokenType::LT, self.row, self.col),
      ">" => Token::new(TokenType::GT, self.row, self.col),
      "S>" => Token::new(TokenType::SGT, self.row, self.col),
      "S<" => Token::new(TokenType::SLT, self.row, self.col),
      "S>=" => Token::new(TokenType::SGTOE, self.row, self.col),
      "S<=" => Token::new(TokenType::SLTOE, self.row, self.col),
      "!=" => Token::new(TokenType::NEQ, self.row, self.col),
      "@" => Token::new(TokenType::AT, self.row, self.col),
      "@@" => Token::new(TokenType::DAT, self.row, self.col),
      _ => Token::new(TokenType::EOF, self.row, self.col),
    }
  }

  fn read_double_quote_string(&mut self, curr_char: char) -> String {
    let mut character = self.position.peek();
    let mut word = String::new();
    word.push(curr_char);

    while character.is_some() && character != Some(&'"') {
      word.push(*character.unwrap());
      if character == Some(&' ') {
        self.row += 1;
        self.col = 0;
      } else {
        self.col += 1;
      }

      self.position.next();
      character = self.position.peek();
    }

    if character == Some(&'"') {
      self.position.next();
      self.col += 1;
    }

    word
  }

  fn read_word(&mut self, curr_char: char) -> String {
    let mut character = self.position.peek();
    let mut word = String::new();
    word.push(curr_char);

    while character.is_some()
      && character != Some(&' ')
      && character != Some(&'(')
      && character != Some(&')')
      && character != Some(&'{')
      && character != Some(&'}')
      && character != Some(&'\n')
    {
      word.push(*character.unwrap());

      if character == Some(&' ') {
        self.row += 1;
        self.col = 0;
      } else {
        self.col += 1;
      }

      self.position.next();
      character = self.position.peek();
    }

    word
  }

  fn next_line(&mut self) {
    while self.position.peek() != Some(&'\n') && self.position.peek() != None {
      self.position.next();
    }
  }
}
