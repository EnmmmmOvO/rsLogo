use nom::{
	branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    sequence::delimited,
    IResult,
};
use crate::{
	assign::parse_assign,
	expr::parse_expr,
	structs::Stmt
};
use crate::decl::parse_decl_name;
use crate::structs::Expr;

fn parse_if(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("IF"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::IF(Box::new(expr), vec![], temp.len(), input.len() - temp.len())))
}

fn parse_while(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("WHILE"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::WHILE(Box::new(expr), vec![], temp.len(), input.len() - temp.len())))
}

fn parse_make(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("MAKE"), space0)(input)?;
	let (temp, assign) = parse_assign(temp)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::MAKE(Box::new(assign), Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_penup(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("PENUP"), space0)(input)?;
	Ok((temp, Stmt::PENUP(temp.len(), input.len() - temp.len())))
}

fn parse_pendown(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("PENDOWN"), space0)(input)?;
	Ok((temp, Stmt::PENDOWN(temp.len(), input.len() - temp.len())))
}

fn parse_forward(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("FORWARD"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::FORWARD(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_back(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("BACK"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::BACK(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_left(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("LEFT"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::LEFT(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_right(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("RIGHT"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::RIGHT(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_setpencolor(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("SETPENCOLOR"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::SETPENCOLOR(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_turn(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("TURN"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::TURN(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_setheading(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("SETHEADING"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::SETHEADING(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_setx(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("SETX"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::SETX(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_sety(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("SETY"), space0)(input)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::SETY(Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_addassign(input: &str) -> IResult<&str, Stmt> {
	let (temp, _) = delimited(space0, tag("ADDASSIGN"), space0)(input)?;
	let (temp, assign) = parse_assign(temp)?;
	let (temp, expr) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 1)
	));
	Ok((temp, Stmt::ADDASSIGN(Box::new(assign), Box::new(expr), temp.len(), input.len() - temp.len())))
}

fn parse_func(input: &str) -> IResult<&str, Stmt> {
	let (mut temp, func_name) = parse_decl_name(input)?;
	let mut var = vec![];
	loop {
		match parse_expr(temp) {
			Ok((str, expr)) => {
				var.push(Box::new(expr));
				temp = str;
			},
			Err(_) => break
		}
	}

	Ok((temp, Stmt::FUNC(Box::new(func_name), var, temp.len(), input.len() - temp.len())))
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