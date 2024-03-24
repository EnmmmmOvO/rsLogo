mod file;
mod func;
mod stmt;
mod expr;
mod err;

use std::path::PathBuf;
use ast::structs::Function;
use file::DrawMethod;
use miette::Result;
use crate::file::export_file;
use crate::func::{transpile_func};

pub fn transpiler_rust<'a>(
    path: &'a PathBuf,
    ast: Function,
    file: &'a Vec<String>,
    width: u32,
    height: u32
) -> Result<()> {
    let mut method = DrawMethod::new();
    let mut result = vec![
        "use crate::draw::Draw;".to_string(),
        "use miette::Result;\n".to_string(),
    ];

    for (name, func) in ast.get_all() {
        result.push(transpile_func(&func.args, &func.stmt_list, name, file, &mut method, &ast)?);
    }

    export_file(path, &method, width, height, &result)?;

    Ok(())
}
