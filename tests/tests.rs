use codegrep::indexer::Indexer;
use codegrep::searcher::Searcher;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_file() {
        let path = "data/sameFile.js";
        let mut indexer = Indexer::new(path);
        indexer.index().unwrap();

        let searcher = Searcher::new(indexer);
        let target = "obj.y";
        let results = searcher.search("foo", path, target);
        assert_eq!(results.len(), 1);
    }
}
