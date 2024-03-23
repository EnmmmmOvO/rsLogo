use ast::structs::Expr;
use crate::err::GenerationError;

pub fn process_expr_boolean(expr: &Expr) -> Result<bool, GenerationError<'static>> {
	Ok(true)
}

pub fn process_expr_value(expr: &Expr) -> Result<f32, GenerationError<'static>> {
	Ok(0.0)
}

pub fn process_expr_value_integer(expr: &Expr) -> Result<i32, GenerationError<'static>> {
	Ok(0)
}