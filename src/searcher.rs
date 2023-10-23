use crate::{indexer::Indexer, utils::is_file};
use regex::Regex;

fn has_fn_call(token: &str) -> Option<(Option<String>, String)> {
    let re = Regex::new(r"([a-zA-Z_$][0-9a-zA-Z_$]*\.)*([a-zA-Z_$][0-9a-zA-Z_$]*)\(.*\);").unwrap();

    if let Some(cap) = re.captures(&token) {
        if let Some(object) = cap.get(1) {
            let mut object = object.as_str().to_string();
            object.pop();
            return Some((Some(object), cap[2].to_string()));
        }

        return Some((None, cap[2].to_string()));
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

    pub fn search(&self, func: &str, file_path: &str, target: &str) -> Result<Vec<String>, String> {
        if !is_file(file_path) {
            return Err(format!("no such file exists {}", file_path));
        }

        let mut results = Vec::new();

        self.traverse(&mut results, func, file_path, target, None);
        Ok(results)
    }

    fn traverse(
        &self,
        results: &mut Vec<String>,
        func_name: &str,
        file_path: &str,
        target: &str,
        object: Option<String>,
    ) {
        for line in self.indexer.iter_fn_content(file_path, func_name, object) {
            if line.chars().next() == Some('}') {
                break;
            }

            if line.contains(target) {
                results.push(format!("GREP: ({}): {}", func_name, line));
            }

            if let Some(call) = has_fn_call(line) {
                let (object, func_name) = call;
                self.traverse(results, &func_name, file_path, target, object);
            }
        }
    }
}
