mod ast;
mod base_parser;
mod chain_reader;
mod errors;
mod interpreter;
mod lexer;
mod lib;
mod node;
mod parser;
mod token;
mod utils;

use std::fs;

use clap::Parser;
use regex::Regex;

use crate::interpreter::Interpreter;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[clap(value_parser)]
    input: String,

    #[clap(value_parser)]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    let regex = Regex::new(&cli.input).expect("Invalid regex");
    let path = "./";
    let paths = fs::read_dir(path).unwrap();

    println!("{:?}", cli.output.clone());
    let mut lex = lexer::Lexer::new(cli.output.clone());
    let tokens = lex.tokenize();
    println!("{:?}", tokens);

    let mut tree = parser::Parser::new(tokens);
    let node_result = tree.parse();
    if let Err(err) = node_result {
        panic!("{}", err);
    }

    let node = node_result.unwrap();
    println!("{:?}", node);

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
            let sh = interpreter
                .execute(node.clone())
                .expect("nice")
                .string()
                .expect("sshesh");
            println!("{} -> {}", &file_name, sh);

            interpreter.update_count();
            count += 1;
        }
    }
}
