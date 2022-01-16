use crate::ast::{Expression, Op};

pub struct Compiler {
  ast: Expression,
}

impl Compiler {
  pub fn new(ast: Expression) -> Self {
    Self { ast }
  }

  pub fn compile(self) -> Result<String, String> {
    let mut byte_code = String::new();
    for expression in &self.ast.exprs {
      byte_code.push_str(&self.compile_expression(&expression)?);
    }

    Ok(byte_code)
  }

  fn compile_expression(&self, expression: &Expression) -> Result<String, String> {
    match expression.op {
      Op::Add => self.compile_add(expression),
      _ => Err(String::from("Error")),
    }
  }

  fn compile_add(&self, add_expr: &Expression) -> Result<String, String> {
    let mut byte_code = String::new();
    let mut counter = 0;

    if add_expr.exprs.len() > 1 {
      for expression in &add_expr.exprs {
        counter += 1;
        match expression.op {
          Op::Num(i) => {
            let num_string = {
              let tmp = i.to_string();
              if tmp.len() < 2 {
                format!("0{}", tmp)
              } else {
                tmp
              }
            };

            byte_code.push_str(&format!("60{}", num_string));
          }
          _ => {
            byte_code += &self.compile_expression(&expression)?
          }
        }
      }

      for _ in 0..(counter - 1) {
        byte_code += "01"
      }

      Ok(byte_code)
    } else {
      Err(String::from("Not enough arguments in add"))
    }
  }
}
