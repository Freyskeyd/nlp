/// Metaphone Double result
#[derive(Debug, PartialEq)]
pub struct DoubleMetaphoneResult {
    /// Primary metaphone
    pub primary: String,
    /// Alternate metaphone
    pub alternate: String,
    max_length: usize
}

impl DoubleMetaphoneResult {
    pub fn new(length: i32) -> DoubleMetaphoneResult {
        DoubleMetaphoneResult { primary: String::with_capacity(length as usize), alternate: String::with_capacity(length as usize), max_length: length as usize }
    }

    pub fn is_complete(&mut self) -> bool {
        self.primary.len() >= self.max_length && self.alternate.len() >= self.max_length
    }

    pub fn append_primary(&mut self, letter: char) {
        self.primary.push(letter);
    }

    pub fn append_alternate(&mut self, letter: char) {
        self.alternate.push(letter);
    }

    pub fn append(&mut self, letter: char) {
        self.primary.push(letter);
        self.alternate.push(letter);
    }

    pub fn cleanup(&mut self) {
        if self.primary.len() > self.max_length {
            self.primary.truncate(self.max_length);
        }
        if self.alternate.len() > self.max_length {
            self.alternate.truncate(self.max_length);
        }
    }
}
