use std::str::CharIndices;

/// The `CodePointsReader` type is used for iterating code points
/// from left-to-right from a string with additional manipulation methods.
#[derive(Clone)]
pub struct CodePointsReader<'a> {
    char_indices: CharIndices<'a>,
}

impl<'a> CodePointsReader<'a> {
    /// Returns the current index in the string.
    pub fn index(&self) -> usize {
        self.clone().char_indices.next().map_or(0, |(i, _)| i)
    }

    /// Returns the next code point. If there are no code points
    /// available, returns U+00.
    pub fn next_or_zero(&mut self) -> char {
        self.char_indices.next().map_or('\x00', |(_, cp)| cp)
    }

    /// Peeks the next code point.
    pub fn peek(&self) -> Option<char> {
        self.clone().char_indices.next().map(|(_, cp)| cp)
    }

    /// Peeks the next code point. If there are no code points
    /// available, returns U+00.
    pub fn peek_or_zero(&self) -> char {
        self.clone().next_or_zero()
    }

    /// Peeks a number of code points until the string's end.
    pub fn peek_seq(&self, num_code_points: u64) -> String {
        let mut r = String::new();
        let mut next_indices = self.char_indices.clone();
        for _ in 0..num_code_points {
            match next_indices.next() {
                None => {
                    break;
                },
                Some(cp) => {
                    r.push(cp.1);
                }
            }
        }
        r
    }
}

impl<'a> From<&'a str> for CodePointsReader<'a> {
    fn from(value: &'a str) -> Self {
        CodePointsReader { char_indices: value.char_indices() }
    }
}

impl<'a> Iterator for CodePointsReader<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.char_indices.next().map(|(_, cp)| cp)
    }
}