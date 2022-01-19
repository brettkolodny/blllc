#[derive(Debug, Clone)]
pub struct Expression {
  pub op: Op,
  pub exprs: Vec<Expression>,
}

impl Expression {
  pub fn new_program() -> Self {
    Self {
      op: Op::Start,
      exprs: vec![],
    }
  }

  pub fn end_program() -> Self {
    Self {
      op: Op::End,
      exprs: vec![]
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
  Add,
  Mul,
  Div,
  Sub,
  Mod,
  And,
  Or,
  XOr,
  Not,
  Lt,
  LtOE,
  Gt,
  GtOE,
  Eq,
  NotEq,
  If,
  Start,
  End,
  Num(u32),
}
