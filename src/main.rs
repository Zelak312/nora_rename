mod ast;
mod errors;
mod lib;
mod tokenizer;
mod utils;

use std::{
    fs::{self, DirEntry},
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let regex = Regex::new(&cli.input).expect("Invalid regex");
    let path = "./";
    let paths = fs::read_dir(path).unwrap();
    let mut vector_paths = vec![];
    for path in paths {
        vector_paths.push(path.expect("Error when getting files"));
    }

    let mut lex = lexer::Lexer::new(cli.output.clone());
    let tokens = lex.tokenize();

    let mut tree = parser::Parser::new(tokens, cli.output.clone());
    let node_result = tree.parse();
    if let Err(e) = node_result {
        println!("{}", e.message());
        exit(1);
    }

    let node = node_result.unwrap();

    if cli.skip {
        run_interpreter(false, &vector_paths, &regex, &node);
    } else {
        run_interpreter(true, &vector_paths, &regex, &node);
        println!("Rename files ? (y\\N)");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("Failed to read input");

        if a.to_lowercase().trim() == "y" {
            run_interpreter(false, &vector_paths, &regex, &node);
        }
    }
    Ok(())
}

fn run_interpreter(
    visualise: bool,
    paths: &Vec<DirEntry>,
    regex: &Regex,
    node: &Rc<dyn ExecutableNode>,
) {
    let mut interpreter = Interpreter::new();
    for path in paths {
        let file_name = path
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

            if visualise {
                println!("{} -> {}", &file_name, sh.unwrap().inner_value);
            } else {
                fs::rename(path.path().to_str().unwrap(), sh.unwrap().inner_value)
                    .expect("Couldn't rename a file");
            }
        }
    }
}
