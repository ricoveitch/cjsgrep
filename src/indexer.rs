extern crate walkdir;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    let file_type = entry.file_type();

    entry
        .file_name()
        .to_str()
        .map(|s| {
            s.contains("node_modules")
                || (file_type.is_file()
                    && (!s.ends_with(".js") || s.starts_with(".") || s.contains("test")))
        })
        .unwrap_or(false)
}

pub struct Indexer {
    project_dir: String,
    content: HashMap<String, Vec<String>>,
    funcs: HashMap<String, Vec<(String, usize)>>,
    fre: Regex,
    afre: Regex,
}

impl Indexer {
    pub fn new(project_dir: &str) -> Indexer {
        Indexer {
            project_dir: project_dir.to_string(),
            content: HashMap::new(),
            funcs: HashMap::new(),
            fre: Regex::new(r"^\s*function\s+([a-zA-Z_$][0-9a-zA-Z_$]*)\s*\(").unwrap(),
            afre: Regex::new(r"^\s*(const|let|var)\s+([a-zA-Z_$][0-9a-zA-Z_$]*)\s+=\s+\(").unwrap(),
        }
    }

    pub fn index(&mut self) -> Result<(), Box<dyn Error>> {
        let walker = WalkDir::new(&self.project_dir).into_iter();

        for file in walker
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|file| file.ok())
            .filter(|file| file.file_type().is_file())
        {
            let file_path = file.path().display().to_string();
            self.read_file(&file_path)?;
        }

        Ok(())
    }

    fn store_content(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let content: Vec<String> = fs::read_to_string(file_path)?
            .lines()
            .map(|s| s.to_string())
            .collect();
        self.content.insert(file_path.to_string(), content);
        Ok(())
    }

    fn find_funcs(&self, file_path: &str) -> Result<Vec<(String, usize)>, String> {
        let content = match self.content.get(&file_path.to_string()) {
            Some(c) => c,
            None => return Err("content not found".to_string()),
        };

        let mut funcs = vec![];
        for (line_idx, line) in content.iter().enumerate() {
            if let Some(cap) = self.fre.captures(&line) {
                funcs.push((cap[1].to_string(), line_idx));
            } else if let Some(cap) = self.afre.captures(&line) {
                funcs.push((cap[2].to_string(), line_idx));
            }
        }
        Ok(funcs)
    }

    fn read_file(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.store_content(file_path)?;
        let funcs = self.find_funcs(file_path)?;

        for (func_name, pos) in funcs {
            println!("{}", func_name);

            self.funcs
                .entry(file_path.to_string())
                .or_insert(Vec::new())
                .push((func_name, pos + 1));
        }
        println!("{:?}", self.funcs);

        Ok(())
    }
}
