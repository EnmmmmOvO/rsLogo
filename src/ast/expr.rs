use crate::ast::{structs::Expr, support::check_name};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{char, space0},
    sequence::delimited,
    IResult,
};

fn parse_true(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("\"TRUE"), space0)(input)?;
    Ok((
        temp,
        Expr::Boolean(true, temp.len(), input.len() - temp.len()),
    ))
}

fn parse_false(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("\"FALSE"), space0)(input)?;
    Ok((
        temp,
        Expr::Boolean(false, temp.len(), input.len() - temp.len()),
    ))
}

fn parse_num(input: &str) -> IResult<&str, Expr> {
    let (temp, num) = delimited(
        space0,
        delimited(
            char('"'),
            take_while1(|c: char| c.is_ascii_digit() || c == '.' || c == '-'),
            space0,
        ),
        space0,
    )(input)?;

    match num.parse::<f32>() {
        Ok(f) => Ok((temp, Expr::Float(f, temp.len(), input.len() - temp.len()))),
        Err(_) => Ok((
            temp,
            Expr::Error(
                "InvalidNum".to_string(),
                temp.len(),
                input.len() - temp.len(),
            ),
        )),
    }
}

fn parse_var(input: &str) -> IResult<&str, Expr> {
    let (temp, s) = delimited(space0, delimited(char(':'), is_not(": "), space0), space0)(input)?;

    if check_name(s) {
        Ok((
            temp,
            Expr::Var(s.to_string(), temp.len(), input.len() - temp.len()),
        ))
    } else {
        Ok((
            temp,
            Expr::Error(
                "InvalidName".to_string(),
                temp.len(),
                input.len() - temp.len(),
            ),
        ))
    }
}

fn parse_add(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, char('+'), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Add(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_sub(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, char('-'), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Sub(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_mul(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, char('*'), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Mul(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_div(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, char('/'), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Div(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_eq(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("EQ"), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Eq(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_ne(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("NE"), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Ne(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_lt(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("LT"), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Lt(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_gt(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("GT"), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Gt(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_and(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("AND"), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::And(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_or(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("OR"), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    let (temp, right) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 0),
    ));
    Ok((
        temp,
        Expr::Or(
            Box::new(left),
            Box::new(right),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

fn parse_xcor(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("XCOR"), space0)(input)?;
    Ok((temp, Expr::XCor(temp.len(), input.len() - temp.len())))
}

fn parse_ycor(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("YCOR"), space0)(input)?;
    Ok((temp, Expr::YCor(temp.len(), input.len() - temp.len())))
}

fn parse_heading(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("HEADING"), space0)(input)?;
    Ok((temp, Expr::Heading(temp.len(), input.len() - temp.len())))
}

fn parse_color(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("COLOR"), space0)(input)?;
    Ok((temp, Expr::Color(temp.len(), input.len() - temp.len())))
}

fn parts_err(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, is_not(" ["), space0)(input)?;
    Ok((
        temp,
        Expr::Error(
            "UnexpectedExpr".to_string(),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_true,
        parse_false,
        parse_num,
        parse_var,
        parse_add,
        parse_sub,
        parse_mul,
        parse_div,
        parse_eq,
        parse_ne,
        parse_lt,
        parse_gt,
        parse_and,
        parse_or,
        parse_xcor,
        parse_ycor,
        parse_heading,
        parse_color,
        parts_err,
    ))(input)
}
