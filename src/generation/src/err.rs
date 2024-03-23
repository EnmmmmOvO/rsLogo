use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("{error}")]
#[diagnostic(
    code("Code Generation Error"),
    help("{help}")
)]
pub enum GenerationError<'a> {
    UnexpectedExtraOperand {
        #[source_code]
        src: String,
        #[label("An unexpected extra operand appears that is not needed in the expression")]
        bad_bit: SourceSpan,
        help: &'a str,
        error: String,
    },
}