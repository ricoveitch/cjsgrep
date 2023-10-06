use codegrep::indexer::Indexer;
use std::process;

fn main() {
    let mut indexer = Indexer::new("/Users/rico/Documents/builds/playground/index.js");

    if let Err(e) = indexer.index() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
