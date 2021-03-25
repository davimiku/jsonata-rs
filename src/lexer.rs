
pub struct Lexer<I: Iterator<Item = char>> {
    /// Iterator for chars from the input
    char_iter: I
}

impl<I: Iterator<Item = char>> Lexer<I> {
    /// Create a Lexer
    pub fn new(mut char_iter: I) -> Self {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}