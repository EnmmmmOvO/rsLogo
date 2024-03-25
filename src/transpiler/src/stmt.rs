use crate::err::{match_err, TranspilerError};
use crate::expr::{get_end_len, transpiler_expr, Value};
use crate::file::DrawMethod;
use ast::structs::{Assign, DeclName, Function, Stmt};
use std::collections::HashMap;

pub fn transpiler_stmt<'a>(
    stmt_list: &Vec<Stmt>,
    file: &[String],
    method: &mut DrawMethod,
    variable: &mut HashMap<String, bool>,
    tab: i32,
    ast: &Function,
) -> Result<String, TranspilerError<'a>> {
    let mut result: Vec<String> = Vec::new();

    for stmt in stmt_list {
        match stmt {
            Stmt::IF(expr, stmt, line) => {
                let condition = match transpiler_expr(expr, *line, &file[*line], variable, method)?
                {
                    Value::B(bool) => bool,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                result.push(format!(
                    "{}if {} {{\n{}{}}}\n",
                    set_tab(tab),
                    condition,
                    transpiler_stmt(stmt, file, method, variable, tab + 1, ast)?,
                    set_tab(tab)
                ));
            }
            Stmt::WHILE(expr, stmt, line) => {
                let condition = match transpiler_expr(expr, *line, &file[*line], variable, method)?
                {
                    Value::B(bool) => bool,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                result.push(format!(
                    "{}while {} {{\n{}{}}}\n",
                    set_tab(tab),
                    condition,
                    transpiler_stmt(stmt, file, method, variable, tab + 1, ast)?,
                    set_tab(tab)
                ));
            }
            Stmt::MAKE(assign, expr, line) => {
                let name = match assign.as_ref() {
                    Assign::VAR(name, ..) => name,
                    _ => unreachable!(),
                };

                match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => match variable.get(name) {
                        Some(true) | None => {
                            result.push(format!("{}let mut {} = {};\n", set_tab(tab), name, num));
                            variable.insert(name.to_string(), false);
                        }
                        _ => result.push(format!("{}{} = {};\n", set_tab(tab), name, num)),
                    },
                    Value::B(bool) => match variable.get(name) {
                        Some(false) | None => {
                            result.push(format!("{}let mut {} = {};\n", set_tab(tab), name, bool));
                            variable.insert(name.to_string(), true);
                        }
                        _ => result.push(format!("{}{} = {};\n", set_tab(tab), name, bool)),
                    },
                };
            }
            Stmt::ADDASSIGN(assign, expr, line) => {
                let name = match assign.as_ref() {
                    Assign::VAR(name, ..) => name,
                    _ => unreachable!(),
                };

                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                match variable.get(name) {
                    Some(true) => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedBooleanType".to_string(),
                            end,
                            len,
                        ))?
                    }
                    None => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnDefinedVariable".to_string(),
                            end,
                            len,
                        ))?
                    }
                    _ => result.push(format!("{}{} += {};\n", set_tab(tab), name, value)),
                }
            }
            Stmt::PENUP(..) => {
                method.insert("pen_up".to_string());
                result.push(format!("{}draw.pen_up();\n", set_tab(tab)));
            }
            Stmt::PENDOWN(..) => {
                method.insert("pen_down".to_string());
                result.push(format!("{}draw.pen_down();\n", set_tab(tab)));
            }
            Stmt::FORWARD(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("pen_move".to_string());
                result.push(format!("{}draw.pen_move(0, {})?;\n", set_tab(tab), value));
            }
            Stmt::BACK(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("pen_move".to_string());
                result.push(format!("{}draw.pen_move(180, {})?;\n", set_tab(tab), value));
            }
            Stmt::LEFT(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("pen_move".to_string());
                result.push(format!("{}draw.pen_move(-90, {})?;\n", set_tab(tab), value));
            }
            Stmt::RIGHT(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("pen_move".to_string());
                result.push(format!("{}draw.pen_move(90, {})?;\n", set_tab(tab), value));
            }
            Stmt::SETPENCOLOR(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("set_pen_color".to_string());
                result.push(format!("{}draw.set_pen_color({})?;\n", set_tab(tab), value));
            }
            Stmt::TURN(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("turn".to_string());
                result.push(format!("{}draw.turn({})?;\n", set_tab(tab), value));
            }
            Stmt::SETHEADING(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("set_heading".to_string());
                result.push(format!("{}draw.set_heading({})?;\n", set_tab(tab), value));
            }
            Stmt::SETX(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("set_x".to_string());
                result.push(format!("{}draw.set_x({});\n", set_tab(tab), value));
            }
            Stmt::SETY(expr, line) => {
                let value = match transpiler_expr(expr, *line, &file[*line], variable, method)? {
                    Value::F(num) => num,
                    _ => {
                        let (end, len) = get_end_len(expr);
                        Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "UnexpectedNumberType".to_string(),
                            end,
                            len,
                        ))?
                    }
                };

                method.insert("set_y".to_string());
                result.push(format!("{}draw.set_y({});\n", set_tab(tab), value));
            }
            Stmt::FUNC(name, args, line) => {
                let (name, end, len) = match name.as_ref() {
                    DeclName::STRING(name, end, len) => (name, end, len),
                    _ => unreachable!(),
                };

                match ast.get_args_value(name) {
                    Some(value) => {
                        if args.len() > value {
                            return Err(match_err(
                                file[*line].to_string(),
                                *line,
                                "TooManyArguments".to_string(),
                                *end,
                                *len,
                            ));
                        } else if args.len() > value {
                            return Err(match_err(
                                file[*line].to_string(),
                                *line,
                                "MissingArguments".to_string(),
                                *end,
                                *len,
                            ));
                        }
                    }
                    None => {
                        return Err(match_err(
                            file[*line].to_string(),
                            *line,
                            "FunctionNotDefined".to_string(),
                            *end,
                            *len,
                        ))
                    }
                };

                let mut args_result: Vec<String> = Vec::new();
                for i in args.iter() {
                    match transpiler_expr(i, *line, &file[*line], variable, method) {
                        Ok(Value::F(num)) => args_result.push(num),
                        _ => {
                            let (end, len) = get_end_len(i);
                            return Err(match_err(
                                file[*line].to_string(),
                                *line,
                                "UnexpectedBooleanType".to_string(),
                                end,
                                len,
                            ));
                        }
                    }
                }

                result.push(format!(
                    "{}{}(draw, {});\n",
                    set_tab(tab),
                    name,
                    args_result.join(", ")
                ));
            }
            Stmt::COMMENT(comment, ..) => result.push(format!("{}// {}\n", set_tab(tab), comment)),
        }
    }

    Ok(result.join("\n"))
}

fn set_tab(tab: i32) -> String {
    (0..tab).map(|_| "\t").collect()
}
