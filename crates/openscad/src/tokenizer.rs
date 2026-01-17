use std::sync::Arc;

use crate::{Message, MessageLevel, Position, Result, WithPosition, source::Source};

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
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
    /// ':'
    Colon,
    /// ';'
    Semicolon,
    /// '.'
    Period,
    /// '='
    Equals,
    /// '=='
    EqualEqual,
    /// '!='
    NotEqual,
    /// '!'
    ExclamationMark,
    /// '+'
    Plus,
    /// '-'
    Minus,
    /// '*'
    Asterisk,
    /// '/'
    ForwardSlash,
    /// '?'
    QuestionMark,
    /// '%'
    Percent,
    /// '&&'
    AmpersandAmpersand,
    /// '||'
    PipePipe,
    /// '^'
    Caret,
    /// '<'
    LessThan,
    /// '<='
    LessThanEqual,
    /// '>'
    GreaterThan,
    /// '>='
    GreaterThanEqual,
    /// 'for'
    For,
    /// 'if'
    If,
    /// 'else'
    Else,
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

pub struct TokenizerResults {
    pub tokens: Option<Vec<TokenWithPosition>>,
    pub messages: Vec<Message>,
}

struct Tokenizer {
    input: Vec<char>,
    pos: usize,
    source: Arc<Box<dyn Source>>,
}

impl Tokenizer {
    pub fn new(source: Arc<Box<dyn Source>>) -> Self {
        Self {
            input: source.get_code().chars().collect(),
            pos: 0,
            source: source.clone(),
        }
    }

