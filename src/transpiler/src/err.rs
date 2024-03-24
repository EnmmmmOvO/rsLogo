use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("{error}")]
#[diagnostic(
    code("Transpiler Error"),
    help("{help}")
)]
pub enum TranspilerError<'a> {
    UnexpectedNumberType {
        #[source_code]
        src: String,
        #[label("Expected a `boolean` expression, found a `numeric` or `variable` expression.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnexpectedBooleanType {
        #[source_code]
        src: String,
        #[label("Expected a `numeric` or `variable` expression, found a `boolean` expression.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    DivideByZero {
        #[source_code]
        src: String,
        #[label("Cannot divide by zero.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnmatchedExprType {
        #[source_code]
        src: String,
        #[label("Cannot compare type 'boolean' with type 'number'")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    NonIntegerValueError {
        #[source_code]
        src: String,
        #[label("Expected an integer value, found a float value with a decimal.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnDefinedColor {
        #[source_code]
        src: String,
        #[label("Color range only pick integer value from 0 to 15")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnDefinedVariable {
        #[source_code]
        src: String,
        #[label("Variable is not defined.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnDefinedVariableValue {
        #[source_code]
        src: String,
        #[label("Variable value is not defined.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    UnDefinedFunction {
        #[source_code]
        src: String,
        #[label("Function is not defined.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    TooManyArguments {
        #[source_code]
        src: String,
        #[label("Too many arguments.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
    MissingArguments {
        #[source_code]
        src: String,
        #[label("Missing arguments in function call.")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
}

pub fn match_err<'a>(src: String, line: usize, err: String, end: usize, len: usize) -> TranspilerError<'a> {
    let start = src.len() - end - len;
    match err.as_str() {
        "UnexpectedNumberType" => TranspilerError::UnexpectedNumberType {
            src,
            bad_bit: (start, len).into(),
            help: "Replace with an expression that returns a boolean, choosing from `EQ`, \
            `NE`, `GT`, `LT`, `AND`, or `OR`.",
            error: format!(
                "Unexpected `number` or `variable` type in expression (Ln {line}, Col {})",
                start + 1
            ),
        },
        "UnexpectedBooleanType" => TranspilerError::UnexpectedBooleanType {
            src,
            bad_bit: (start, len).into(),
            help: "Replace with an expression that returns a number, choosing from `FLOAT`, \
            `VAR`, `ADD`, `SUB`, `MUL`, `DIV`, `XCOR`, `YCOR`, `HEADING`, or `COLOR`.",
            error: format!(
                "Unexpected `boolean` type in expression (Ln {line}, Col {})",
                start + 1
            ),
        },
        "DivideByZero" => TranspilerError::DivideByZero {
            src,
            bad_bit: (start, len).into(),
            help: "Change the divisor to a non-zero value or check the variable value.",
            error: format!("Divide by zero error (Ln {line}, Col {})", start + 1),
        },
        "UnmatchedExprType" => TranspilerError::UnmatchedExprType {
            src,
            bad_bit: (start, len).into(),
            help: "Change the expression to match the other expression type.",
            error: format!("Unmatched expression type (Ln {line}, Col {})", start + 1),
        },
        "NonIntegerValueError" => TranspilerError::NonIntegerValueError {
            src,
            bad_bit: (start, len).into(),
            help: "Change the value to an integer value or check the variable value.",
            error: format!("Non-integer value error (Ln {line}, Col {})", start + 1),
        },
        "UnDefinedColor" => TranspilerError::UnDefinedColor {
            src,
            bad_bit: (start, len).into(),
            help: "Change the color value to an integer value from 0 to 15.",
            error: format!("Undefined color error (Ln {line}, Col {})", start + 1),
        },
        "UnDefinedVariable" => TranspilerError::UnDefinedVariable {
            src,
            bad_bit: (start, len).into(),
            help: "Use `MAKE` to define the variable before using it.",
            error: format!("Undefined variable error (Ln {line}, Col {})", start + 1),
        },
        "UnDefinedVariableValue" => {
            TranspilerError::UnDefinedVariableValue {
                src,
                bad_bit: (start, len).into(),
                help: "This value may be defined in the arguments of `TO`, but its specific value \
                has not been defined yet and cannot be used.",
                error: format!("Undefined variable value error (Ln {line}, Col {})", start + 1),
            }
        },
        "UnDefinedFunction" => TranspilerError::UnDefinedFunction {
            src,
            bad_bit: (start, len).into(),
            help: "Define the `TO` function before using it.",
            error: format!("Undefined function error (Ln {line}, Col {})", start + 1),
        },
        "TooManyArguments" => TranspilerError::TooManyArguments {
            src,
            bad_bit: (start, len).into(),
            help: "Remove the extra arguments from the function call.",
            error: format!("Too many arguments error (Ln {line}, Col {})", start + 1),
        },
        "MissingArguments" => TranspilerError::MissingArguments {
            src,
            bad_bit: (start, len).into(),
            help: "Add the missing arguments to the function call.",
            error: format!("Missing arguments error (Ln {line}, Col {})", start + 1),
        },
        _ => unreachable!(),
    }
}