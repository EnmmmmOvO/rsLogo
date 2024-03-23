use unsvg::Image;
use err::GenerationError;
use ast::structs::Decl;
use draw::Draw;

mod err;
mod draw;

pub fn code_generation(ast: Vec<Decl>, width: u32, height: u32) -> Result<Image, GenerationError<'static>> {
	let mut image = Image::new(width, height);
	let mut draw = Draw::new(width as f32, height as f32, &mut image);






	Ok(image)
}

