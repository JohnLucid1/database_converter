mod library;
use clap::Parser;
use library::lib::create_vec_from_string;
use library::tokens::{Token, TokenKind};
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    file: PathBuf,
    #[arg(short, long)]
    to: String,
}

fn handle_lang(lang: String, vec: Vec<Token>) -> Result<String, ()> {
    match lang.as_str() {
        "sqlite" => Ok(Token::to_sqlite(vec)),
        "postgresql" => Ok(Token::to_postgresql(vec)),
        "mysql" => Ok(Token::to_mysql(vec)),
        _ => {
            eprintln!("ERROR: can't parse langauge \nList of parsable languages: sqlite, postgresql, mysql");
            exit(1)
        }
    }
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(args.file).unwrap_or_else(|err| {
        eprintln!("ERROR: no such file \n{err:?}");
        exit(1)
    });

    let mut something: Vec<Token> = Vec::new();

    let data: Vec<String> = create_vec_from_string(content);

    for i in data.into_iter() {
        something.push(TokenKind::tokenize(i))
    }

    let answer = handle_lang(args.to, something).unwrap_or_else(|err| {
        eprintln!("ERROR: Couldn't parse langauge {err:#?}");
        exit(1)
    });

    println!("{}", answer)
}
