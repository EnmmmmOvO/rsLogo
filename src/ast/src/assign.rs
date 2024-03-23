use nom::{
    bytes::complete::is_not,
    character::complete::{space0, char},
    sequence::delimited,
    IResult,
};
use nom::branch::alt;
use crate::structs::{Assign};
use crate::support::check_name;


pub fn parse_var(input: &str) -> IResult<&str, Assign> {
	let (temp, s) = delimited(space0, delimited(char('"'), is_not("\" "), space0), space0)(input)?;

	if check_name(s) {
		return Ok((temp, Assign::VAR(s.to_string(), temp.len(), input.len() - temp.len())));
	} else {
		return Ok((temp, Assign::ERROR("InvalidName".to_string(), temp.len(), input.len() - temp.len())));
	}
}

pub fn parse_error(input: &str) -> IResult<&str, Assign> {
	let (temp, _) = delimited(space0, is_not(" "), space0)(input)?;
	Ok((temp, Assign::ERROR("UnexpectedAssign".to_string(), temp.len(), input.len() - temp.len())))
}

pub fn parse_assign(input: &str) -> IResult<&str, Assign> {

	alt(( parse_var, parse_error, ))(input)
}