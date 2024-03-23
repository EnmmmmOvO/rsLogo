use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use crate::structs::{Assign, Decl, DeclName, Expr, Stmt};

#[derive(Error, Debug, Diagnostic)]
#[error("{error}")]
#[diagnostic(
    code("Compile Error"),
    help("{help}")
)]
pub enum ASTError<'a> {
    UnexpectedExtraOperand {
        #[source_code]
        src: String,
        #[label("An unexpected extra operand appears that is not needed in the expression")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    InvalidNumber {
        #[source_code]
        src: String,
        #[label("Could not convert to float")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnexpectedExpr {
        #[source_code]
        src: String,
        #[label("Could not convert to number, variable or system variable.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingLeftBracket {
        #[source_code]
        src: String,
        #[label("Expected `[`, found `...`")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingRightBracket {
        #[source_code]
        src: String,
        #[label("Expected `]`, found `...`")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingOperand {
        #[source_code]
        src: String,
        #[label("Expected Operand, found `...`")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnexpectedAssign {
        #[source_code]
        src: String,
        #[label("Unsatisfied the requirements of the assignment format")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingEnd {
        #[source_code]
        src: String,
        #[label("Expected `END`, found `...`")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingName {
        #[source_code]
        src: String,
        #[label("Missing the name of the function")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    InvalidName {
        #[source_code]
        src: String,
        #[label("Invalid format of the function or variable name")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingTo {
        #[source_code]
        src: String,
        #[label("Missing the `TO` Declaration above the `END`")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingWhileOrIF {
        #[source_code]
        src: String,
        #[label("Missing the `WHILE` or `IF` Condition Declaration above the `]`")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    DeclWrongPosition {
        #[source_code]
        src: String,
        #[label("Function declaration cannot be placed inside the `IF` or `WHILE` statement")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    RepeatFunctionName {
        #[source_code]
        src: String,
        #[label("Function name already exists")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
}

pub fn match_err<'a>(src: String, line: usize, err: String, end: usize, len: usize) -> ASTError<'a> {
    let start = src.len() - end - len;
    match err.as_str() {
        "UnexpectedExtraOperand" => ASTError::UnexpectedExtraOperand {
            src,
            bad_bit: (start, len).into(),
            help: "Check the format of each command and remove redundant operand",
            error: format!("Unexpected extra operands (Ln {line}, Col {})", start + 1),
        },
        "InvalidNum" => ASTError::InvalidNumber {
            src,
            bad_bit: (start, len).into(),
            help: "Try using a valid number",
            error: format!("Invalid number (Ln {line}, Col {})", start + 1),
        },
        "UnexpectedExpr" => ASTError::UnexpectedExpr {
            src,
            bad_bit: (start, len).into(),
            help: "If it is a variable, the format is `:{ variable_name }`. \nIf it is a number, \
            the format is `\"{ number }`. \nIf it is a system variable, only `XCOR`, \
            `YCOR`, `COLOR`, `HEADING` are allowed.",
            error: format!("Unexpected Expression (Ln {line}, Col {})", start + 1),
        },
        "MissingLeftBracket" => ASTError::MissingLeftBracket {
            src,
            bad_bit: (start, len).into(),
            help: "Add a left curly brace `[` to end of the `IF` or `WHILE` statement.",
            error: format!("Missing left curly brace (Ln {line}, Col {})", start + 1),
        },
        "MissingRightBracket" => ASTError::MissingRightBracket {
            src,
            bad_bit: (start, len).into(),
            help: "Add a right curly brace `]` in a new line to end the `IF` or `WHILE` statement.",
            error: format!("Missing right curly brace (Ln {line}, Col {})", start + 1),
        },
        "MissingOperand" => ASTError::MissingOperand {
            src,
            bad_bit: (start, len).into(),
            help: "Check the format of each command and Add the missing Operand.",
            error: format!("Missing Operand (Ln {line}, Col {})", start + 1),
        },
        "UnexpectedAssign" => ASTError::UnexpectedAssign {
            src,
            bad_bit: (start, len).into(),
            help: "Check the format of the variable name, the format is `\"{ variable_name }`.",
            error: format!("Unexpected Assignment format(Ln {line}, Col {})", start + 1),
        },
        "MissingEnd" => ASTError::MissingEnd {
            src,
            bad_bit: (start, len).into(),
            help: "Add `END` to end the function declaration.",
            error: format!("Missing END (Ln {line}, Col {})", start + 1),
        },
        "MissingName" => ASTError::MissingName {
            src,
            bad_bit: (start, len).into(),
            help: "Add a valid function name to the function declaration.",
            error: format!("Missing Function Name (Ln {line}, Col {})", start + 1),
        },
        "InvalidName" => ASTError::InvalidName {
            src,
            bad_bit: (start, len).into(),
            help: "Change a valid function or variable name which need follow by the `Rust` function or variable name rules.",
            error: format!("Invalid Function Name (Ln {line}, Col {})", start + 1),
        },
        "MissingTo" => ASTError::MissingTo {
            src,
            bad_bit: (start, len).into(),
            help: "Add `TO` to start the function declaration.",
            error: format!("Missing Function declaration (Ln {line}, Col {})", start + 1),
        },
        "MissingWhileOrIf" => ASTError::MissingWhileOrIF {
            src,
            bad_bit: (start, len).into(),
            help: "Add `IF` or `WHILE` to start the condition declaration.",
            error: format!("Missing `IF` or `WHILE` (Ln {line}, Col {})", start + 1),
        },
        "DeclWrongPosition" => ASTError::DeclWrongPosition {
            src,
            bad_bit: (start, len).into(),
            help: "Move the function declaration outside the `IF` or `WHILE` statement.",
            error: format!("Wrong Position of Function Declaration (Ln {line}, Col {})", start + 1),
        },
        "RepeatFunctionName" => ASTError::RepeatFunctionName {
            src,
            bad_bit: (start, len).into(),
            help: "Change the function name to a unique name.",
            error: format!("Repeat Function Name (Ln {line}, Col {})", start + 1),
        },
        _ => unreachable!(),
    }
}

