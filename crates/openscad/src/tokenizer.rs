use crate::WithPosition;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    /// '('
    LeftParen,
    /// ')'
    RightParen,
    /// '['
    LeftBracket,
    /// ']'
    RightBracket,
    /// '{'
    LeftCurlyBracket,
    /// '}'
    RightCurlyBracket,
    /// ','
    Comma,
    /// ';'
    Semicolon,
    /// '='
    Equals,
    /// '-'
    Minus,
    /// '<'
    LessThan,
    /// '>'
    GreaterThan,
    /// 'for'
    For,
    /// 'true'
    True,
    /// 'false'
    False,
    /// 'include <filename>'
    Include {
        filename: String,
    },
    /// 'use <filename>'
    Use {
        filename: String,
    },
    Cube,
    Cylinder,
    Translate,
    Rotate,
    Scale,
    Camera,

    // TODO module, function, if, else, let, assign, sphere, polyhedron, square, circle, polygon, union, difference, intersection, mirror, hull, minkowski, linear_extrude, rotate_extrude, projection
    Unknown(char),
    Eof,
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

pub type TokenWithPosition = WithPosition<Token>;

struct Tokenizer {
    input: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(mut self) -> Vec<TokenWithPosition> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next() {
            tokens.push(tok);
        }
        tokens.push(TokenWithPosition::new(Token::Eof, self.pos, self.pos));
        tokens
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let idx = self.pos + offset;
        if idx < self.input.len() {
            Some(self.input[idx])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;
        self.pos += 1;
        Some(ch)
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();
            if !self.skip_comment() {
                break;
            }
        }
    }

    fn skip_comment(&mut self) -> bool {
        if self.current() == Some('/') {
            if self.peek(1) == Some('/') {
                // Line comment
                while self.current().is_some() && self.current() != Some('\n') {
                    self.advance();
                }
                return true;
            } else if self.peek(1) == Some('*') {
                // Block comment
                self.advance(); // /
                self.advance(); // *
                while self.current().is_some() {
                    if self.current() == Some('*') && self.peek(1) == Some('/') {
                        self.advance(); // *
                        self.advance(); // /
                        break;
                    }
                    self.advance();
                }
                return true;
            }
        }
        false
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn try_read_number(&mut self) -> Option<f64> {
        let mut result = String::new();
        let mut offset = 0;
        let mut found_number = false;
        let mut found_decimal = false;

        // find middle decimals
        while let Some(ch) = self.peek(offset) {
            if ch.is_ascii_digit() {
                result.push(ch);
                found_number = true;
            } else if !found_decimal && ch == '.' {
                result.push(ch);
                found_decimal = true;
            } else {
                break;
            }
            offset += 1;
        }

        if !found_number {
            return None;
        }

        // scientific notation
        if let Some(ch) = self.peek(offset)
            && (ch == 'e' || ch == 'E')
        {
            result.push(ch);
            offset += 1;

            // +/-
            if let Some(ch) = self.peek(offset)
                && (ch == '+' || ch == '-')
            {
                result.push(ch);
                offset += 1;
            }

            // number
            while let Some(ch) = self.peek(offset) {
                if ch.is_ascii_digit() {
                    result.push(ch);
                    offset += 1;
                } else {
                    break;
                }
            }
        }

        match result.parse() {
            Ok(v) => {
                self.advance_n(offset);
                Some(v)
            }
            Err(_) => None,
        }
    }

    fn next(&mut self) -> Option<TokenWithPosition> {
        self.skip_whitespace_and_comments();

        let start = self.pos;

        let token = match self.current() {
            None => {
                return None;
            }
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some('[') => {
                self.advance();
                Token::LeftBracket
            }
            Some(']') => {
                self.advance();
                Token::RightBracket
            }
            Some('{') => {
                self.advance();
                Token::LeftCurlyBracket
            }
            Some('}') => {
                self.advance();
                Token::RightCurlyBracket
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some('=') => {
                self.advance();
                Token::Equals
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('<') => {
                self.advance();
                Token::LessThan
            }
            Some('>') => {
                self.advance();
                Token::GreaterThan
            }
            Some(ch) if ch.is_alphabetic() || ch == '_' || ch == '$' => {
                let identifier = self.read_identifier();
                if identifier == "include" {
                    Token::Include {
                        filename: self.read_include_filename(),
                    }
                } else if identifier == "use" {
                    Token::Use {
                        filename: self.read_include_filename(),
                    }
                } else if identifier == "for" {
                    Token::For
                } else if identifier == "true" {
                    Token::True
                } else if identifier == "false" {
                    Token::False
                } else if identifier == "cube" {
                    Token::Cube
                } else if identifier == "cylinder" {
                    Token::Cylinder
                } else if identifier == "translate" {
                    Token::Translate
                } else if identifier == "rotate" {
                    Token::Rotate
                } else if identifier == "scale" {
                    Token::Scale
                } else if identifier == "camera" {
                    Token::Camera
                } else {
                    Token::Identifier(identifier)
                }
            }
            Some(ch) => {
                if let Some(number) = self.try_read_number() {
                    Token::Number(number)
                } else {
                    self.advance();
                    Token::Unknown(ch)
                }
            }
        };

        Some(TokenWithPosition::new(token, start, self.pos))
    }

    fn read_include_filename(&mut self) -> String {
        self.skip_whitespace();
        if !matches!(self.current(), Some('<')) {
            todo!("include should be followed by '<'");
        }
        self.advance();

        let mut filename = String::new();
        while let Some(ch) = self.current() {
            self.advance();
            if ch == '>' {
                break;
            } else {
                filename.push(ch);
            }
        }

        filename
    }
}

pub fn openscad_tokenize(input: &str) -> Vec<TokenWithPosition> {
    let tokenizer = Tokenizer::new(input);
    tokenizer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens_with_pos(input: &str, expected: &[TokenWithPosition]) {
        let found = openscad_tokenize(input);
        assert_eq!(found, expected);
    }

    fn assert_tokens(input: &str, expected: &[Token]) {
        let found = openscad_tokenize(input);
        let found_without_pos: Vec<Token> = found.iter().map(|tok| tok.item.clone()).collect();
        assert_eq!(found_without_pos, expected);
    }

    fn assert_token_with_pos(input: &str, token: Token, start: usize, end: usize) {
        assert_tokens_with_pos(
            input,
            &vec![
                TokenWithPosition::new(token, start, end),
                TokenWithPosition::new(Token::Eof, end, end),
            ],
        )
    }

    #[test]
    fn test_re_number() {
        assert_token_with_pos("1", Token::Number(1.0), 0, 1);
        assert_token_with_pos("42", Token::Number(42.0), 0, 2);
        assert_token_with_pos("42.34", Token::Number(42.34), 0, 5);
        assert_token_with_pos("42.34e11", Token::Number(42.34e11), 0, 8);
        assert_token_with_pos("42.34E-11", Token::Number(42.34e-11), 0, 9);

        assert_tokens_with_pos(
            "42.34a",
            &vec![
                TokenWithPosition::new(Token::Number(42.34), 0, 5),
                TokenWithPosition::new(Token::Identifier("a".to_string()), 5, 6),
                TokenWithPosition::new(Token::Eof, 6, 6),
            ],
        );
    }

    #[test]
    fn test_re_identifier() {
        assert_token_with_pos("a", Token::Identifier("a".to_string()), 0, 1);
        assert_token_with_pos("cube_2", Token::Identifier("cube_2".to_string()), 0, 6);

        assert_tokens_with_pos(
            "cube(",
            &vec![
                TokenWithPosition::new(Token::Cube, 0, 4),
                TokenWithPosition::new(Token::LeftParen, 4, 5),
                TokenWithPosition::new(Token::Eof, 5, 5),
            ],
        );
    }

    #[test]
    fn test_cube() {
        assert_tokens_with_pos(
            "cube(10);",
            &vec![
                TokenWithPosition {
                    item: Token::Cube,
                    start: 0,
                    end: 4,
                },
                TokenWithPosition {
                    item: Token::LeftParen,
                    start: 4,
                    end: 5,
                },
                TokenWithPosition {
                    item: Token::Number(10.0),
                    start: 5,
                    end: 7,
                },
                TokenWithPosition {
                    item: Token::RightParen,
                    start: 7,
                    end: 8,
                },
                TokenWithPosition {
                    item: Token::Semicolon,
                    start: 8,
                    end: 9,
                },
                TokenWithPosition {
                    item: Token::Eof,
                    start: 9,
                    end: 9,
                },
            ],
        );
    }

    #[test]
    fn test_cube_vector() {
        assert_tokens(
            "cube([20,30,50]);",
            &vec![
                Token::Cube,
                Token::LeftParen,
                Token::LeftBracket,
                Token::Number(20.0),
                Token::Comma,
                Token::Number(30.0),
                Token::Comma,
                Token::Number(50.0),
                Token::RightBracket,
                Token::RightParen,
                Token::Semicolon,
                Token::Eof,
            ],
        );
    }

    #[test]
    fn test_cube_named_parameter() {
        assert_tokens(
            "cube(size=20);",
            &vec![
                Token::Cube,
                Token::LeftParen,
                Token::Identifier("size".to_string()),
                Token::Equals,
                Token::Number(20.0),
                Token::RightParen,
                Token::Semicolon,
                Token::Eof,
            ],
        );
    }

    #[test]
    fn test_set_fa() {
        assert_tokens(
            "$fa = 1;",
            &vec![
                Token::Identifier("$fa".to_string()),
                Token::Equals,
                Token::Number(1.0),
                Token::Semicolon,
                Token::Eof,
            ],
        );
    }

    #[test]
    fn test_include() {
        assert_tokens(
            "include <test.scad>",
            &vec![
                Token::Include {
                    filename: "test.scad".to_owned(),
                },
                Token::Eof,
            ],
        );
    }
}
