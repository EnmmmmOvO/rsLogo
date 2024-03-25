use crate::ast::structs::{Assign, Function, Stmt};
use crate::transpiler::{err::TranspilerError, file::DrawMethod, stmt::transpiler_stmt};
use std::collections::HashMap;

pub fn transpile_func<'a>(
    args: &[Assign],
    stmt_list: &Vec<Stmt>,
    name: &str,
    file: &[String],
    method: &mut DrawMethod,
    ast: &Function,
) -> Result<String, TranspilerError<'a>> {
    if name.is_empty() {
        Ok(format!(
            "pub fn process_svg(draw: &mut Draw) -> Result<()> {{\n{}\tOk(())\n}}\n",
            transpiler_stmt(stmt_list, file, method, &mut HashMap::new(), 1, ast)?
        ))
    } else {
        let args: Vec<String> = args
            .iter()
            .map(|x| {
                if let Assign::Var(name, ..) = &x {
                    format!("{}: f32", name)
                } else {
                    unreachable!();
                }
            })
            .collect();

        Ok(format!(
            "fn {}(draw: &mut Draw, {}) -> Result<()> {{\n{}\tOk(())\n}}\n",
            name,
            args.join(", "),
            transpiler_stmt(
                stmt_list,
                file,
                method,
                &mut ast.get_args_by_name(name),
                1,
                ast
            )?
        ))
    }
}
