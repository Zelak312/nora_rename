mod chain_reader;
mod lexer;
mod token;
mod utils;

use std::fs;

use clap::Parser;
use regex::Regex;

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

        if let Some(captues) = regex.captures(&file_name) {
            println!("{:?}", captues);
            let mut lex = lexer::Lexer::new(cli.output.clone());
            let tokens = lex.tokenize();
            println!("{:?}", tokens);
            break;
        }
    }
}
