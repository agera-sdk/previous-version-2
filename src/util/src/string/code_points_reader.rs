use std::str::CharIndices;

/// The `CodePointsReader` type is used for iterating code points
/// from left-to-right in a string with additional manipulation methods.
#[derive(Clone)]
pub struct CodePointsReader<'a> {
    char_indices: CharIndices<'a>,
}

impl<'a> CodePointsReader<'a> {
    /// Indicates if there are remaining code points to read.
    pub fn has_remaining(&self) -> bool {
        self.clone().char_indices.next().is_some()
    }

    /// Indicates if the reader has reached the end of the string.
    pub fn reached_end(&self) -> bool {
        self.clone().char_indices.next().is_none()
    }

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

    /// Constructs a `CodePointsReader` from a string at a given `index` position.
    /// If `index` is beyond the limit, the reader reaches the end of the string.
    pub fn from_index(string: &'a str, index: usize) -> CodePointsReader<'a> {
        let mut indices = string.char_indices();
        for _ in 0..index {
            if indices.next().is_none() {
                break;
            }
        }
        CodePointsReader { char_indices: indices }
    }
}

impl<'a> From<&'a str> for CodePointsReader<'a> {
    /// Constructs a `CodePointsReader` from a string.
    fn from(value: &'a str) -> Self {
        CodePointsReader { char_indices: value.char_indices() }
    }
}

impl<'a> From<&'a String> for CodePointsReader<'a> {
    /// Constructs a `CodePointsReader` from a string.
    fn from(value: &'a String) -> Self {
        CodePointsReader { char_indices: value.char_indices() }
    }
}

impl<'a> Iterator for CodePointsReader<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.char_indices.next().map(|(_, cp)| cp)
    }
}

#[cfg(test)]
mod test {
    use super::CodePointsReader;
    #[test]
    fn test() {
        let mut reader = CodePointsReader::from("foo");
        assert!(reader.has_remaining());
        assert_eq!(reader.peek_seq(5), "foo");
        assert_eq!(reader.peek_seq(1), "f");
        assert_eq!(reader.peek_or_zero(), 'f');
        for _ in 0..3 {
            reader.next();
        }
        assert_eq!(reader.peek_or_zero(), '\x00');
        assert!(reader.reached_end());
    }
}