#[derive(Debug)]
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
  Start,
  End,
  Num(u32),
}
