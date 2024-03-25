use crate::ast::structs::{Assign, DeclName, Function, Stmt};

use crate::generation::{
    draw::Draw,
    err::{match_err, GenerationError},
    expr::{get_end_len, process_expr, Value},
    variable::{Type, Variable},
};

pub fn process_stmt(
    stmt_list: &Vec<Stmt>,
    variable: &mut Variable,
    draw: &mut Draw,
    function: &Function,
    file: &[String],
) -> Result<(), GenerationError<'static>> {
    for stmt in stmt_list {
        match stmt {
            Stmt::If(expr, stmt, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(true) => process_stmt(stmt, variable, draw, function, file)?,
                    Value::F(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    _ => {}
                }
            }
            Stmt::While(expr, stmt, line) => loop {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(true) => process_stmt(stmt, variable, draw, function, file)?,
                    Value::F(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    _ => break,
                }
            },
            Stmt::Make(assign, expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(bool) => {
                        if let Assign::Var(name, ..) = assign.as_ref() {
                            variable.insert_bool(name.to_string(), Some(bool));
                        }
                    }
                    Value::F(num) => {
                        if let Assign::Var(name, ..) = assign.as_ref() {
                            variable.insert_num(name.to_string(), Some(num));
                        }
                    }
                }
            }
            Stmt::AddAssign(assign, expr, line) => {
                let org = match assign.as_ref() {
                    Assign::Var(name, end, len) => match variable.get(name) {
                        Some(Some(Type::F(num))) => num,
                        Some(Some(Type::B(_))) => Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedBooleanType".to_string(),
                            *end,
                            *len,
                        ))?,
                        Some(None) => Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnDefinedVariableValue".to_string(),
                            *end,
                            *len,
                        ))?,
                        None => Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnDefinedVariable".to_string(),
                            *end,
                            *len,
                        ))?,
                    },
                    _ => unreachable!(),
                };

                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => {
                        if let Assign::Var(name, ..) = assign.as_ref() {
                            variable.insert_num(name.to_string(), Some(num + org));
                        }
                    }
                }
            }
            Stmt::PenUp(..) => draw.pen_up(),
            Stmt::PenDown(..) => draw.pen_down(),
            Stmt::Forward(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => draw.pen_move(0, num),
                }
            }
            Stmt::Back(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => draw.pen_move(180, num),
                }
            }
            Stmt::Left(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => draw.pen_move(-90, num),
                }
            }
            Stmt::Right(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => draw.pen_move(90, num),
                }
            }
            Stmt::SetPenColor(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => {
                        if !num.is_finite() || num.fract() != 0.0 {
                            let (end, len) = get_end_len(expr.as_ref());
                            return Err(match_err(
                                file[*line].to_string(),
                                *line,
                                "NonIntegerValueError".to_string(),
                                end,
                                len,
                            ));
                        }

                        if !(0.0..=15.0).contains(&num) {
                            let (end, len) = get_end_len(expr.as_ref());
                            return Err(match_err(
                                file[*line].to_string(),
                                *line,
                                "UnDefinedColor".to_string(),
                                end,
                                len,
                            ));
                        }

                        draw.set_pen_color(num as usize);
                    }
                }
            }
            Stmt::Turn(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => {
                        if !num.is_finite() || num.fract() != 0.0 {
                            let (end, len) = get_end_len(expr.as_ref());
                            return Err(match_err(
                                file[*line].to_string(),
                                *line,
                                "NonIntegerValueError".to_string(),
                                end,
                                len,
                            ));
                        }

                        draw.turn(num as i32);
                    }
                }
            }
            Stmt::SetHeading(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => {
                        if !num.is_finite() || num.fract() != 0.0 {
                            let (end, len) = get_end_len(expr.as_ref());
                            return Err(match_err(
                                file[*line].to_string(),
                                *line,
                                "NonIntegerValueError".to_string(),
                                end,
                                len,
                            ));
                        }

                        draw.set_heading(num as i32);
                    }
                }
            }
            Stmt::SetX(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => draw.set_x(num),
                }
            }
            Stmt::SetY(expr, line) => {
                match process_expr(expr.as_ref(), variable, *line, &file[*line], draw)? {
                    Value::B(_) => {
                        let (end, len) = get_end_len(expr.as_ref());
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ));
                    }
                    Value::F(num) => draw.set_y(num),
                }
            }
            Stmt::Func(name, args, line) => {
                let (name, end, len) = match name.as_ref() {
                    DeclName::String(name, end, len) => (name, end, len),
                    _ => unreachable!(),
                };

                let func = match function.get(name) {
                    Some(func) => func,
                    None => {
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnDefinedFunction".to_string(),
                            *end,
                            *len,
                        ))
                    }
                };

                if args.len() > func.args.len() {
                    return Err(match_err(
                        file[*line].to_string(),
                        *line,
                        "TooManyArguments".to_string(),
                        *end,
                        *len,
                    ));
                } else if args.len() > func.args.len() {
                    return Err(match_err(
                        file[*line].to_string(),
                        *line,
                        "MissingArguments".to_string(),
                        *end,
                        *len,
                    ));
                }

                for (arg, value) in func.args.iter().zip(args.iter()) {
                    match process_expr(value, variable, *line, &file[*line], draw)? {
                        Value::F(num) => {
                            if let Assign::Var(name, ..) = &arg {
                                variable.insert_num(name.to_string(), Some(num));
                            }
                        }
                        Value::B(bool) => {
                            if let Assign::Var(name, ..) = &arg {
                                variable.insert_bool(name.to_string(), Some(bool));
                            }
                        }
                    }
                }

                process_stmt(&func.stmt_list, variable, draw, function, file)?;
            }
            _ => continue,
        }
    }
    Ok(())
}
