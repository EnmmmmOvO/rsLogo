use ast::structs::{Assign, Stmt};

use crate::draw::Draw;
use crate::err::GenerationError;
use crate::expr::{process_expr_boolean, process_expr_value, process_expr_value_integer};
use crate::function::Function;
use crate::variable::Variable;

pub fn precess_stmt(stmt_list: &Vec<Stmt>, variable: &mut Variable, draw: &mut Draw, function: &Function) ->Result<(), GenerationError<'static>> {
	for stmt in stmt_list {
		match stmt {
			Stmt::IF(expr, stmt, ..) => {
				if process_expr_boolean(expr.as_ref())? {
					precess_stmt(stmt, variable, draw, function)?;
				}
			},
			Stmt::WHILE(expr, stmt, ..) => {
				while process_expr_boolean(expr.as_ref())? {
					precess_stmt(stmt, variable, draw, function)?;
				}
			},
			Stmt::MAKE(assign, expr, ..) => {
				let value = process_expr_value(expr.as_ref())?;
				match assign.as_ref() {
					Assign::VAR(name, ..) => variable.insert(name.to_string(), Some(value)),
					_ => unreachable!()
				}
			},
			Stmt::PENUP(..) => draw.pen_up(),
			Stmt::PENDOWN(..) => draw.pen_down(),
			Stmt::FORWARD(expr, ..) =>
				draw.pen_move(0, process_expr_value(expr.as_ref())?),
			Stmt::BACK(expr, ..) =>
				draw.pen_move(180, process_expr_value(expr.as_ref())?),
			Stmt::LEFT(expr, ..) =>
				draw.pen_move(-90, process_expr_value(expr.as_ref())?),
			Stmt::RIGHT(expr, ..) =>
				draw.pen_move(90, process_expr_value(expr.as_ref())?),
			Stmt::SETPENCOLOR(expr, ..) => {
				let value = process_expr_value_integer(expr.as_ref())?;

				if value < 0 || value > 15 {

				}

				draw.set_pen_color(value as usize);
			},
			Stmt::TURN(expr, ..) =>
				draw.turn(process_expr_value_integer(expr.as_ref())?),
			Stmt::SETHEADING(expr, ..) =>
				draw.set_heading(process_expr_value_integer(expr.as_ref())?),
			Stmt::SETX(expr, ..) =>
				draw.set_x(process_expr_value(expr.as_ref())?),
			Stmt::SETY(expr, ..) =>
				draw.set_y(process_expr_value(expr.as_ref())?),
			Stmt::ADDASSIGN(assign, expr, ..) => {
				// let value = process_expr_value(expr.as_ref())?;
				// match assign.as_ref() {
				// 	Assign::VAR(name, ..) => {
				//
				//
				// 		variable.insert(name.to_string(), Some(value))
				// 	},
				// 	_ => unreachable!()
				// }
			},
			Stmt::FUNC(..) => {},
			Stmt::COMMENT(..) => {},
		}
	}
	Ok(())
}
