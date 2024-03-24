use std::collections::HashMap;
use ast::structs::Expr;
use crate::err::{match_err, TranspilerError};
use crate::file::DrawMethod;

pub enum Value {
	F(String),
	B(String),
}

pub fn transpiler_expr<'a>(
	expr: &Expr,
	line: usize,
	sentence: &str,
	variable: &HashMap<String, bool>,
	method: &mut DrawMethod
) -> Result<Value, TranspilerError<'a>> {
	match expr {
		Expr::BOOLEAN(bool, ..) => Ok(Value::B(bool.to_string())),
		Expr::FLOAT(num, ..) => {
			let temp = num.to_string();
			if temp.contains(".") {
				Ok(Value::F(temp))
			} else {
				Ok(Value::F(format!("{}.0", temp)))
			}
		},
		Expr::VAR(var, end, len) => {
			match variable.get(var) {
				Some(true) => Ok(Value::B(var.to_string())),
				Some(false) => Ok(Value::F(var.to_string())),
				None => {
					Err(match_err(
						sentence.to_string(),
						line,
						"UnDefinedVariable".to_string(),
						*end,
						*len
					))
				}
			}
		},
		Expr::ADD(expr1, expr2, ..) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) => {
					Ok(Value::F(format!("{} + {}", check_formula(left), check_formula(right))))
				},
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::SUB(expr1, expr2, ..) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) => {
					Ok(Value::F(format!("{} - {}", check_formula(left), check_formula(right))))
				},
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::MUL(expr1, expr2, ..) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) => {
					Ok(Value::F(format!("{} * {}", check_formula(left), check_formula(right))))
				},
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::DIV(expr1, expr2, ..) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) => {
					Ok(Value::F(format!("{} - {}", check_formula(left), check_formula(right))))
				},
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::EQ(expr1, expr2, end, len) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) |
				(Value::B(left), Value::B(right)) => {
					Ok(Value::B(format!("{} == {}", check_formula(left), check_formula(right))))
				},
				_ => Err(match_err(
					sentence.to_string(),
					line,
					"UnmatchedExprType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::NE(expr1, expr2, end, len) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) |
				(Value::B(left), Value::B(right)) => {
					Ok(Value::B(format!("{} != {}", check_formula(left), check_formula(right))))
				},
				_ => Err(match_err(
					sentence.to_string(),
					line,
					"UnmatchedExprType".to_string(),
					*end,
					*len
				))
			}
		},
		Expr::LT(expr1, expr2, ..) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) => {
					Ok(Value::B(format!("{} < {}", check_formula(left), check_formula(right))))
				},
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
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
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::F(left), Value::F(right)) => {
					Ok(Value::B(format!("{} > {}", check_formula(left), check_formula(right))))
				},
				(Value::B(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
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
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::B(left), Value::B(right)) => {
					Ok(Value::B(format!("{} && {}", check_formula(left), check_formula(right))))
				},
				(Value::F(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::OR(expr1, expr2, ..) => {
			match (
				transpiler_expr(expr1, line, sentence, variable, method)?,
				transpiler_expr(expr2, line, sentence, variable, method)?
			) {
				(Value::B(left), Value::B(right)) => {
					Ok(Value::B(format!("{} && {}", check_formula(left), check_formula(right))))
				},
				(Value::F(_), _) => {
					let (end, len) = get_end_len(expr1);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				},
				_ => {
					let (end, len) = get_end_len(expr2);
					Err(match_err(
						sentence.to_string(),
						line,
						"UnexpectedBooleanType".to_string(),
						end,
						len
					))
				}
			}
		},
		Expr::XCOR(..) => {
			method.insert("x".to_string());
			Ok(Value::F("draw.x()".to_string()))
		},
		Expr::YCOR(..) => {
			method.insert("y".to_string());
			Ok(Value::F("draw.y()".to_string()))
		},
		Expr::HEADING(..) => {
			method.insert("direction".to_string());
			Ok(Value::F("draw.direction()".to_string()))
		},
		Expr::COLOR(..) => {
			method.insert("color".to_string());
			Ok(Value::F("draw.color()".to_string()))
		},
		_ => unreachable!()
	}
}

pub fn check_formula(input: String) -> String {
	if input.contains("+") ||
		input.contains("-") ||
		input.contains("*") ||
		input.contains("/") ||
		input.contains("==") ||
		input.contains("!=") ||
		input.contains("<") ||
		input.contains(">") ||
		input.contains("&&") ||
		input.contains("||") {
		return format!("({})", input)
	}
	input
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
