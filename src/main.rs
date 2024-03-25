use clap::Parser;

use ast::parse_ast;
use generation::code_generation;
use miette::{miette, Result};
use transpiler::transpiler_rust;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let file = match std::fs::read_to_string(file_path) {
        Ok(file) => file.lines().map(|x| x.to_string()).collect::<Vec<String>>(),
        Err(e) => return Err(miette!(e)),
    };

    let ast = parse_ast(&file)?;

    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let image = code_generation(ast, &file, width, height)?;

            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                return Err(miette!("Error saving svg: {e}"));
            }
        }
        Some("png") => {
            let image = code_generation(ast, &file, width, height)?;

            let res = image.save_png(&image_path);
            if let Err(e) = res {
                return Err(miette!("Error saving png: {e}"));
            }
        }
        None => {
            if image_path.is_dir() || image_path.display().to_string().contains('/') {
                return Err(miette!("Rust transpiler does not support directories"));
            }
            transpiler_rust(&image_path, ast, &file, width, height)?
        }
        _ => return Err(miette!("File extension not supported")),
    }

    Ok(())
}
