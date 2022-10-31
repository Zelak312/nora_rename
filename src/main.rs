mod ast;
mod errors;
mod lib;
mod tokenizer;
mod utils;

use std::{
    collections::HashMap,
    fs::{read_dir, rename},
    io,
    process::exit,
    rc::Rc,
};

use ast::nodes::ExecutableNode;
use clap::Parser;
use errors::Error;
use regex::Regex;

use crate::{
    ast::{interpreter::Interpreter, parser},
    tokenizer::lexer,
};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    input: String,

    #[clap(value_parser)]
    output: String,

    /// Skip the preview (useful in scripts)
    #[clap(short, long)]
    skip: bool,

    /// Pretty_print the output
    #[clap(short, long)]
    pretty_print: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let regex = Regex::new(&cli.input).expect("Invalid regex");
    let path = "./";

    let mut lex = lexer::Lexer::new(cli.output.clone());
    let tokens = lex.tokenize();

    let mut tree = parser::Parser::new(tokens, cli.output.clone());
    let node_result = tree.parse();
    if let Err(e) = node_result {
        println!("{}", e.message());
        exit(1);
    }

    let node = node_result.unwrap();
    let file_rename = run_interpreter(path, &regex, &node);
    if file_rename.len() == 0 {
        println!("No files to rename, exiting");
        exit(1);
    }

    // Detect duplicates
    if find_duplicates(&file_rename) {
        println!("Found name duplicates, cannot process renaming");
        exit(1);
    }

    if !cli.skip {
        for (file_name, new_file_name) in file_rename.iter() {
            if cli.pretty_print {
                println!("{}\n ╰─> {}", file_name, new_file_name);
            } else {
                println!("{} -> {}", file_name, new_file_name);
            }
        }

        println!("Rename files ? (y\\N)");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read input");

        if a.to_lowercase().trim() != "y" {
            exit(0);
        }
    }

    for (file_name, new_file_name) in file_rename.iter() {
        rename_file(&path, &file_name, &new_file_name);
    }
    Ok(())
}

fn run_interpreter(
    path: &str,
    regex: &Regex,
    node: &Rc<dyn ExecutableNode>,
) -> HashMap<String, String> {
    let paths = read_dir(path).expect("Couldn't read dir");
    let mut file_rename = HashMap::new();
    let mut interpreter = Interpreter::new();
    for path in paths {
        let file_name = path
            .expect("Couldn't read file")
            .file_name()
            .to_str()
            .expect("Couldn't get file_name")
            .to_owned();

        if let Some(captures) = regex.captures(&file_name) {
            let result = interpreter.execute(&captures, regex.capture_names(), node.clone());

            if let Err(e) = result {
                println!("{}", e.message());
                exit(1);
            }

            let sh = result.unwrap().into_string();
            if let Err(e) = sh {
                println!("{}", e.message());
                exit(1);
            }

            file_rename.insert(file_name, sh.unwrap().inner_value.trim().to_owned());
        }
    }

    file_rename
}

fn rename_file(path: &str, old_name: &str, new_name: &str) {
    rename(path.to_owned() + old_name, path.to_owned() + new_name).expect("Couldn't rename file");
}

fn find_duplicates(file_rename: &HashMap<String, String>) -> bool {
    for (k, v) in file_rename.iter() {
        for (k2, v2) in file_rename.iter() {
            if v == v2 && k != k2 {
                return true;
            }
        }
    }

    false
}
