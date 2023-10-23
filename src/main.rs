use clap::Parser;
use codegrep::indexer::Indexer;
use codegrep::logger;
use codegrep::searcher::Searcher;
use std::process;

#[derive(Parser, Debug)]
struct Cli {
    project_path: String,

    #[arg(short = 't', long)]
    pattern: String,

    #[arg(short = 'f', long)]
    start_func_name: String,

    #[arg(short = 'p', long)]
    start_func_path: String,
}

fn main() {
    let args = Cli::parse();

    let mut indexer = Indexer::new(&args.project_path);

    if let Err(e) = indexer.index() {
        logger::err(&format!("Failed to parse project: {e}"));
        process::exit(1);
    }

    let searcher = Searcher::new(indexer);

    let results = match searcher.search(&args.start_func_name, &args.start_func_path, &args.pattern)
    {
        Ok(res) => res,
        Err(e) => {
            logger::err(&format!("Failed to search for pattern: {e}"));
            process::exit(1);
        }
    };

    for g in results {
        logger::info(&g);
    }
}
