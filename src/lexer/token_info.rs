#[derive(Clone, Copy)]
pub struct TokenInfo {
    offset: usize,
    len: usize,
}

impl TokenInfo {
    pub fn new(offset: usize, len: usize) -> Self {
        Self { offset, len }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
