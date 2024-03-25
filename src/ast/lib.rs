use std::collections::VecDeque;

use crate::ast::{
    decl::parse_decl,
    err::ASTError,
    err::{check_decl_err, check_stmt_err, match_err},
    stmt::parse_stmt,
    structs::{Decl, DeclName, Function, Stmt},
};

fn struct_check(file: &[String]) -> Result<(), ASTError<'static>> {
    let mut if_while_list = vec![];
    let mut to = 0_usize;
    for (idx, sentence) in file.iter().enumerate() {
        let str = sentence.trim();
        if str.starts_with("IF") || str.starts_with("WHILE") {
            if !str.ends_with('[') {
                return Err(match_err(
                    format!("{}  ", sentence.trim_end()),
                    idx + 1,
                    "MissingLeftBracket".to_string(),
                    0,
                    1,
                ));
            }
            if_while_list.push(idx);
        } else if str.starts_with(']') {
            if if_while_list.is_empty() {
                return Err(match_err(
                    format!(" \n{}", sentence),
                    idx + 1,
                    "MissingWhileOrIf".to_string(),
                    sentence.len() + 1,
                    1,
                ));
            }
            if_while_list.pop();
        } else if str.starts_with("TO") {
            if to != 0 {
                return Err(match_err(
                    format!("{}\n ", file[to - 1]),
                    idx + 1,
                    "MissingEnd".to_string(),
                    0,
                    1,
                ));
            } else if !if_while_list.is_empty() {
                return Err(match_err(
                    format!("{}\n{}", file[if_while_list.pop().unwrap()], sentence),
                    idx + 1,
                    "DeclWrongPosition".to_string(),
                    0,
                    sentence.len(),
                ));
            }
            to = idx + 1;
        } else if str.starts_with("END") {
            if to != 0 && !if_while_list.is_empty() {
                return Err(match_err(
                    format!("{}\n ", file[if_while_list.pop().unwrap()]),
                    idx + 1,
                    "MissingRightBracket".to_string(),
                    0,
                    1,
                ));
            } else if to == 0 && !if_while_list.is_empty() {
                return Err(match_err(
                    format!("{}\n{}", file[if_while_list.pop().unwrap()], sentence),
                    idx + 1,
                    "DeclWrongPosition".to_string(),
                    0,
                    sentence.len(),
                ));
            } else if to == 0 {
                return Err(match_err(
                    format!(" \n{}", sentence),
                    idx + 1,
                    "MissingTo".to_string(),
                    sentence.len() + 1,
                    1,
                ));
            }
            to = 0;
        }
    }

    if to != 0 {
        return Err(match_err(
            format!("{}\n ", file[to - 1]),
            0,
            "MissingEnd".to_string(),
            0,
            1,
        ));
    }

    if !if_while_list.is_empty() {
        return Err(match_err(
            format!("{}\n ", file[if_while_list.pop().unwrap()]),
            0,
            "MissingRightBracket".to_string(),
            0,
            1,
        ));
    }
    Ok(())
}

