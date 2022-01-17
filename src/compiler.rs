use crate::ast::{Expression, Op};

pub struct Compiler {
  ast: Expression,
}

impl Compiler {
  pub fn new(ast: Expression) -> Self {
    Self { ast }
  }

  pub fn compile(self) -> Result<String, String> {
    let mut byte_code: Vec<String> = Vec::new();
    for expression in &self.ast.exprs {
      byte_code = vec![byte_code, self.compile_expression(&expression)?].concat();
    }

    Ok(byte_code.join(""))
  }

  fn compile_expression(&self, expression: &Expression) -> Result<Vec<String>, String> {
    match expression.op {
      Op::Add | Op::Div | Op::Sub | Op::Mul | Op::Mod | Op::And | Op::Or | Op::XOr => {
        self.compile_arithmetic(expression)
      }
      Op::Lt => self.compile_binary(expression),
      _ => Err(String::from("Error")),
    }
  }

  fn compile_arithmetic(&self, arith_expr: &Expression) -> Result<Vec<String>, String> {
    let mut byte_code: Vec<String> = Vec::new();
    let mut counter = 0;

    if arith_expr.exprs.len() > 1 {
      for expression in &arith_expr.exprs {
        counter += 1;
        match expression.op {
          Op::Num(i) => {
            let num_string = {
              let tmp = format!("{:x}", i);
              if tmp.len() % 2 != 0 {
                format!("0{}", tmp)
              } else {
                tmp
              }
            };

            let push_code = {
              let num_bytes = num_string.len() / 2;

              if num_bytes > 32 {
                Err("Number too large")
              } else {
                Ok(format!("{:x}", 96 + (num_bytes - 1)))
              }
            };
            byte_code = vec![vec![push_code?, num_string], byte_code].concat();
          }
          _ => {
            byte_code = vec![byte_code, self.compile_expression(&expression)?].concat();
          }
        }
      }

      for _ in 0..(counter - 1) {
        let op_code = {
          match arith_expr.op {
            Op::Add => "01".to_owned(),
            Op::Mul => "02".to_owned(),
            Op::Sub => "03".to_owned(),
            Op::Div => "04".to_owned(),
            Op::Mod => "06".to_owned(),
            Op::And => "16".to_owned(),
            Op::Or => "17".to_owned(),
            Op::XOr => "18".to_owned(),
            _ => return Err(String::from("Not arimetic expression")),
          }
        };

        byte_code.push(op_code);
      }

      Ok(byte_code)
    } else {
      self.compile_expression(&arith_expr.exprs[0])
    }
  }

  fn compile_binary(&self, bin_expr: &Expression) -> Result<Vec<String>, String> {
    if bin_expr.exprs.len() != 2 {
      Err("Too many arguments in expression".to_owned())
    } else {
      let left = self.compile_expression(&bin_expr.exprs[0])?;
      let right = self.compile_expression(&bin_expr.exprs[1])?;

      let op_code = {
        match bin_expr.op {
          Op::Lt => "10".to_owned(),
          _ => return Err(String::from("Not binary expression")),
        }
      };

      let byte_code = vec![
        right,
        vec!["7F".to_owned()],
        left,
        vec!["7F".to_owned()],
        vec![op_code],
      ]
      .concat();

      Ok(byte_code)
    }
  }
}
