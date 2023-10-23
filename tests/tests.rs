use codegrep::indexer::Indexer;
use codegrep::searcher::Searcher;

fn search(target: &str) -> Vec<String> {
    let mut indexer = Indexer::new("data/mixed");
    indexer.index().unwrap();

    let searcher = Searcher::new(indexer);

    searcher
        .search("foo", "data/mixed/index.js", target)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let results = search("nonexistent");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn same_file() {
        let results = search("obj.bar");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn single_destructure_import() {
        let results = search("obj.fixed");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn multi_destructure_import() {
        let results = search("obj.qux");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn multi_nl_destructure_import() {
        let results = search("obj.double");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn default_import() {
        let results = search("obj.lar");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn single_nest() {
        let results = search("obj.baz");
        assert_eq!(results.len(), 1);
    }
}
