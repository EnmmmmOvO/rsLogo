use unsvg::Image;
use err::GenerationError;
use ast::structs::{Decl, DeclName};
use draw::Draw;
use variable::Variable;

mod err;
mod draw;
mod variable;
mod function;

pub fn code_generation(ast: Vec<Decl>, width: u32, height: u32) -> Result<Image, GenerationError<'static>> {
	let mut image = Image::new(width, height);
	let mut draw = Draw::new(width as f32, height as f32, &mut image);
	let mut variable = Variable::new();

	let mut main: Decl;

	for decl in ast {

	}

	Ok(image)
}

