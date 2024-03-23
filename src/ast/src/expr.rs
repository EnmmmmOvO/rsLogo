use nom::{
	branch::alt,
    bytes::complete::{tag, is_not, take_while1},
    character::complete::{space0, char},
    sequence::delimited,
    IResult,
};
use crate::structs::Expr;
use crate::support::check_name;

fn parse_num(input: &str) -> IResult<&str, Expr> {
	let (temp, num) = delimited(
		space0, delimited(
			char('"'),
			take_while1(|c: char| c.is_digit(10) || c == '.' || c == '-'),
			space0
		), space0)(input)?;

	match num.parse::<f32>() {
		Ok(f) => Ok((temp, Expr::FLOAT(f, temp.len(), input.len() - temp.len()))),
		Err(_) => Ok((temp, Expr::ERROR("InvalidNum".to_string(), temp.len(), input.len() - temp.len()))),
	}
}

fn parse_var(input: &str) -> IResult<&str, Expr> {
	let (temp, s) = delimited(space0, delimited(char(':'), is_not(": "), space0), space0)(input)?;

	if check_name(s) {
		return Ok((temp, Expr::VAR(s.to_string(), temp.len(), input.len() - temp.len())));
	} else {
		return Ok((temp, Expr::ERROR("InvalidName".to_string(), temp.len(), input.len() - temp.len())));
	}
}

fn parse_add(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, char('+'), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::ADD(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_sub(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, char('-'), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::SUB(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_mul(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, char('*'), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::MUL(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_div(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, char('/'), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::DIV(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_eq(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("EQ"), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::EQ(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_ne(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("NE"), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::NE(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_lt(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("LT"), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::LT(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_gt(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("GT"), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::GT(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_and(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("AND"), space0)(input)?;
	let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
	Ok((temp, Expr::AND(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_or(input: &str) -> IResult<&str, Expr> {
    let (temp, _) = delimited(space0, tag("OR"), space0)(input)?;
    let (temp, left) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
    let (temp, right) = parse_expr(temp).unwrap_or(
		(temp, Expr::ERROR("MissingOperand".to_string(), temp.len(), 0))
	);
    Ok((temp, Expr::OR(Box::new(left), Box::new(right), temp.len(), input.len() - temp.len())))
}

fn parse_xcor(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("XCOR"), space0)(input)?;
	Ok((temp, Expr::XCOR(temp.len(), input.len() - temp.len())))
}

fn parse_ycor(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("YCOR"), space0)(input)?;
	Ok((temp, Expr::YCOR(temp.len(), input.len() - temp.len())))
}

fn parse_heading(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("HEADING"), space0)(input)?;
	Ok((temp, Expr::HEADING(temp.len(), input.len() - temp.len())))
}

fn parse_color(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, tag("COLOR"), space0)(input)?;
	Ok((temp, Expr::COLOR(temp.len(), input.len() - temp.len())))
}

fn parts_err(input: &str) -> IResult<&str, Expr> {
	let (temp, _) = delimited(space0, is_not(" ["), space0)(input)?;
	Ok((temp, Expr::ERROR("UnexpectedExpr".to_string(), temp.len(), input.len() - temp.len())))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
	alt((
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