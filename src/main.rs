mod ast;
mod errors;
mod lib;
mod tokenizer;
mod utils;

use std::{fs, process::exit};

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
    /// Optional name to operate on
    #[clap(value_parser)]
    input: String,

    #[clap(value_parser)]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let regex = Regex::new(&cli.input).expect("Invalid regex");
    let path = "./";
    let paths = fs::read_dir(path).unwrap();

    // println!("{:?}", cli.output.clone());
    let mut lex = lexer::Lexer::new(cli.output.clone());
    let tokens = lex.tokenize();
    // println!("{:?}", tokens);

    let mut tree = parser::Parser::new(tokens, cli.output.clone());
    let node_result = tree.parse();
    if let Err(e) = node_result {
        println!("{}", e.message());
        exit(1);
    }

    let node = node_result.unwrap();
    // println!("{:?}", node);

    let mut count = 1;
    for path in paths {
        if path.is_err() {
            panic!("{}", path.err().unwrap());
        }

        let curr_path = path.unwrap();
        let file_name = curr_path
            .file_name()
            .to_str()
            .expect("Couldn't get file_name")
            .to_owned();

        if let Some(captures) = regex.captures(&file_name) {
            // println!("{:?}", captures);
            let mut interpreter = Interpreter::new(count, captures, regex.capture_names());
            // TODO: interpreter should only be made once
            // And handle parsing multiple asts
            let result = interpreter.execute(node.clone());

            if let Err(e) = result {
                println!("{}", e.message());
                exit(1);
            }

            let sh = result.unwrap().into_string();
            if let Err(e) = sh {
                println!("{}", e.message());
                exit(1);
            }

            println!("{} -> {}", &file_name, sh.unwrap().inner_value);

            interpreter.update_count();
            count += 1;
        }
    }

    Ok(())
}
