use codegrep::{parser::Parser, visitor::ASTVisitor};
use std::{fs, process};

fn parse_file() {
    let filename = "./data/parser-test.js";
    let src = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("failed to read file: {}", err.to_string());
            process::exit(1);
        }
    };

    let program = Parser::new(&src).parse();
    // println!("{:?}", program);

    let mut visitor = ASTVisitor::new(&src, "pin");
    let _ = visitor.search(&program, Some("foo"));
}

fn main() {
    parse_file();
}
