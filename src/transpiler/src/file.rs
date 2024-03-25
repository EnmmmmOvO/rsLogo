use miette::{IntoDiagnostic, Result};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct DrawMethod {
    pub set: HashSet<String>,
}

impl DrawMethod {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    pub fn insert(&mut self, key: String) {
        self.set.insert(key);
    }

    pub fn keys(&self) -> &HashSet<String> {
        &self.set
    }
}

fn read_file_to_string(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).into_diagnostic()?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).into_diagnostic()?;
    Ok(contents)
}

fn export_draw_file(map: &DrawMethod, path: &Path) -> Result<()> {
    let mut draw = read_file_to_string("src/transpiler/template/draw.rs.template")?;

    let mut list = vec![read_file_to_string(
        "src/transpiler/template/draw_impl/new.template",
    )?];

    for key in map.keys() {
        list.push(read_file_to_string(&format!(
            "src/transpiler/template/draw_impl/{key}.template"
        ))?);
    }

    draw.push_str(&format!("\nimpl<'a> Draw<'a> {{\n{}}}", list.join("\n")));

    File::create(format!("{}/src/draw.rs", path.display()))
        .into_diagnostic()?
        .write_all(draw.as_bytes())
        .into_diagnostic()?;

    Ok(())
}

pub fn export_main_file(path: &Path, width: u32, height: u32) -> Result<()> {
    let main = read_file_to_string("src/transpiler/template/main.rs.template")?
        .replace("{WIDTH}", &width.to_string())
        .replace("{HEIGHT}", &height.to_string())
        .replace("{FILENAME}", &path.display().to_string());

    File::create(format!("{}/src/main.rs", path.display()))
        .into_diagnostic()?
        .write_all(main.as_bytes())
        .into_diagnostic()?;

    Ok(())
}

pub fn export_cargo_file(path: &Path) -> Result<()> {
    let main = read_file_to_string("src/transpiler/template/Cargo.toml.template")?
        .replace("{FILENAME}", &path.display().to_string());

    File::create(format!("{}/Cargo.toml", path.display()))
        .into_diagnostic()?
        .write_all(main.as_bytes())
        .into_diagnostic()?;

    Ok(())
}

pub fn export_file(
    path: &PathBuf,
    map: &DrawMethod,
    width: u32,
    height: u32,
    result: &[String],
) -> Result<()> {
    fs::create_dir(path).into_diagnostic()?;

    export_cargo_file(path)?;

    fs::create_dir(format!("{}/src", path.display())).into_diagnostic()?;

    export_main_file(path, width, height)?;
    export_draw_file(map, path)?;

    File::create(format!("{}/src/process.rs", path.display()))
        .into_diagnostic()?
        .write_all(result.join("\n").as_bytes())
        .into_diagnostic()?;

    Ok(())
}
