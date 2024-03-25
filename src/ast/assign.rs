use crate::ast::{structs::Assign, support::check_name};
use nom::branch::alt;
use nom::{
    bytes::complete::is_not,
    character::complete::{char, space0},
    sequence::delimited,
    IResult,
};

pub fn parse_var(input: &str) -> IResult<&str, Assign> {
    let (temp, s) = delimited(space0, delimited(char('"'), is_not("\" "), space0), space0)(input)?;

    if check_name(s) {
        Ok((
            temp,
            Assign::Var(s.to_string(), temp.len(), input.len() - temp.len()),
        ))
    } else {
        Ok((
            temp,
            Assign::Error(
                "InvalidName".to_string(),
                temp.len(),
                input.len() - temp.len(),
            ),
        ))
    }
}

pub fn parse_error(input: &str) -> IResult<&str, Assign> {
    let (temp, _) = delimited(space0, is_not(" "), space0)(input)?;
    Ok((
        temp,
        Assign::Error(
            "UnexpectedAssign".to_string(),
            temp.len(),
            input.len() - temp.len(),
        ),
    ))
}

pub fn parse_assign(input: &str) -> IResult<&str, Assign> {
    alt((parse_var, parse_error))(input)
}
