use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

pub fn new(args: Vec<String>) -> io::Result<()> {
    if args.len() < 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Project name is required",
        ));
    }

    let project_name = &args[2];
    let project_dir = format!("./{}", project_name);

    let path = PathBuf::from(&project_dir);
    let path2 = PathBuf::from(format!("{}/src", &project_dir));

    if path.exists() {
        println!("Folder already exists. Please consider being original or delete the old folder.");
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Directory already exists",
        ));
    }

    fs::create_dir_all(&path2)?;

    let config_file_path = PathBuf::from(format!("{}/fiskur.toml", &project_dir));
    let main_file_path = PathBuf::from(format!("{}/src/main.kty", &project_dir));

    let mut config_file = File::create(config_file_path)?;
    let mut main_file = File::create(main_file_path)?;

    let config_content = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2025\"",
        project_name
    );
    config_file.write_all(config_content.as_bytes())?;

    let main_content = b"fn main() {\n\tchirp(\"Hello World!!!\");\n}";
    main_file.write_all(main_content)?;

    println!("Project '{}' created successfully!", project_name);

    Ok(())
}
