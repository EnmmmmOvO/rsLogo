use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::space0,
    sequence::delimited,
    IResult,
};
use crate::{
    structs::Decl,
    assign::parse_assign,
};
use crate::structs::DeclName;
use crate::support::check_name;

fn parse_func(input: &str) -> IResult<&str, Decl> {
    let (input, _) = delimited(space0, tag("TO"), space0)(input)?;
    let (input, name) = parse_decl_name(input).unwrap_or(
        (input, DeclName::ERROR("MissingName".to_string(), input.len(), 1))
    );
    let (input, expr) = parse_assign(input)?;


    Ok((input, Decl::FUNC1(Box::new(name), Box::new(expr), Vec::new())))
}

fn parse_func_without_input(input: &str) -> IResult<&str, Decl> {
    let (input, _) = delimited(space0, tag("TO"), space0)(input)?;
    let (input, name) = parse_decl_name(input).unwrap_or(
        (input, DeclName::ERROR("MissingName".to_string(), input.len(), 1))
    );

    Ok((input, Decl::FUNC0(Box::new(name), Vec::new())))
}

pub fn parse_decl(input: &str) -> IResult<&str, Decl> {
    alt((parse_func, parse_func_without_input))(input)
}

pub fn parse_decl_name(input: &str) -> IResult<&str, DeclName> {
    let (temp, s) = delimited(space0, is_not(" "), space0)(input)?;
    return if check_name(s) {
        Ok((temp, DeclName::STRING(s.to_string(), temp.len(), input.len() - temp.len())))
    } else {
        Ok((temp, DeclName::ERROR("InvalidName".to_string(), temp.len(), input.len() - temp.len())))
    }
}

