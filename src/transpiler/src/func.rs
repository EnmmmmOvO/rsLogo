use std::collections::HashSet;
use ast::structs::{Function, Stmt};
use crate::err::TranspilerError;
use ast::structs::Assign;
use crate::file::DrawMethod;
use crate::stmt::transpiler_stmt;

pub fn transpile_func<'a>(
	args: &Vec<Box<Assign>>,
	stmt_list: &Vec<Stmt>,
	name: &str,
	file: &Vec<String>,
	method: &mut DrawMethod,
	ast: &Function,
) -> Result<String, TranspilerError<'a>> {
	if name == "" {
		Ok(format!(
			"pub fn process_svg(draw: &mut Draw) -> Result<()> {{\n{}\tOk(())\n}}\n",
			transpiler_stmt(stmt_list, file, method, &mut HashSet::new(), 1, ast)?
		))
	} else {
		let args: Vec<String> = args.iter().map(|x|
			if let Assign::VAR(name, ..) = x.as_ref() {
				format!("{}: f32", name)
			} else {
				unreachable!();
			}
		).collect();

		Ok(format!(
			"fn {}({}, draw: &mut Draw) -> Result<()> {{\n{}\tOk(())\n}}\n",
			name,
			args.join(", "),
			transpiler_stmt(stmt_list, file, method, &mut ast.get_args_by_name(name), 1, ast)?
		))
	}
}