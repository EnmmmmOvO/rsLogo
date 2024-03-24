use unsvg::Image;
use err::GenerationError;
use ast::structs::Function;
use draw::Draw;
use variable::Variable;
use crate::stmt::process_stmt;

mod err;
mod draw;
mod variable;
mod stmt;
mod expr;

pub fn code_generation(ast: Function, file: &Vec<String>, width: u32, height: u32) -> Result<Image, GenerationError<'static>> {
	let mut image = Image::new(width, height);
	let mut draw = Draw::new(width as f32, height as f32, &mut image);
	let mut variable = Variable::new();

	ast.get_args().iter().for_each(|arg| { variable.insert_num(arg.to_string(), None) });
	process_stmt(ast.get_main(), &mut variable, &mut draw, &ast, file)?;

	Ok(image)
}


