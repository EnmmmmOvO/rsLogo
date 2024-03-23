use ast::structs::{Assign, DeclName, Function, Stmt};

use crate::draw::Draw;
use crate::err::{GenerationError, match_err};
use crate::expr::{get_end_len, process_expr, Value};
use crate::variable::Variable;

pub fn process_stmt(
	stmt_list: &Vec<Stmt>,
	variable: &mut Variable,
	draw: &mut Draw,
	function: &Function,
	file: &Vec<String>
) ->Result<(), GenerationError<'static>> {
	for stmt in stmt_list {
		match stmt {
			Stmt::IF(expr, stmt, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(true) => process_stmt(stmt, variable, draw, function, file)?,
					Value::F(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					}
					_ => {}
				}
			},
			Stmt::WHILE(expr, stmt, line) => {
				loop {
					match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
						Value::B(true) => process_stmt(stmt, variable, draw, function, file)?,
						Value::F(_) => {
							let (end, len) = get_end_len(expr.as_ref());
							return Err(match_err(
								file[*line].to_string(),
								*line,
								"UnexpectedNumberType".to_string(),
								end,
								len
							))
						}
						_ => break
					}
				}
			},
			Stmt::MAKE(assign, expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => {
						if let Assign::VAR(name, ..) = assign.as_ref() {
							variable.insert(name.to_string(), Some(num));
						}
					},
				}
			},
			Stmt::ADDASSIGN(assign, expr, line) => {
				let org = match assign.as_ref() {
					Assign::VAR(name, end, len) => {
						match variable.get(name) {
							Some(Some(num)) => num,
							Some(None) => Err(match_err(
								file[*line].to_string(),
								*line,
								"UnDefinedVariableValue".to_string(),
								*end,
								*len
							))?,
							None => Err(match_err(
								file[*line].to_string(),
								*line,
								"UnDefinedVariable".to_string(),
								*end,
								*len
							))?
						}
					},
					_ => unreachable!()
				};

				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => {
						if let Assign::VAR(name, ..) = assign.as_ref() {
							variable.insert(name.to_string(), Some(num + org));
						}
					},
				}
			},
			Stmt::PENUP(..) => draw.pen_up(),
			Stmt::PENDOWN(..) => draw.pen_down(),
			Stmt::FORWARD(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => draw.pen_move(0, num),
				}
			},
			Stmt::BACK(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => draw.pen_move(180, num),
				}
			},
			Stmt::LEFT(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => draw.pen_move(-90, num),
				}
			}
			Stmt::RIGHT(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => draw.pen_move(90, num),
				}
			}
			Stmt::SETPENCOLOR(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => {
						if !num.is_finite() || num.fract() != 0.0 {
							let (end, len) = get_end_len(expr.as_ref());
							return Err(match_err(
								file[*line].to_string(),
								*line,
								"NonIntegerValueError".to_string(),
								end,
								len
							))
						}

						if num < 0.0 || num > 15.0 {
							let (end, len) = get_end_len(expr.as_ref());
							return Err(match_err(
								file[*line].to_string(),
								*line,
								"UnDefinedColor".to_string(),
								end,
								len
							))
						}

						draw.set_pen_color(num as usize);
					},
				}
			},
			Stmt::TURN(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => {
						if !num.is_finite() || num.fract() != 0.0 {
							let (end, len) = get_end_len(expr.as_ref());
							return Err(match_err(
								file[*line].to_string(),
								*line,
								"NonIntegerValueError".to_string(),
								end,
								len
							))
						}

						draw.turn(num as i32);
					},
				}
			},
			Stmt::SETHEADING(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => {
						if !num.is_finite() || num.fract() != 0.0 {
							let (end, len) = get_end_len(expr.as_ref());
							return Err(match_err(
								file[*line].to_string(),
								*line,
								"NonIntegerValueError".to_string(),
								end,
								len
							))
						}

						draw.set_heading(num as i32);
					},
				}
			},
			Stmt::SETX(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => draw.set_x(num)
				}
			},
			Stmt::SETY(expr, line) => {
				match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
					Value::B(_) => {
						let (end, len) = get_end_len(expr.as_ref());
						return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							end,
							len
						))
					},
					Value::F(num) => draw.set_y(num)
				}
			},
			Stmt::FUNC(name, args, line) => {
				let (name, end, len) = match name.as_ref() {
					DeclName::STRING(name, end, len) => (name, end, len),
					_ => unreachable!()
				};

				let func = match function.get(name) {
					Some(func) => func,
					None => return Err(match_err(
						file[*line].to_string(),
						*line,
						"UnDefinedFunction".to_string(),
						*end,
						*len
					))
				};

				if args.len() > func.args.len() {
					return Err(match_err(
						file[*line].to_string(),
						*line,
						"TooManyArguments".to_string(),
						*end,
						*len
					))
				} else if args.len() > func.args.len() {
					return Err(match_err(
						file[*line].to_string(),
						*line,
						"MissingArguments".to_string(),
						*end,
						*len
					))
				}

				for (arg, value) in func.args.iter().zip(args.iter()) {
					match process_expr(value.as_ref(), variable, *line, &file[*line], draw)? {
						Value::F(num) => {
							if let Assign::VAR(name, ..) = arg.as_ref() {
								variable.insert(name.to_string(), Some(num));
							}
						},
						_ => return Err(match_err(
							file[*line].to_string(),
							*line,
							"UnexpectedNumberType".to_string(),
							*end,
							*len
						))
					}
				}

				process_stmt(&func.stmt_list, variable, draw, function, file)?;
			},
			_ => continue
		}
	}
	Ok(())
}
