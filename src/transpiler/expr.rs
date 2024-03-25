use crate::ast::structs::Expr;
use crate::transpiler::{
    err::{match_err, TranspilerError},
    file::DrawMethod,
};
use std::collections::HashMap;

pub enum Value {
    F(String),
    B(String),
}

pub fn transpiler_expr<'a>(
    expr: &Expr,
    line: usize,
    sentence: &str,
    variable: &HashMap<String, bool>,
    method: &mut DrawMethod,
) -> Result<Value, TranspilerError<'a>> {
    match expr {
        Expr::Boolean(bool, ..) => Ok(Value::B(bool.to_string())),
        Expr::Float(num, ..) => {
            let temp = num.to_string();
            if temp.contains('.') {
                Ok(Value::F(temp))
            } else {
                Ok(Value::F(format!("{}.0", temp)))
            }
        }
        Expr::Var(var, end, len) => match variable.get(var) {
            Some(true) => Ok(Value::B(var.to_string())),
            Some(false) => Ok(Value::F(var.to_string())),
            None => Err(match_err(
                sentence.to_string(),
                line,
                "UnDefinedVariable".to_string(),
                *end,
                *len,
            )),
        },
        Expr::Add(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) => Ok(Value::F(format!(
                    "{} + {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::Sub(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) => Ok(Value::F(format!(
                    "{} - {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::Mul(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) => Ok(Value::F(format!(
                    "{} * {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::Div(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) => Ok(Value::F(format!(
                    "{} / {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::Eq(expr1, expr2, end, len) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) | (Value::B(left), Value::B(right)) => {
                    Ok(Value::B(format!(
                        "{} == {}",
                        check_formula(left),
                        check_formula(right)
                    )))
                }
                _ => Err(match_err(
                    sentence.to_string(),
                    line,
                    "UnmatchedExprType".to_string(),
                    *end,
                    *len,
                )),
            }
        }
        Expr::Ne(expr1, expr2, end, len) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) | (Value::B(left), Value::B(right)) => {
                    Ok(Value::B(format!(
                        "{} != {}",
                        check_formula(left),
                        check_formula(right)
                    )))
                }
                _ => Err(match_err(
                    sentence.to_string(),
                    line,
                    "UnmatchedExprType".to_string(),
                    *end,
                    *len,
                )),
            }
        }
        Expr::Lt(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) => Ok(Value::B(format!(
                    "{} < {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::Gt(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::F(left), Value::F(right)) => Ok(Value::B(format!(
                    "{} > {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::And(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::B(left), Value::B(right)) => Ok(Value::B(format!(
                    "{} && {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::F(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::Or(expr1, expr2, ..) => {
            match (
                transpiler_expr(expr1, line, sentence, variable, method)?,
                transpiler_expr(expr2, line, sentence, variable, method)?,
            ) {
                (Value::B(left), Value::B(right)) => Ok(Value::B(format!(
                    "{} || {}",
                    check_formula(left),
                    check_formula(right)
                ))),
                (Value::F(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ))
                }
            }
        }
        Expr::XCor(..) => {
            method.insert("x".to_string());
            Ok(Value::F("draw.x()".to_string()))
        }
        Expr::YCor(..) => {
            method.insert("y".to_string());
            Ok(Value::F("draw.y()".to_string()))
        }
        Expr::Heading(..) => {
            method.insert("direction".to_string());
            Ok(Value::F("draw.direction()".to_string()))
        }
        Expr::Color(..) => {
            method.insert("color".to_string());
            Ok(Value::F("draw.color()".to_string()))
        }
        _ => unreachable!(),
    }
}

pub fn check_formula(input: String) -> String {
    if input.contains('+')
        || input.contains('-')
        || input.contains('*')
        || input.contains('/')
        || input.contains("==")
        || input.contains("!=")
        || input.contains('<')
        || input.contains('>')
        || input.contains("&&")
        || input.contains("||")
    {
        return format!("({})", input);
    }
    input
}

pub fn get_end_len(expr: &Expr) -> (usize, usize) {
    match expr {
        Expr::Float(.., end, len)
        | Expr::Var(.., end, len)
        | Expr::Add(.., end, len)
        | Expr::Sub(.., end, len)
        | Expr::Mul(.., end, len)
        | Expr::Div(.., end, len)
        | Expr::XCor(end, len)
        | Expr::YCor(end, len)
        | Expr::Heading(end, len)
        | Expr::Color(end, len)
        | Expr::Eq(.., end, len)
        | Expr::Ne(.., end, len)
        | Expr::Lt(.., end, len)
        | Expr::Gt(.., end, len)
        | Expr::And(.., end, len)
        | Expr::Or(.., end, len)
        | Expr::Boolean(.., end, len) => (*end, *len),
        _ => unreachable!(),
    }
}
