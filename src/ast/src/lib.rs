use std::collections::VecDeque;
use std::path::PathBuf;

pub mod structs;
mod decl;
mod stmt;
mod expr;
mod assign;
mod err;
mod support;

use structs::{Stmt, Decl};
use stmt::parse_stmt;
use decl::parse_decl;
use err::ASTError;
use crate::err::{check_decl_err, check_stmt_err, match_err};
use crate::structs::DeclName;


fn parse_file(path: PathBuf) -> Vec<String> {
    let file = std::fs::read_to_string(path).expect("Failed to read file");
    file.lines().map(|x| x.to_string()).collect()
}

pub fn parse_ast(path: PathBuf) -> Result<Vec<Decl>, ASTError<'static>> {
    let mut result: Vec<Decl> = Vec::new();

    let mut record: VecDeque<Stmt> = VecDeque::new();
    let mut local: VecDeque<Vec<Stmt>> = VecDeque::new();

    let mut func_record: Option<Decl> = None;
    local.push_front(Vec::new());

    for (idx, sentence) in parse_file(path).iter().enumerate() {
        let trim_sentence = sentence.trim();

        if trim_sentence.is_empty() {
            continue;
        }

        if trim_sentence.starts_with("//") {
            local.front_mut().unwrap().push(
                Stmt::COMMENT(sentence.trim().trim_start_matches("//").trim().to_string())
            );
        } else if trim_sentence == "]" {
            if local.len() == 1 {
                return Err(match_err(
                    sentence.to_string(),
                    idx + 1,
                    "MissingWhileOrIf".to_string(),
                    0,
                    1
                ))
            }

            let block = local.pop_front().expect("Expected a block in local");
            let stmt_to_modify = record.pop_front().expect("Expected a statement to modify in record");

            match stmt_to_modify {
                Stmt::IF(expr, _, end, len) => {
                    let modified_stmt = Stmt::IF(expr, block, end, len);
                    local.front_mut().expect("Expected a front block in local").push(modified_stmt);
                },
                Stmt::WHILE(expr, _, end, len) => {
                    let modified_stmt = Stmt::WHILE(expr, block, end, len);
                    local.front_mut().expect("Expected a front block in local").push(modified_stmt);
                },
                _ => unreachable!(),
            }
        } else if trim_sentence.starts_with("TO") {
            let result = parse_decl(&sentence).expect("Failed to parse declaration");
            if !result.0.is_empty() {
                return Err(match_err(
                    sentence.to_string(),
                    idx + 1,
                    "UnexpectedExtraOperand".to_string(),
                    0,
                    result.0.len()
                ))
            }

            if let Some((err, end, len)) = check_decl_err(&result.1) {
                return Err(match_err(sentence.to_string(), idx + 1, err, end, len))
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
                    sentence.len()
                ))
            }

            let mut decl = func_record.take().expect("Expected a declaration");
            decl.stmt_list = local.pop_front().expect("Expected a block in local");
            result.push(decl);
        } else {
            let result = parse_stmt(&sentence).expect("Expected a block in local");

            if trim_sentence.starts_with("IF") || trim_sentence.starts_with("WHILE") {
                if result.0 != "[" {
                    return if result.0.is_empty() || !result.0.ends_with('[') {
                        Err(match_err(
                            sentence.to_string(),
                            idx + 1,
                            "MissingLeftBracket".to_string(), 0, 1
                        ))
                    } else {
                        Err(match_err(
                            sentence.to_string(),
                            idx + 1,
                            "UnexpectedExtraOperand".to_string(),
                            1,
                            result.0.len() - 1
                        ))
                    }
                }

                if let Some((err, end, len)) = check_stmt_err(&result.1) {
                    return Err(match_err(sentence.to_string(), idx + 1, err, end, len))
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
                        result.0.len()
                    ))
                }

                if let Some((err, end, len)) = check_stmt_err(&result.1) {
                    return Err(match_err(sentence.to_string(), idx + 1, err, end, len))
                }

                local.front_mut().expect("Expected a front block in local").push(result.1);
            }
        }
    }

    result.push(Decl {
        name: Box::new(DeclName::STRING("".to_string(), 0, 0)),
        var: Vec::new(),
        stmt_list: local.pop_front().expect("Expected a block in local")
    });

    if func_record.is_some() {
        return Err(match_err(
            " ".to_string(),
            0,
            "MissingEnd".to_string(),
            0,
            1,
        ))
    }

    if !local.is_empty() {
        return Err(match_err(
            " ".to_string(),
            0,
            "MissingRightBracket".to_string(),
            0,
            1,
        ))
    }
    Ok(result)
}
