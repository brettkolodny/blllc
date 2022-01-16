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
      Op::Add | Op::Div | Op::Sub | Op::Mul | Op::Mod => self.compile_arithmetic(expression),
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
            _ => return Err(String::from("Not arimetic expression")),
          }
        };

        byte_code.push(op_code);
      }

      Ok(byte_code)
    } else {
      Err(String::from("Not enough arguments in add"))
    }
  }
}
