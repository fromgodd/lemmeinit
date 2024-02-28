use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};


#[derive(Debug)]
enum ProgrammingLanguage {
    Python,
    // todo add Julia, R, Cpp
}

// init command
fn init(lang: ProgrammingLanguage, project_name: &str, venv_name: Option<&str>) -> io::Result<()> {
    match lang {
        ProgrammingLanguage::Python => {
            // Create Python project directory
            let project_dir = Path::new(project_name);
            fs::create_dir(project_dir)?;

            // main.py object
            // todo create folder placeholder and take contents from that (example /placeholder/lang_name/main.file)
            let main_py_content = "def main():\n    pass\n";
            let main_py_file = project_dir.join("main.py");
            fs::write(&main_py_file, main_py_content)?;

            // if venv is provided, create venv
            if let Some(venv_name) = venv_name {
                // Create virtual environment using python -m venv
                let venv_command = format!("python -m venv {}", project_dir.join(venv_name).display());
                match Command::new("cmd").arg("/C").arg(&venv_command).status() {
                    Ok(status) => {
                        if status.success() {
                            println!("Python project '{}' initialized successfully.", project_name);
                        } else {
                            println!("Error: Failed to create virtual environment.");
                            exit(1);
                        }
                    }
                    Err(_) => {
                        println!("Error: Failed to execute virtual environment creation command.");
                        exit(1);
                    }
                }
            } else {
                println!("Python project '{}' initialized successfully.", project_name);
            }
        }
    }

    Ok(())
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: init <lang> <project_name> [--venvact <venv_folder_name>]");
        exit(1);
    }

    // parse args
    let lang_str = &args[0];
    let project_name = &args[1];

    // Parse venv argument --venvact
    let mut venv_name: Option<&str> = None;
    if let Some(index) = args.iter().position(|arg| arg == "--venvact") {
        if index + 1 < args.len() {
            venv_name = Some(&args[index + 1]);
        } else {
            println!("Error: Missing virtual environment name after --venvact.");
            exit(1);
        }
    }

    // Convert langs to enum of ProgrammingLanugage
    let lang = match lang_str.as_str() {
        "py" => ProgrammingLanguage::Python,
        _ => {
            println!("Unsupported programming language: {}", lang_str);
            exit(1);
        }
    };

    
    if let Err(err) = init(lang, project_name, venv_name) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}
