use crate::ast::{
    assign::parse_assign,
    decl::parse_decl_name,
    expr::parse_expr,
    structs::{Expr, Stmt},
};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space0, sequence::delimited, IResult,
};

fn parse_if(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("IF"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::If(Box::new(expr), vec![], 0)))
}

fn parse_while(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("WHILE"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::While(Box::new(expr), vec![], 0)))
}

fn parse_make(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("MAKE"), space0)(input)?;
    let (temp, assign) = parse_assign(temp)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::Make(Box::new(assign), Box::new(expr), 0)))
}

fn parse_penup(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("PENUP"), space0)(input)?;
    Ok((temp, Stmt::PenUp(0)))
}

fn parse_pendown(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("PENDOWN"), space0)(input)?;
    Ok((temp, Stmt::PenDown(0)))
}

fn parse_forward(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("FORWARD"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::Forward(Box::new(expr), 0)))
}

fn parse_back(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("BACK"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::Back(Box::new(expr), 0)))
}

fn parse_left(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("LEFT"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::Left(Box::new(expr), 0)))
}

fn parse_right(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("RIGHT"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::Right(Box::new(expr), 0)))
}

fn parse_setpencolor(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("SETPENCOLOR"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::SetPenColor(Box::new(expr), 0)))
}

fn parse_turn(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("TURN"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::Turn(Box::new(expr), 0)))
}

fn parse_setheading(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("SETHEADING"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::SetHeading(Box::new(expr), 0)))
}

fn parse_setx(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("SETX"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::SetX(Box::new(expr), 0)))
}

fn parse_sety(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("SETY"), space0)(input)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::SetY(Box::new(expr), 0)))
}

fn parse_addassign(input: &str) -> IResult<&str, Stmt> {
    let (temp, _) = delimited(space0, tag("ADDASSIGN"), space0)(input)?;
    let (temp, assign) = parse_assign(temp)?;
    let (temp, expr) = parse_expr(temp).unwrap_or((
        temp,
        Expr::Error("MissingOperand".to_string(), temp.len(), 1),
    ));
    Ok((temp, Stmt::AddAssign(Box::new(assign), Box::new(expr), 0)))
}

fn parse_func(input: &str) -> IResult<&str, Stmt> {
    let (mut temp, func_name) = parse_decl_name(input)?;
    let mut var = vec![];

    while let Ok((str, expr)) = parse_expr(temp) {
        var.push(expr);
        temp = str;
    }

    Ok((temp, Stmt::Func(Box::new(func_name), var, 0)))
}

pub fn parse_stmt(input: &str) -> IResult<&str, Stmt> {
    alt((
        parse_if,
        parse_while,
        parse_make,
        parse_penup,
        parse_pendown,
        parse_forward,
        parse_back,
        parse_left,
        parse_right,
        parse_setpencolor,
        parse_turn,
        parse_setheading,
        parse_setx,
        parse_sety,
        parse_addassign,
        parse_func,
    ))(input)
}
