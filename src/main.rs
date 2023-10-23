use codegrep::indexer::Indexer;
use codegrep::searcher::Searcher;
use std::process;

fn main() {
    let path = "data/mixed";
    let mut indexer = Indexer::new(path);

    if let Err(e) = indexer.index() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    let start_func = "foo";
    let text = "obj.double";
    let searcher = Searcher::new(indexer);

    for g in searcher.search(start_func, "data/mixed/index.js", text) {
        println!("{}", g);
    }
}
