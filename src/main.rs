use clap::{command, Arg};
use codegrep::visitor::ASTVisitor;
use std::env;

fn parse_file(filename: &str, pattern: &str, func_start: Option<&str>) {
    // let src = match fs::read_to_string(filename) {
    //     Ok(s) => s,
    //     Err(err) => {
    //         eprintln!("failed to read file: {}", err.to_string());
    //         process::exit(1);
    //     }
    // };

    let mut visitor = ASTVisitor::new(pattern);
    let _ = visitor.search(filename, func_start);
}

fn main() {
    let matches = command!()
        .arg(
            Arg::new("pattern")
                .required(true)
                .help("the pattern to search for"),
        )
        .arg(
            Arg::new("filepath")
                .required(true)
                .help("the starting filename"),
        )
        .arg(
            Arg::new("function")
                .short('n')
                .long("function-name")
                .help("the starting function name"),
        )
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let filepath = matches.get_one::<String>("filepath").unwrap();
    let func_start = matches.get_one::<String>("function");

    parse_file(filepath, pattern, func_start.map(|s| s.as_str()));
}
