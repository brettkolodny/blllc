use crate::ast::{Expression, Op};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::mem;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  current_token: Token,
  peek_token: Token,
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer<'a>) -> Self {
    let current_token = lexer.next();
    let peek_token = lexer.next();

    Parser {
      lexer,
      current_token,
      peek_token,
    }
  }

  fn advance_tokens(&mut self) {
    mem::swap(&mut self.peek_token, &mut self.current_token);
    self.peek_token = self.lexer.next();
  }

  pub fn parse(&mut self) -> Result<Expression, String> {
    let mut ast = Expression::new_program();

    let mut expressions: Vec<Expression> = vec![];

    while self.current_token.token_type != TokenType::EOF {
      expressions.push(self.parse_program()?);
      self.advance_tokens();
    }

    ast.exprs = expressions;

    Ok(ast)
  }

  fn parse_program(&mut self) -> Result<Expression, String> {
    match &self.current_token.token_type {
      TokenType::LPAREN | TokenType::RPAREN => {
        self.advance_tokens();
        self.parse_program()
      }
      TokenType::ADD => self.parse_expression(Op::Add),
      TokenType::SUB => self.parse_expression(Op::Sub),
      TokenType::MUL => self.parse_expression(Op::Mul),
      TokenType::DIV => self.parse_expression(Op::Div),
      TokenType::MOD => self.parse_expression(Op::Mod),
      TokenType::LT => self.parse_expression(Op::Lt),
      TokenType::LTOE => self.parse_expression(Op::LtOE),
      TokenType::GT => self.parse_expression(Op::Gt),
      TokenType::GTOE => self.parse_expression(Op::GtOE),
      TokenType::EQ => self.parse_expression(Op::Eq),
      TokenType::NEQ => self.parse_expression(Op::NotEq),
      TokenType::BAND => self.parse_expression(Op::And),
      TokenType::BOR => self.parse_expression(Op::Or),
      TokenType::BXOR => self.parse_expression(Op::XOr),
      TokenType::BNOT => self.parse_expression(Op::Not),
      TokenType::INT(i) => Ok(Expression {
        op: Op::Num(*i),
        exprs: vec![],
      }),
      TokenType::IDENT(i) => match i.as_ref() {
        "if" => self.parse_expression(Op::If),
        "when" => self.parse_expression(Op::When),
        "unless" => self.parse_expression(Op::Unless),
        _ => Err(String::from("Error pase_expression")),
      },
      TokenType::EOF => Ok(Expression::end_program()),
      _ => Err(String::from("Error pase_expression")),
    }
  }

  fn parse_expression(&mut self, op: Op) -> Result<Expression, String> {
    let mut add_expr = Expression { op, exprs: vec![] };

    let mut exprs = vec![];

    while self.peek_token.token_type != TokenType::RPAREN
      && self.peek_token.token_type != TokenType::EOF
    {
      self.advance_tokens();
      exprs.push(self.parse_program()?);
    }

    self.advance_tokens();

    add_expr.exprs = exprs;

    Ok(add_expr)
  }
}
