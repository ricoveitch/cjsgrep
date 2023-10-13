use codegrep::indexer::Indexer;
use codegrep::searcher::Searcher;
use std::process;

fn main() {
    let path = "data/sameFile.js";
    let mut indexer = Indexer::new(path);

    if let Err(e) = indexer.index() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    let start_func = "foo";
    let text = "obj.y";
    let searcher = Searcher::new(indexer);
    searcher.search(start_func, path, text);
}