    pub fn tokenize(mut self) -> TokenizerResults {
        let mut tokens = Vec::new();
        loop {
            match self.next() {
                Ok(token) => match token {
                    Some(token) => tokens.push(token),
                    None => break,
                },
                Err(err) => {
                    return TokenizerResults {
                        messages: vec![err],
                        tokens: None,
                    };
                }
            }
        }
        tokens.push(TokenWithPosition::new(
            Token::Eof,
            Position {
                start: self.pos,
                end: self.pos,
                source: self.source.clone(),
            },
        ));
        TokenizerResults {
            messages: vec![],
            tokens: Some(tokens),
        }
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

    fn expect(&mut self, expected_ch: char) -> Result<()> {
        let start = self.pos;
        if let Some(found_ch) = self.current() {
            if found_ch == expected_ch {
                self.advance();
                Ok(())
            } else {
                Err(Message {
                    level: MessageLevel::Error,
                    message: format!("Expected '{expected_ch}' but found '{found_ch}'"),
                    position: Position {
                        start,
                        end: start,
                        source: self.source.clone(),
                    },
                })
            }
        } else {
            Err(Message {
                level: MessageLevel::Error,
                message: format!("Expected '{expected_ch}' but found EOF"),
                position: Position {
                    start,
                    end: start,
                    source: self.source.clone(),
                },
            })
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

    fn next(&mut self) -> Result<Option<TokenWithPosition>> {
        self.skip_whitespace_and_comments();

        let start = self.pos;

        let token = match self.current() {
            None => {
                return Ok(None);
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
            Some(':') => {
                self.advance();
                Token::Colon
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some('.') => {
                self.advance();
                Token::Period
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                Token::Asterisk
            }
            Some('/') => {
                self.advance();
                Token::ForwardSlash
            }
            Some('?') => {
                self.advance();
                Token::QuestionMark
            }
            Some('%') => {
                self.advance();
                Token::Percent
            }
            Some('^') => {
                self.advance();
                Token::Caret
            }
            Some('&') => {
                self.advance();
                if let Some(ch) = self.current() {
                    if ch == '&' {
                        self.advance();
                        Token::AmpersandAmpersand
                    } else {
                        return Err(Message {
                            level: MessageLevel::Error,
                            message: format!("Invalid character '{ch}'"),
                            position: Position {
                                start,
                                end: start + 1,
                                source: self.source.clone(),
                            },
                        });
                    }
                } else {
                    return Err(Message {
                        level: MessageLevel::Error,
                        message: "Invalid end of file".to_string(),
                        position: Position {
                            start,
                            end: start + 1,
                            source: self.source.clone(),
                        },
                    });
                }
            }
            Some('|') => {
                self.advance();
                if let Some(ch) = self.current() {
                    if ch == '|' {
                        self.advance();
                        Token::PipePipe
                    } else {
                        return Err(Message {
                            level: MessageLevel::Error,
                            message: format!("Invalid character '{ch}'"),
                            position: Position {
                                start,
                                end: start + 1,
                                source: self.source.clone(),
                            },
                        });
                    }
                } else {
                    return Err(Message {
                        level: MessageLevel::Error,
                        message: "Invalid end of file".to_string(),
                        position: Position {
                            start,
                            end: start + 1,
                            source: self.source.clone(),
                        },
                    });
                }
            }
            Some('!') => {
                self.advance();
                if let Some(ch) = self.current()
                    && ch == '='
                {
                    self.advance();
                    Token::NotEqual
                } else {
                    Token::ExclamationMark
                }
            }
            Some('=') => {
                self.advance();
                if let Some(ch) = self.current()
                    && ch == '='
                {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equals
                }
            }
            Some('<') => {
                self.advance();
                if let Some(ch) = self.current()
                    && ch == '='
                {
                    self.advance();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            Some('>') => {
                self.advance();
                if let Some(ch) = self.current()
                    && ch == '='
                {
                    self.advance();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            Some('"') => self.parse_string()?,
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
                } else if identifier == "if" {
                    Token::If
                } else if identifier == "else" {
                    Token::Else
                } else if identifier == "true" {
                    Token::True
                } else if identifier == "false" {
                    Token::False
                } else {
                    Token::Identifier(identifier)
                }
            }
            Some(ch) => {
                if let Some(number) = self.try_read_number() {
                    Token::Number(number)
                } else {
                    self.advance();
                    return Err(Message {
                        level: MessageLevel::Error,
                        message: format!("Invalid character '{ch}'"),
                        position: Position {
                            start,
                            end: start + 1,
                            source: self.source.clone(),
                        },
                    });
                }
            }
        };

        Ok(Some(TokenWithPosition::new(
            token,
            Position {
                start,
                end: self.pos,
                source: self.source.clone(),
            },
        )))
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

    fn parse_string(&mut self) -> Result<Token> {
        self.expect('"')?;

        let mut result = String::new();
        while let Some(ch) = self.current() {
            if ch == '"' {
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(ch) = self.current() {
                    match ch {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '\'' => result.push('\''),
                        '"' => result.push('"'),
                        _ => {
                            // If not a recognized escape, keep both characters
                            result.push('\\');
                            result.push(ch);
                        }
                    }
                    self.advance();
                } else {
                    return Err(Message {
                        level: MessageLevel::Error,
                        message: "Expected escape character but found EOF".to_string(),
                        position: Position {
                            start: self.pos,
                            end: self.pos,
                            source: self.source.clone(),
                        },
                    });
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        self.expect('"')?;

        Ok(Token::String(result))
    }
}

pub fn openscad_tokenize(source: Arc<Box<dyn Source>>) -> TokenizerResults {
    let tokenizer = Tokenizer::new(source);
    tokenizer.tokenize()
}

#[cfg(test)]
mod tests {
    use crate::source::StringSource;

    use super::*;

    fn assert_tokens_with_pos(source: Arc<Box<dyn Source>>, expected: &[TokenWithPosition]) {
        let found = openscad_tokenize(source);
        assert_eq!(found.tokens.unwrap(), expected);
    }

    fn assert_tokens(input: &str, expected: &[Token]) {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(input)));
        let found = openscad_tokenize(source);
        let found_without_pos: Vec<Token> = found
            .tokens
            .unwrap()
            .iter()
            .map(|tok| tok.item.clone())
            .collect();
        assert_eq!(found_without_pos, expected);
    }

    fn assert_token_with_pos(input: &str, token: Token, start: usize, end: usize) {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(input)));
        assert_tokens_with_pos(
            source.clone(),
            &vec![
                TokenWithPosition::new(
                    token,
                    Position {
                        start,
                        end,
                        source: source.clone(),
                    },
                ),
                TokenWithPosition::new(
                    Token::Eof,
                    Position {
                        start: end,
                        end,
                        source,
                    },
                ),
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

        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("42.34a")));
        assert_tokens_with_pos(
            source.clone(),
            &vec![
                TokenWithPosition::new(
                    Token::Number(42.34),
                    Position {
                        start: 0,
                        end: 5,
                        source: source.clone(),
                    },
                ),
                TokenWithPosition::new(
                    Token::Identifier("a".to_string()),
                    Position {
                        start: 5,
                        end: 6,
                        source: source.clone(),
                    },
                ),
                TokenWithPosition::new(
                    Token::Eof,
                    Position {
                        start: 6,
                        end: 6,
                        source,
                    },
                ),
            ],
        );
    }

    #[test]
    fn test_re_identifier() {
        assert_token_with_pos("a", Token::Identifier("a".to_string()), 0, 1);
        assert_token_with_pos("cube_2", Token::Identifier("cube_2".to_string()), 0, 6);

        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("cube(")));
        assert_tokens_with_pos(
            source.clone(),
            &vec![
                TokenWithPosition::new(
                    Token::Identifier("cube".to_string()),
                    Position {
                        start: 0,
                        end: 4,
                        source: source.clone(),
                    },
                ),
                TokenWithPosition::new(
                    Token::LeftParen,
                    Position {
                        start: 4,
                        end: 5,
                        source: source.clone(),
                    },
                ),
                TokenWithPosition::new(
                    Token::Eof,
                    Position {
                        start: 5,
                        end: 5,
                        source,
                    },
                ),
            ],
        );
    }

    #[test]
    fn test_cube() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("cube(10);")));
        assert_tokens_with_pos(
            source.clone(),
            &vec![
                TokenWithPosition {
                    item: Token::Identifier("cube".to_string()),
                    position: Position {
                        start: 0,
                        end: 4,
                        source: source.clone(),
                    },
                },
                TokenWithPosition {
                    item: Token::LeftParen,
                    position: Position {
                        start: 4,
                        end: 5,
                        source: source.clone(),
                    },
                },
                TokenWithPosition {
                    item: Token::Number(10.0),
                    position: Position {
                        start: 5,
                        end: 7,
                        source: source.clone(),
                    },
                },
                TokenWithPosition {
                    item: Token::RightParen,
                    position: Position {
                        start: 7,
                        end: 8,
                        source: source.clone(),
                    },
                },
                TokenWithPosition {
                    item: Token::Semicolon,
                    position: Position {
                        start: 8,
                        end: 9,
                        source: source.clone(),
                    },
                },
                TokenWithPosition {
                    item: Token::Eof,
                    position: Position {
                        start: 9,
                        end: 9,
                        source: source.clone(),
                    },
                },
            ],
        );
    }

    #[test]
    fn test_cube_vector() {
        assert_tokens(
            "cube([20,30,50]);",
            &vec![
                Token::Identifier("cube".to_string()),
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
                Token::Identifier("cube".to_string()),
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

    #[test]
    fn test_string() {
        assert_tokens(
            r#" "Test \"quotes\"" "#,
            &vec![Token::String("Test \"quotes\"".to_owned()), Token::Eof],
        );
    }
}
