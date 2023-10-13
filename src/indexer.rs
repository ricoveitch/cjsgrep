extern crate walkdir;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process;
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

struct Index {
    content: Vec<String>,
    fn_offsets: HashMap<String, usize>,
}

pub struct Indexer {
    project_dir: String,
    index: HashMap<String, Index>,
    fre: Regex,
    afre: Regex,
}

impl Indexer {
    pub fn new(project_dir: &str) -> Indexer {
        Indexer {
            project_dir: project_dir.to_string(),
            index: HashMap::new(),
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
            .map(|s| s.trim().to_string())
            .collect();
        self.index.insert(
            file_path.to_string(),
            Index {
                content,
                fn_offsets: HashMap::new(),
            },
        );
        Ok(())
    }

    fn find_funcs(&self, file_path: &str) -> Result<Vec<(String, usize)>, String> {
        let content = match self.index.get(&file_path.to_string()) {
            Some(c) => &c.content,
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
            self.index.entry(file_path.to_string()).and_modify(|f| {
                f.fn_offsets.insert(func_name, pos);
            });
        }

        Ok(())
    }

    pub fn get_fn_content(&self, func_name: &str, path: &str) -> impl Iterator<Item = &String> {
        let index = self.index.get(path).unwrap_or_else(|| {
            eprintln!("Failed to to find {path} index record");
            process::exit(1);
        });
        let offset = index.fn_offsets.get(func_name).unwrap_or_else(|| {
            eprintln!("Failed to to find {func_name} offset");
            process::exit(1);
        });
        index.content.iter().skip(*offset)
    }
}
