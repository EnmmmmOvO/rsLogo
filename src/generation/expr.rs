use crate::ast::structs::Expr;
use crate::generation::{
    draw::Draw,
    err::{match_err, GenerationError},
    variable::{Type, Variable},
};

#[derive(Debug, PartialEq)]
pub enum Value {
    F(f32),
    B(bool),
}

pub fn process_expr(
    expr: &Expr,
    variable: &Variable,
    line: usize,
    sentence: &str,
    draw: &Draw,
) -> Result<Value, GenerationError<'static>> {
    match expr {
        Expr::Boolean(bool, ..) => Ok(Value::B(*bool)),
        Expr::Float(num, ..) => Ok(Value::F(*num)),
        Expr::Var(var, end, len) => match variable.get(var) {
            Some(Some(Type::F(num))) => Ok(Value::F(*num)),
            Some(Some(Type::B(bool))) => Ok(Value::B(*bool)),
            Some(None) => Err(match_err(
                sentence.to_string(),
                line,
                "UnDefinedVariableValue".to_string(),
                *end,
                *len,
            )),
            None => Err(match_err(
                sentence.to_string(),
                line,
                "UnDefinedVariable".to_string(),
                *end,
                *len,
            )),
        },
        Expr::Add(expr1, expr2, end, len) => {
            if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
                if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
                    Ok(Value::F(num1 + num2))
                } else {
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        *end,
                        *len,
                    ))
                }
            } else {
                Err(match_err(
                    sentence.to_string(),
                    line,
                    "UnexpectedBooleanType".to_string(),
                    *end,
                    *len,
                ))
            }
        }
        Expr::Sub(expr1, expr2, end, len) => {
            if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
                if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
                    Ok(Value::F(num1 - num2))
                } else {
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        *end,
                        *len,
                    ))
                }
            } else {
                Err(match_err(
                    sentence.to_string(),
                    line,
                    "UnexpectedBooleanType".to_string(),
                    *end,
                    *len,
                ))
            }
        }
        Expr::Mul(expr1, expr2, end, len) => {
            if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
                if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
                    Ok(Value::F(num1 * num2))
                } else {
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        *end,
                        *len,
                    ))
                }
            } else {
                Err(match_err(
                    sentence.to_string(),
                    line,
                    "UnexpectedBooleanType".to_string(),
                    *end,
                    *len,
                ))
            }
        }
        Expr::Div(expr1, expr2, end, len) => {
            if let Value::F(num1) = process_expr(expr1, variable, line, sentence, draw)? {
                if let Value::F(num2) = process_expr(expr2, variable, line, sentence, draw)? {
                    if num2 == 0.0 {
                        return Err(match_err(
                            sentence.to_string(),
                            line,
                            "DivideByZero".to_string(),
                            *end,
                            *len,
                        ));
                    }
                    Ok(Value::F(num1 / num2))
                } else {
                    Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        *end,
                        *len,
                    ))
                }
            } else {
                Err(match_err(
                    sentence.to_string(),
                    line,
                    "UnexpectedBooleanType".to_string(),
                    *end,
                    *len,
                ))
            }
        }
        Expr::Eq(expr1, expr2, end, len) => {
            match (
                process_expr(expr1, variable, line, sentence, draw)?,
                process_expr(expr2, variable, line, sentence, draw)?,
            ) {
                (Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 == num2)),
                (Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 == bool2)),
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
                process_expr(expr1, variable, line, sentence, draw)?,
                process_expr(expr2, variable, line, sentence, draw)?,
            ) {
                (Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 != num2)),
                (Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 != bool2)),
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
                process_expr(expr1, variable, line, sentence, draw)?,
                process_expr(expr2, variable, line, sentence, draw)?,
            ) {
                (Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 < num2)),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ));
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ));
                }
            }
        }
        Expr::Gt(expr1, expr2, ..) => {
            match (
                process_expr(expr1, variable, line, sentence, draw)?,
                process_expr(expr2, variable, line, sentence, draw)?,
            ) {
                (Value::F(num1), Value::F(num2)) => Ok(Value::B(num1 > num2)),
                (Value::B(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ));
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedBooleanType".to_string(),
                        end,
                        len,
                    ));
                }
            }
        }
        Expr::And(expr1, expr2, ..) => {
            match (
                process_expr(expr1, variable, line, sentence, draw)?,
                process_expr(expr2, variable, line, sentence, draw)?,
            ) {
                (Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 && bool2)),
                (Value::F(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedNumberType".to_string(),
                        end,
                        len,
                    ));
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedNumberType".to_string(),
                        end,
                        len,
                    ));
                }
            }
        }
        Expr::Or(expr1, expr2, ..) => {
            match (
                process_expr(expr1, variable, line, sentence, draw)?,
                process_expr(expr2, variable, line, sentence, draw)?,
            ) {
                (Value::B(bool1), Value::B(bool2)) => Ok(Value::B(bool1 || bool2)),
                (Value::F(_), _) => {
                    let (end, len) = get_end_len(expr1);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedNumberType".to_string(),
                        end,
                        len,
                    ));
                }
                _ => {
                    let (end, len) = get_end_len(expr2);
                    return Err(match_err(
                        sentence.to_string(),
                        line,
                        "UnexpectedNumberType".to_string(),
                        end,
                        len,
                    ));
                }
            }
        }
        Expr::XCor(..) => Ok(Value::F(draw.x())),
        Expr::YCor(..) => Ok(Value::F(draw.y())),
        Expr::Heading(..) => Ok(Value::F(draw.direction() as f32)),
        Expr::Color(..) => Ok(Value::F(draw.color() as f32)),
        _ => unreachable!(),
    }
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
