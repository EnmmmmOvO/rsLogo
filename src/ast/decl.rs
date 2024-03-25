use crate::ast::{
    assign::parse_assign,
    structs::{Assign, Decl, DeclName},
    support::check_name,
};
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::space0,
    sequence::delimited,
    IResult,
};

pub fn parse_decl(input: &str) -> IResult<&str, Decl> {
    let (input, _) = delimited(space0, tag("TO"), space0)(input)?;
    let (input, name) = parse_decl_name(input).unwrap_or((
        input,
        DeclName::Error("MissingName".to_string(), input.len(), 1),
    ));
    let mut var: Vec<Assign> = Vec::new();
    let mut temp = input;

    while let Ok((str, assign)) = parse_assign(temp) {
        var.push(assign);
        temp = str;
    }

    Ok((
        temp,
        Decl {
            name: Box::new(name),
            var,
        },
    ))
}

pub fn parse_decl_name(input: &str) -> IResult<&str, DeclName> {
    let (temp, s) = delimited(space0, is_not(" "), space0)(input)?;
    if check_name(s) {
        Ok((
            temp,
            DeclName::String(s.to_string(), temp.len(), input.len() - temp.len()),
        ))
    } else {
        Ok((
            temp,
            DeclName::Error(
                "InvalidName".to_string(),
                temp.len(),
                input.len() - temp.len(),
            ),
        ))
    }
}
