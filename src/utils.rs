use std::path::Path;
use std::{fs, io};

pub struct OptionIterator<I> {
    pub iter: Option<I>,
}

impl<I, T> Iterator for OptionIterator<I>
where
    I: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match &mut self.iter {
            Some(iter) => iter.next(),
            None => None,
        }
    }
}

impl<I> OptionIterator<I> {
    pub fn new(iter: Option<I>) -> OptionIterator<I> {
        OptionIterator { iter }
    }
}

pub fn path_exists(path: &str) -> bool {
    if let Err(_) = fs::metadata(path) {
        return false;
    }

    return true;
}

pub fn is_file(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(f) => f.is_file(),
        Err(_) => false,
    }
}

pub fn get_absolute_path(path: &str) -> io::Result<String> {
    match Path::new(path).canonicalize() {
        Ok(pb) => Ok(pb.display().to_string()),
        Err(e) => Err(e),
    }
}
