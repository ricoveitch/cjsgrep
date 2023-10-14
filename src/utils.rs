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
