const ESCAPE: &'static str = "\x1b[0m";

fn red(msg: &str) -> String {
    format!("\x1b[31m{}{}", msg, ESCAPE)
}

fn green(msg: &str) -> String {
    format!("\x1b[32m{}{}", msg, ESCAPE)
}

fn yellow(msg: &str) -> String {
    format!("\x1b[33m{}{}", msg, ESCAPE)
}

pub fn err(msg: &str) {
    eprintln!("{}", red(msg));
}

pub fn warn(msg: &str) {
    println!("{}", yellow(msg));
}

pub fn info(msg: &str) {
    println!("{}", green(msg));
}
