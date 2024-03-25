use crate::ast::structs::Function;
use crate::transpiler::{
    file::{export_file, DrawMethod},
    func::transpile_func,
};

use miette::Result;
use std::path::PathBuf;

pub fn transpiler_rust(
    path: &PathBuf,
    ast: Function,
    file: &[String],
    width: u32,
    height: u32,
) -> Result<()> {
    let mut method = DrawMethod::new();
    let mut result = vec![
        "use crate::draw::Draw;".to_string(),
        "use miette::Result;\n".to_string(),
    ];

    for (name, func) in ast.get_all() {
        result.push(transpile_func(
            &func.args,
            &func.stmt_list,
            name,
            file,
            &mut method,
            &ast,
        )?);
    }

    export_file(path, &method, width, height, &result)?;

    Ok(())
}
