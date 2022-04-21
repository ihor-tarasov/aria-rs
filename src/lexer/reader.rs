use std::{
    iter::{Cloned, Peekable},
    slice::Iter,
};

pub struct PeekableReader<'a> {
    source: Peekable<Cloned<Iter<'a, u8>>>,
    counter: usize,
}

impl<'a> PeekableReader<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source: source.iter().cloned().peekable(),
            counter: 0,
        }
    }

    pub fn peek(&mut self) -> Option<u8> {
        Some(*self.source.peek()?)
    }

    pub fn next(&mut self) -> Option<u8> {
        self.counter += 1;
        self.source.next()
    }

    pub fn counter(&self) -> usize {
        self.counter
    }
}
