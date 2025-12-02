use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Number(f64),
    LeftParen,
    RightParen,
    Semicolon,
    EOF,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-10;
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => (l0 - r0).abs() < EPSILON,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenizerError {
    PastEndOfFile,
}

pub type Result<T> = std::result::Result<T, TokenizerError>;

const RE_NUMBER: &str = r"^[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?";
const RE_IDENTIFIER: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*";

pub struct OpenscadTokenizer {
    contents: String,
    position: usize,
    re_number: Regex,
    re_identifier: Regex,
}

impl OpenscadTokenizer {
    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_owned(),
            position: 0,
            re_number: Regex::new(RE_NUMBER).unwrap(),
            re_identifier: Regex::new(RE_IDENTIFIER).unwrap(),
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        let s = match self.contents.get(self.position..) {
            Some(s) => s,
            None => return Err(TokenizerError::PastEndOfFile),
        };

        // end of file
        if s.is_empty() {
            self.position += 1;
            return Ok(Token::EOF);
        }

        // symbols
        if s.starts_with(';') {
            self.position += 1;
            return Ok(Token::Semicolon);
        }
        if s.starts_with('(') {
            self.position += 1;
            return Ok(Token::LeftParen);
        }
        if s.starts_with(')') {
            self.position += 1;
            return Ok(Token::RightParen);
        }

        // number
        if let Some(m) = self.re_number.captures(s) {
            let m = m.get_match();
            self.position += m.end();
            if let Ok(v) = m.as_str().parse::<f64>() {
                return Ok(Token::Number(v));
            }
        }

        // identifier
        if let Some(m) = self.re_identifier.captures(s) {
            let m = m.get_match();
            self.position += m.end();
            return Ok(Token::Identifier(m.as_str().to_owned()));
        }

        todo!("{s}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_match {
        ($re:expr, $input:expr, $expected:expr, $end:expr) => {
            let m = $re.captures($input).unwrap().get_match();
            assert_eq!($expected, m.as_str());
            assert_eq!($end, m.end());
        };
        ($re:expr, $input:expr, $expected:expr, $end:expr, $msg:expr) => {
            let m = $re.captures($input).unwrap().get_match();
            assert_eq!($expected, m.as_str(), $msg);
            assert_eq!($end, m.end(), $msg);
        };
    }

    #[test]
    fn test_re_number() {
        let re = Regex::new(RE_NUMBER).unwrap();
        assert_match!(re, "1", "1", 1);
        assert_match!(re, "42", "42", 2);
        assert_match!(re, "-42.34", "-42.34", 6);
        assert_match!(re, "-42.34e11", "-42.34e11", 9);
        assert_match!(re, "-42.34e-11", "-42.34e-11", 10);
        assert_match!(re, "-42.34a", "-42.34", 6, "trailing character");
    }

    #[test]
    fn test_re_identifier() {
        let re = Regex::new(RE_IDENTIFIER).unwrap();
        assert_match!(re, "a", "a", 1);
        assert_match!(re, "cube", "cube", 4);
        assert_match!(re, "cube(", "cube", 4, "trailing character");
    }

    #[test]
    fn test_simple() {
        let mut tokenizer = OpenscadTokenizer::new("cube(10);");
        assert_eq!(
            tokenizer.next_token().unwrap(),
            Token::Identifier("cube".to_string())
        );
        assert_eq!(tokenizer.next_token().unwrap(), Token::LeftParen);
        assert_eq!(tokenizer.next_token().unwrap(), Token::Number(10.0));
        assert_eq!(tokenizer.next_token().unwrap(), Token::RightParen);
        assert_eq!(tokenizer.next_token().unwrap(), Token::Semicolon);
        assert_eq!(tokenizer.next_token().unwrap(), Token::EOF);
        assert_eq!(
            tokenizer.next_token().unwrap_err(),
            TokenizerError::PastEndOfFile
        );
    }
}