pub fn check_decl_err(input: &Decl) -> Option<(String, usize, usize)> {
    match &*input.name {
        DeclName::ERROR(s, end, len) => return Some((s.to_string(), *end, *len)),
        _ => {},
    }

    for var in input.var.iter() {
        if let Assign::ERROR(s, end, len) = var.as_ref() {
            return Some((s.to_string(), *end, *len));
        }
    }
    None
}

pub fn check_decl_name_err(input: &DeclName) -> Option<(String, usize, usize)> {
    match input {
        DeclName::ERROR(s, end, len) => Some((s.to_string(), *end, *len)),
        _ => None,
    }
}

pub fn check_stmt_err(input: &Stmt) -> Option<(String, usize, usize)> {
    match input {
        Stmt::IF(expr, ..) | Stmt::WHILE(expr, ..) =>
            check_expr_err(expr.as_ref()),
        Stmt::MAKE(expr1, expr2, ..) |
        Stmt::ADDASSIGN(expr1, expr2, ..) => {
            if let Some(temp) = check_assign_err(expr1.as_ref()) {
                return Some(temp);
            }
            check_expr_err(expr2.as_ref())
        },
        Stmt::FORWARD(expr, ..) |
        Stmt::BACK(expr, ..) |
        Stmt::LEFT(expr, ..) |
        Stmt::RIGHT(expr, ..) |
        Stmt::SETPENCOLOR(expr, ..) |
        Stmt::TURN(expr, ..) |
        Stmt::SETHEADING(expr, ..) |
        Stmt::SETX(expr, ..) |
        Stmt::SETY(expr, ..) => check_expr_err(expr.as_ref()),
        Stmt::FUNC(name, assign, ..) => {
            if let Some(temp) = check_decl_name_err(name.as_ref()) {
                return Some(temp);
            }

            for var in assign.iter() {
                return check_expr_err(var.as_ref());
            }
            None
        },
        _ => None,
    }
}

pub fn check_assign_err(input: &Assign) -> Option<(String, usize, usize)> {
    match input {
        Assign::ERROR(s, end, len) => Some((s.to_string(), *end, *len)),
        _ => None,
    }
}

pub fn check_expr_err(input: &Expr) -> Option<(String, usize, usize)> {
    match input {
        Expr::ADD(a, b, ..) |
        Expr::SUB(a, b, ..) |
        Expr::MUL(a, b, ..) |
        Expr::DIV(a, b, ..) |
        Expr::EQ(a, b, ..) |
        Expr::NE(a, b, ..) |
        Expr::LT(a, b, ..) |
        Expr::GT(a, b, ..) |
        Expr::AND(a, b, ..) |
        Expr::OR(a, b, ..) => {
            let (a, b) = (a.as_ref(), b.as_ref());
            match (a, b) {
                (Expr::ERROR(s, end, len), _) => Some((s.to_string(), *end, *len)),
                (_, Expr::ERROR(s, end, len)) => Some((s.to_string(), *end, *len)),
                _ => None,
            }
        },
        Expr::ERROR(s, end, len) => Some((s.to_string(), *end, *len)),
        _ => None,
    }
}