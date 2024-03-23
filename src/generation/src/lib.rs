use unsvg::Image;
use err::GenerationError;
use ast::structs::{Assign, Decl, DeclName, Stmt};
use draw::Draw;
use variable::Variable;
use function::Function;
use crate::stmt::precess_stmt;

mod err;
mod draw;
mod variable;
mod function;
mod stmt;
mod expr;

pub fn code_generation(ast: Vec<Decl>, width: u32, height: u32) -> Result<Image, GenerationError<'static>> {
	let mut image = Image::new(width, height);
	let mut draw = Draw::new(width as f32, height as f32, &mut image);
	let mut variable = Variable::new();
	let mut function = Function::new();

	let mut main: Vec::<Stmt> = Vec::new();

	for decl in ast {
		if decl.var.len() != 0 {
			decl.var.iter().for_each(|x| {
				match &*x.as_ref() {
					Assign::VAR(name, ..) => variable.insert(name.to_string(), None),
					_ => unreachable!()
				}
			});
		}
		match &*decl.name {
			DeclName::STRING(s, ..) => {
				if s == "" {
					main = decl.stmt_list;
				} else {
					function.insert(s.to_string(), decl.var, decl.stmt_list);
				}
			},
			_ => unreachable!()
		}
	}

	precess_stmt(&main, &mut variable, &mut draw, &function)?;

	println!("{:?}", variable);

	Ok(image)
}