pub fn parse_ast(file: &[String]) -> Result<Function, ASTError<'static>> {
    struct_check(file)?;

    let mut res = Function::new();

    let mut record: VecDeque<Stmt> = VecDeque::new();
    let mut local: VecDeque<Vec<Stmt>> = VecDeque::new();

    let mut func_record: Option<Decl> = None;
    local.push_front(Vec::new());

    for (idx, sentence) in file.iter().enumerate() {
        let trim_sentence = sentence.trim();

        if trim_sentence.is_empty() {
            continue;
        }

        if trim_sentence.starts_with("//") {
            local.front_mut().unwrap().push(Stmt::Comments(
                sentence.trim().trim_start_matches("//").trim().to_string(),
                idx + 1,
            ));
        } else if trim_sentence == "]" {
            if local.len() == 1 {
                return Err(match_err(
                    sentence.to_string(),
                    idx + 1,
                    "MissingWhileOrIf".to_string(),
                    0,
                    1,
                ));
            }

            let block = local.pop_front().expect("Expected a block in local");
            let stmt_to_modify = record
                .pop_front()
                .expect("Expected a statement to modify in record");

            match stmt_to_modify {
                Stmt::If(expr, _, line) => {
                    let modified_stmt = Stmt::If(expr, block, line);
                    local
                        .front_mut()
                        .expect("Expected a front block in local")
                        .push(modified_stmt);
                }
                Stmt::While(expr, _, line) => {
                    let modified_stmt = Stmt::While(expr, block, line);
                    local
                        .front_mut()
                        .expect("Expected a front block in local")
                        .push(modified_stmt);
                }
                _ => unreachable!(),
            }
        } else if trim_sentence.starts_with("TO") {
            let result = parse_decl(sentence).expect("Failed to parse declaration");
            if !result.0.is_empty() {
                return Err(match_err(
                    sentence.to_string(),
                    idx + 1,
                    "UnexpectedExtraOperand".to_string(),
                    0,
                    result.0.len(),
                ));
            }

            if let DeclName::String(name, end, len) = &*result.1.name {
                if res.check_name(name) {
                    return Err(match_err(
                        sentence.to_string(),
                        idx + 1,
                        "RepeatFunctionName".to_string(),
                        *end,
                        *len,
                    ));
                }
            }

            if let Some((err, end, len)) = check_decl_err(&result.1) {
                return Err(match_err(sentence.to_string(), idx + 1, err, end, len));
            }

            func_record = Some(result.1);
            local.push_front(Vec::new());
        } else if trim_sentence == "END" {
            if func_record.is_none() {
                return Err(match_err(
                    sentence.to_string(),
                    idx + 1,
                    "MissingTo".to_string(),
                    0,
                    sentence.len(),
                ));
            }

            let decl = func_record.take().expect("Expected a declaration");

            if let DeclName::String(name, _, _) = &*decl.name {
                res.insert(
                    name.to_string(),
                    decl.var,
                    local.pop_front().expect("Expected a block in local"),
                );
            }
        } else {
            let mut result = parse_stmt(sentence).expect("Expected a block in local");

            match &mut result.1 {
                Stmt::If(.., line)
                | Stmt::While(.., line)
                | Stmt::Make(.., line)
                | Stmt::PenUp(line)
                | Stmt::PenDown(line)
                | Stmt::Forward(.., line)
                | Stmt::Back(.., line)
                | Stmt::Left(.., line)
                | Stmt::Right(.., line)
                | Stmt::SetPenColor(.., line)
                | Stmt::Turn(.., line)
                | Stmt::SetHeading(.., line)
                | Stmt::SetX(.., line)
                | Stmt::SetY(.., line)
                | Stmt::AddAssign(.., line)
                | Stmt::Func(.., line) => *line = idx,
                _ => (),
            }

            if trim_sentence.starts_with("IF") || trim_sentence.starts_with("WHILE") {
                if result.0 != "[" {
                    return if result.0.is_empty() || !result.0.ends_with('[') {
                        Err(match_err(
                            sentence.to_string(),
                            idx + 1,
                            "MissingLeftBracket".to_string(),
                            0,
                            1,
                        ))
                    } else {
                        Err(match_err(
                            sentence.to_string(),
                            idx + 1,
                            "UnexpectedExtraOperand".to_string(),
                            1,
                            result.0.len() - 1,
                        ))
                    };
                }

                if let Some((err, end, len)) = check_stmt_err(&result.1) {
                    return Err(match_err(sentence.to_string(), idx + 1, err, end, len));
                }

                record.push_front(result.1);
                local.push_front(Vec::new());
            } else {
                if !result.0.is_empty() {
                    return Err(match_err(
                        sentence.to_string(),
                        idx + 1,
                        "UnexpectedExtraOperand".to_string(),
                        0,
                        result.0.len(),
                    ));
                }

                if let Some((err, end, len)) = check_stmt_err(&result.1) {
                    return Err(match_err(sentence.to_string(), idx + 1, err, end, len));
                }

                local
                    .front_mut()
                    .expect("Expected a front block in local")
                    .push(result.1);
            }
        }
    }

    res.insert(
        "".to_string(),
        Vec::new(),
        local.pop_front().expect("Expected a block in local"),
    );

    if func_record.is_some() {
        return Err(match_err(
            " ".to_string(),
            0,
            "MissingEnd".to_string(),
            0,
            1,
        ));
    }

    if !local.is_empty() {
        return Err(match_err(
            " ".to_string(),
            0,
            "MissingRightBracket".to_string(),
            0,
            1,
        ));
    }
    Ok(res)
}
