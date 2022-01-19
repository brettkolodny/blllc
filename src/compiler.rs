use crate::ast::{Expression, Op};

pub struct Compiler {
  ast: Expression,
  pc: usize,
}

impl Compiler {
  pub fn new(ast: Expression) -> Self {
    Self { ast, pc: 0 }
  }

  pub fn compile(&mut self) -> Result<String, String> {
    let mut byte_code: Vec<String> = Vec::new();
    for expression in self.ast.exprs.clone().into_iter() {
      byte_code = vec![byte_code, self.compile_expression(&expression)?].concat();
    }

    Ok(
      byte_code
        .clone()
        .into_iter()
        .map(|x: String| {
          if x.starts_with("jump-") {
            let split = x.split("-");
            let num = split.last().expect("Malformed jump");

            let byte_dest = {
              byte_code
                .clone()
                .into_iter()
                .position(|s| {
                  return s == format!("dest-{}", num);
                })
                .expect("No destination found")
            };

            let dest = {
              let tmp = format!("{:x}", byte_dest);
              if tmp.len() % 2 != 0 {
                format!("0{}", tmp)
              } else {
                tmp
              }
            };

            dest
          } else if x.starts_with("dest-") {
            "5B".to_owned()
          } else {
            x
          }
        })
        .collect::<Vec<String>>()
        .join(""),
    )
  }

  fn compile_expression(&mut self, expression: &Expression) -> Result<Vec<String>, String> {
    match expression.op {
      Op::Num(i) => {
        let num_vec = {
          let mut bytes = Vec::new();
          let tmp = format!("{:x}", i);
          let mut byte = String::new();
          for char in tmp.chars() {
            byte.push(char);

            if byte.len() == 2 {
              bytes.push(byte);
              byte = String::new();
            }
          }

          if byte.len() > 0 {
            bytes.push(format!("0{}", byte));
          }
          bytes
        };

        let push_code = {
          let num_bytes = num_vec.len();
          if num_bytes > 32 {
            Err("Number too large")
          } else {
            Ok(format!("{:x}", 96 + (num_bytes - 1)))
          }
        };

        self.pc += num_vec.len() + 1;

        Ok(vec![vec![push_code?], num_vec].concat())
      }
      Op::Add | Op::Div | Op::Sub | Op::Mul | Op::Mod | Op::And | Op::Or | Op::XOr => {
        self.compile_multiary(expression)
      }
      Op::Lt | Op::LtOE | Op::Gt | Op::GtOE | Op::Eq | Op::NotEq => self.compile_binary(expression),
      Op::Not => self.compile_unary(expression),
      Op::If => self.compile_if(expression),
      Op::When | Op::Unless => self.compile_when_or_unless(expression),
      Op::End => Ok(Vec::new()),
      _ => Err(String::from("Error")),
    }
  }

  fn compile_multiary(&mut self, arith_expr: &Expression) -> Result<Vec<String>, String> {
    let mut byte_code: Vec<String> = Vec::new();
    let mut counter = 0;

    if arith_expr.exprs.len() > 1 {
      for expression in &arith_expr.exprs {
        counter += 1;
        byte_code = vec![self.compile_expression(&expression)?, byte_code].concat();
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

        self.pc += 1;
        byte_code.push(op_code);
      }

      Ok(byte_code)
    } else {
      self.compile_expression(&arith_expr.exprs[0])
    }
  }

  fn compile_if(&mut self, if_expr: &Expression) -> Result<Vec<String>, String> {
    if if_expr.exprs.len() != 3 {
      return Err("Invalid number of arguments".to_owned());
    }

    let comp_expr = self.compile_expression(&if_expr.exprs[0])?;
    let else_expr = self.compile_expression(&if_expr.exprs[2])?;
    let dest_then = self.pc;
    let then_expr = self.compile_expression(&if_expr.exprs[1])?;
    let dest_next = self.pc;
    let jump_then = vec!["60".to_owned(), format!("jump-{}", dest_then)];
    let jump_next = vec!["60".to_owned(), format!("jump-{}", dest_next)];

    let mut byte_code = Vec::new();
    byte_code.push(comp_expr);
    byte_code.push(jump_then);
    byte_code.push(vec!["57".to_owned()]);
    byte_code.push(else_expr);
    byte_code.push(jump_next);
    byte_code.push(vec!["56".to_owned()]);
    byte_code.push(vec![format!("dest-{}", dest_then)]);
    byte_code.push(then_expr);
    byte_code.push(vec![format!("dest-{}", dest_next)]);

    Ok(byte_code.concat())
  }

  fn compile_when_or_unless(&mut self, when_expr: &Expression) -> Result<Vec<String>, String> {
    if when_expr.exprs.len() != 2 {
      return Err("Invalid number of arguments".to_owned());
    }

    let comp_expr = self.compile_expression(&when_expr.exprs[0])?;
    let then_expr = self.compile_expression(&when_expr.exprs[1])?;

    let dest_next = self.pc;
    let jump_next = vec!["60".to_owned(), format!("jump-{}", dest_next)];

    let mut byte_code = Vec::new();
    byte_code.push(comp_expr);

    if let Op::When = when_expr.op {
      byte_code.push(vec!["15".to_owned()]);
    }
    
    byte_code.push(jump_next);
    byte_code.push(vec!["57".to_owned()]);
    byte_code.push(then_expr);
    byte_code.push(vec![format!("dest-{}", dest_next)]);

    Ok(byte_code.concat())
  }

  fn compile_binary(&mut self, bin_expr: &Expression) -> Result<Vec<String>, String> {
    if bin_expr.exprs.len() != 2 {
      Err("Too many arguments in expression".to_owned())
    } else {
      let mut left: Option<Vec<String>> = None;
      let mut right: Option<Vec<String>> = None;

      let op_code = {
        match bin_expr.op {
          Op::Lt => "10".to_owned(),
          Op::Gt => "11".to_owned(),
          Op::Eq => "14".to_owned(),
          Op::NotEq => "1415".to_owned(),
          Op::LtOE | Op::GtOE => {
            let comp_op = {
              if let Op::LtOE = bin_expr.op {
                Op::Lt
              } else {
                Op::Gt
              }
            };

            let lt_or_gt_expr = Expression {
              op: comp_op,
              exprs: vec![bin_expr.exprs[0].clone(), bin_expr.exprs[1].clone()],
            };

            left = Some(self.compile_expression(&lt_or_gt_expr)?);

            let eq_expr = Expression {
              op: Op::Eq,
              exprs: vec![bin_expr.exprs[0].clone(), bin_expr.exprs[1].clone()],
            };

            right = Some(self.compile_expression(&eq_expr)?);

            "17".to_owned()
          }
          _ => return Err(String::from("Not binary expression")),
        }
      };

      if left.is_none() && right.is_none() {
        left = Some(self.compile_expression(&bin_expr.exprs[0])?);
        right = Some(self.compile_expression(&bin_expr.exprs[1])?);
      }

      self.pc += op_code.len() / 2;
      let byte_code = vec![right.unwrap(), left.unwrap(), vec![op_code]].concat();
      Ok(byte_code)
    }
  }

  fn compile_unary(&mut self, unary_expr: &Expression) -> Result<Vec<String>, String> {
    if unary_expr.exprs.len() != 1 {
      return Err("Invalid number of arguments".to_owned());
    }

    let byte_code = vec![
      self.compile_expression(&unary_expr.exprs[0])?,
      vec!["19".to_owned()],
    ]
    .concat();

    self.pc += 1;

    Ok(byte_code)
  }
}
