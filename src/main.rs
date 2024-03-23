use clap::Parser;

use miette::{Diagnostic, miette, Result};
use ast::parse_ast;
use generation::code_generation;


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

    let ast = parse_ast(file_path)?;

    match image_path.extension().map(|s| s.to_str()).flatten() {
        Some("svg") => {
            let image = code_generation(ast, width, height)?;

            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                return Err(miette!("Error saving svg: {e}"));
            }
        }
        Some("png") => {
            let image = code_generation(ast, width, height)?;

            let res = image.save_png(&image_path);
            if let Err(e) = res {
                return Err(miette!("Error saving png: {e}"));
            }
        }
        Some("rs") => {
            println!("Rust file detected");
        }
        _ => {
            code_generation(ast, width, height)?;
            // return Err(miette!("File extension not supported"));
        }
    }

    Ok(())
}
