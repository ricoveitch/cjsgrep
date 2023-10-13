use crate::indexer::Indexer;
use regex::Regex;

fn has_fn_call(token: &str) -> Option<String> {
    let re = Regex::new(r"([a-zA-Z_$][0-9a-zA-Z_$]*)\(.*\);").unwrap();

    if let Some(cap) = re.captures(&token) {
        println!("func_call={}", cap[1].to_string());

        return Some(cap[1].to_string());
    }
    None
}

pub struct Searcher {
    indexer: Indexer,
}

impl Searcher {
    pub fn new(indexer: Indexer) -> Searcher {
        return Searcher { indexer };
    }

    pub fn search(&self, func: &str, path: &str, target: &str) -> Vec<String> {
        let mut results = Vec::new();

        self.traverse(&mut results, func, path, target);
        results
    }

    fn traverse(&self, results: &mut Vec<String>, func: &str, path: &str, target: &str) {
        for line in self.indexer.get_fn_content(func, path) {
            if line.chars().next() == Some('}') {
                break;
            }

            if line.contains(target) {
                results.push(format!("({}):{}", func, line));
            }

            if let Some(func_name) = has_fn_call(line) {
                self.traverse(results, &func_name, path, target);
            }
        }
    }
}
