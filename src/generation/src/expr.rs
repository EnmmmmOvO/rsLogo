use ast::structs::Expr;
use crate::draw::Draw;
use crate::err::{GenerationError, match_err};
use crate::variable::{Type, Variable};

#[derive(Debug, PartialEq)]
pub enum Value {
	F(f32),
	B(bool),
}

pub fn process_expr(expr: &Expr, variable: &Variable, line: usize, sentence: &str, draw: &Draw) -> Result<Value, GenerationError<'static>> {
	match expr {
		Expr::BOOLEAN(bool,..) => Ok(Value::B(*bool)),
		Expr::FLOAT(num,..) => Ok(Value::F(*num)),
		Expr::VAR(var,..) =>
			match variable.get(var) {
				Some(Some(Type::F(num))) => Ok(Value::F(*num)),
				Some(Some(Type::B(bool))) => Ok(Value::B(*bool)),
				Some(None) => Ok(Value::F(0.0)),
				None => Ok(Value::F(0.0)),
			}
		,
		Expr::ADD(expr1, expr2, end, len) => {
			if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
				if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
					Ok(Value::F(num1 + num2))
				} else {
					Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						*end,
						*len
					))
				}
			} else {
				Err(match_err(
					sentence.to_string(),
					line,
					"UnexpectedBooleanType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::SUB(expr1, expr2, end, len) => {
			if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
				if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
					Ok(Value::F(num1 - num2))
				} else {
					Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						*end,
						*len
					))
				}
			} else {
				Err(match_err(
					sentence.to_string(),
					line,
					"UnexpectedBooleanType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::MUL(expr1, expr2, end, len) => {
			if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
				if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
					Ok(Value::F(num1 * num2))
				} else {
					Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						*end,
						*len
					))
				}
			} else {
				Err(match_err(
					sentence.to_string(),
					line,
					"UnexpectedBooleanType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::DIV(expr1, expr2, end, len) => {
			if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
				if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
					if num2 == 0.0 {
						return Err(match_err(sentence.to_string(),
							line,
							"DivideByZero".to_string(),
							*end,
							*len
						))
					}
					Ok(Value::F(num1 / num2))
				} else {
					Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						*end,
						*len
					))
				}
			} else {
				Err(match_err(
					sentence.to_string(),
					line,
					"UnexpectedBooleanType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::EQ(expr1, expr2, end, len) => {
			match (
				process_expr(expr1, variable, line, sentence, draw)?,
				process_expr(expr2, variable, line, sentence, draw)?
			) {
				(Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 == num2)),
				(Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 == bool2)),
				_ => Err(match_err(sentence.to_string(),
					line,
					"UnmatchedExprType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::NE(expr1, expr2, end, len) => {
			match (
				process_expr(expr1, variable, line, sentence, draw)?,
				process_expr(expr2, variable, line, sentence, draw)?
			) {
				(Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 != num2)),
				(Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 != bool2)),
				_ => Err(match_err(sentence.to_string(),
					line,
					"UnmatchedExprType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::LT(expr1, expr2, ..) => {
			match (
				process_expr(expr1, variable, line, sentence, draw)?,
				process_expr(expr2, variable, line, sentence, draw)?
			) {
				(Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 < num2)),
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::GT(expr1, expr2, ..) => {
			match (
				process_expr(expr1, variable, line, sentence, draw)?,
				process_expr(expr2, variable, line, sentence, draw)?
			) {
				(Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 > num2)),
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::AND(expr1, expr2, ..) => {
			match (
				process_expr(expr1, variable, line, sentence, draw)?,
				process_expr(expr2, variable, line, sentence, draw)?
			) {
				(Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 && bool2)),
				(Value::F(_), _) => {
					let (end, len) = get_end_len(expr1);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedNumberType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedNumberType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::OR(expr1, expr2, ..) => {
			match (
				process_expr(expr1, variable, line, sentence, draw)?,
				process_expr(expr2, variable, line, sentence, draw)?
			) {
				(Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 || bool2)),
				(Value::F(_), _) => {
					let (end, len) = get_end_len(expr1);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedNumberType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					return Err(match_err(sentence.to_string(),
						line,
						"UnexpectedNumberType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::XCOR(..) => Ok(Value::F(draw.x())),
		Expr::YCOR(..) => Ok(Value::F(draw.y())),
		Expr::HEADING(..) => Ok(Value::F(draw.direction() as f32)),
		Expr::COLOR(..) => Ok(Value::F(draw.color() as f32)),
		_ => unreachable!()
	}
}

pub fn get_end_len(expr: &Expr) -> (usize, usize) {
	match expr {
		Expr::FLOAT(.., end, len) | Expr::VAR(.., end, len) |
		Expr::ADD(.., end, len) | Expr::SUB(.., end, len) |
		Expr::MUL(.., end, len) | Expr::DIV(.., end, len) |
		Expr::XCOR(end, len) | Expr::YCOR(end, len) |
		Expr::HEADING(end, len) | Expr::COLOR(end, len) |
		Expr::EQ(.., end, len) | Expr::NE(.., end, len) |
		Expr::LT(.., end, len) | Expr::GT(.., end, len) |
		Expr::AND(.., end, len) | Expr::OR(.., end, len) |
		Expr::BOOLEAN(.., end, len) => (*end, *len),
		_ => unreachable!()
	}
}