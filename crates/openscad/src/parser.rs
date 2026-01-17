use std::{sync::Arc, vec};

use crate::{
    Message, MessageLevel, Position, Result, WithPosition,
    source::Source,
    tokenizer::{Token, TokenWithPosition},
};

#[derive(Debug, PartialEq)]
pub enum Statement {
    /// ';'
    Empty,
    // TODO '{' <inner_input> '}'
    /// <assignment>
    Assignment {
        identifier: String,
        expr: ExprWithPosition,
    },
    /// "include" <include_file>
    Include { filename: String },
    // TODO "use" <include_file>
    // TODO "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
    // "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
    FunctionDecl {
        function_name: String,
        arguments: Vec<DeclArgumentWithPosition>,
        expr: ExprWithPosition,
    },

    // TODO '!' <module_instantiation>
    // TODO '#' <module_instantiation>
    // TODO '%' <module_instantiation>
    // TODO '*' <module_instantiation>
    //  <ifelse_statement>
    If {
        expr: ExprWithPosition,
        true_statements: Vec<StatementWithPosition>,
        false_statements: Vec<StatementWithPosition>,
    },
    /// <single_module_instantiation> <child_statement>
    ModuleInstantiation {
        module_id: ModuleIdWithPosition,
        call_arguments: Vec<CallArgumentWithPosition>,
        child_statements: Vec<StatementWithPosition>,
    },
}

pub type StatementWithPosition = WithPosition<Statement>;

pub type ModuleIdWithPosition = WithPosition<String>;

#[derive(Debug, PartialEq, Clone)]
pub enum CallArgument {
    /// <identifier> '=' <expr>
    NamedArgument {
        identifier: String,
        expr: ExprWithPosition,
    },
    /// <expr>
    Expr { expr: ExprWithPosition },
}

pub type CallArgumentWithPosition = WithPosition<CallArgument>;

#[derive(Debug, PartialEq, Clone)]
pub enum DeclArgument {
    /// <identifier> '=' <expr>
    WithDefault {
        identifier: String,
        default_expr: ExprWithPosition,
    },
    /// <identifier>
    Identifier { identifier: String },
}

pub type DeclArgumentWithPosition = WithPosition<DeclArgument>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    // "true"
    True,
    // "false"
    False,
    // TODO "undef"
    //  <identifier>
    Identifier {
        name: String,
    },
    // <expr> '.' <identifier>
    FieldAccess {
        lhs: Box<ExprWithPosition>,
        field: String,
    },
    // <string>
    String(String),
    /// <number>
    Number(f64),
    // TODO "let" <call_arguments> <expr>
    // '[' <expr> ':' <expr> ']'
    Range {
        start: Box<ExprWithPosition>,
        end: Box<ExprWithPosition>,
        increment: Option<Box<ExprWithPosition>>,
    },
    // TODO '[' <expr> ':' <expr> ':' <expr> ']'
    // TODO '[' <list_comprehension_elements> ']'
    // TODO '[' <optional_commas> ']'

    // '[' (<expr> ',' <optional_commas>)* ']'
    Vector {
        items: Vec<ExprWithPosition>,
    },

    /// <expr> '*' <expr>
    /// <expr> '/' <expr>
    /// <expr> '%' <expr>
    /// <expr> '+' <expr>
    /// <expr> '-' <expr>
    /// <expr> '<' <expr>
    /// <expr> "<=" <expr>
    /// <expr> "==" <expr>
    /// <expr> "!=" <expr>
    /// <expr> ">=" <expr>
    /// <expr> '>' <expr>
    /// <expr> "&&" <expr>
    /// <expr> "||" <expr>
    Binary {
        operator: BinaryOperator,
        lhs: Box<ExprWithPosition>,
        rhs: Box<ExprWithPosition>,
    },

    /// '+' <expr>
    /// '-' <expr>
    /// '!' <expr>
    Unary {
        operator: UnaryOperator,
        rhs: Box<ExprWithPosition>,
    },
    // TODO '(' <expr> ')'
    // <expr> '?' <expr> ':' <expr>
    Ternary {
        condition: Box<ExprWithPosition>,
        true_expr: Box<ExprWithPosition>,
        false_expr: Box<ExprWithPosition>,
    },
    // <expr> '[' <expr> ']'
    Index {
        lhs: Box<ExprWithPosition>,
        index: Box<ExprWithPosition>,
    },

    // <identifier> <call_arguments>
    FunctionCall {
        name: String,
        arguments: Vec<CallArgumentWithPosition>,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum BinaryOperator {
    Exponentiation,
    Modulus,
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    EqualEqual,
    NotEqual,
    And,
    Or,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Or => 0,
            BinaryOperator::And => 1,
            BinaryOperator::LessThan
            | BinaryOperator::LessThanEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanEqual
            | BinaryOperator::EqualEqual
            | BinaryOperator::NotEqual => 2,
            BinaryOperator::Add | BinaryOperator::Subtract => 3,
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulus => 4,
            BinaryOperator::Exponentiation => 5,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UnaryOperator {
    Minus,
    Negation,
}

pub type ExprWithPosition = WithPosition<Expr>;

#[derive(Debug)]
pub struct ParseResult {
    pub statements: Option<Vec<StatementWithPosition>>,
    pub messages: Vec<Message>,
}

struct Parser {
    tokens: Vec<TokenWithPosition>,
    pos: usize,
    source: Arc<Box<dyn Source>>,
    messages: Vec<Message>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithPosition>, source: Arc<Box<dyn Source>>) -> Self {
        Self {
            tokens,
            pos: 0,
            source,
            messages: vec![],
        }
    }

    fn current(&self) -> Option<&TokenWithPosition> {
        self.peek(0)
    }

    fn peek(&self, n: usize) -> Option<&TokenWithPosition> {
        self.tokens.get(self.pos + n)
    }

    fn get_current(&self) -> Result<Position> {
        match self.current() {
            Some(current) => Ok(current.position.clone()),
            None => Err(Message {
                level: MessageLevel::Error,
                message: "no current token".to_owned(),
                position: Position {
                    start: 0,
                    end: 0,
                    source: self.source.clone(),
                },
            }),
        }
    }

    fn current_token_start(&self) -> usize {
        self.current().map(|t| t.position.start).unwrap_or(0)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn current_matches(&self, expected: Token) -> bool {
        self.peek_matches(0, expected)
    }

    fn current_matches_identifier(&self) -> Option<String> {
        if let Some(tok) = self.current()
            && let Token::Identifier(identifier) = &tok.item
        {
            Some(identifier.clone())
        } else {
            None
        }
    }

    fn peek_matches(&self, n: usize, expected: Token) -> bool {
        match self.peek(n) {
            None => false,
            Some(tok) => tok.item == expected,
        }
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        match self.current() {
            None => Err(Message {
                level: MessageLevel::Error,
                message: format!("Expected {:?}, found EOF", expected),
                position: Position {
                    start: 0,
                    end: 0,
                    source: self.source.clone(),
                },
            }),
            Some(tok) => {
                if tok.item == expected {
                    self.advance();
                    Ok(())
                } else {
                    Err(Message {
                        level: MessageLevel::Error,
                        message: format!("Expected {:?}, found {:?}", expected, tok.item),
                        position: tok.position.clone(),
                    })
                }
            }
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        match self.current() {
            None => Err(Message {
                level: MessageLevel::Error,
                message: "Expected identifier, found EOF".to_string(),
                position: Position {
                    start: 0,
                    end: 0,
                    source: self.source.clone(),
                },
            }),
            Some(tok) => {
                if let Token::Identifier(identifier) = &tok.item {
                    let identifier = identifier.clone();
                    self.advance();
                    Ok(identifier)
                } else {
                    Err(Message {
                        level: MessageLevel::Error,
                        message: format!("Expected identifier, found {:?}", tok.item),
                        position: tok.position.clone(),
                    })
                }
            }
        }
    }

    /// <statement> ::=
    ///   ';'
    ///   '{' <inner_input> '}'
    ///   "include" <include_file>
    ///   "use" <include_file>
    ///   "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
    ///   "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
    ///   <assignment>
    ///   <module_instantiation>
    fn parse_statement(&mut self) -> Result<StatementWithPosition> {
        let pos = self.get_current()?;

        // ';'
        if self.current_matches(Token::Semicolon) {
            self.advance();
            return Ok(StatementWithPosition::new(
                Statement::Empty,
                Position {
                    start: pos.start,
                    end: self.current_token_start(),
                    source: pos.source.clone(),
                },
            ));
        }

        // TODO '{' <inner_input> '}'

        // TODO "use" <include_file>
        // "include" <include_file>
        if let Some(tok) = self.current()
            && let Token::Include { filename } = &tok.item
        {
            let filename = filename.to_owned();
            self.advance();
            return Ok(StatementWithPosition::new(
                Statement::Include { filename },
                Position {
                    start: pos.start,
                    end: self.current_token_start(),
                    source: pos.source.clone(),
                },
            ));
        }

        if let Some(identifier) = self.current_matches_identifier() {
            if identifier == "function" {
                self.advance(); // function
                return self.parse_function_decl();
            } else if identifier == "module" {
                // TODO "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
                todo!("module decl")
            }
        }

        // <assignment>
        if self.current_matches_identifier().is_some() && self.peek_matches(1, Token::Equals) {
            return self.parse_assignment();
        }

        // <module_instantiation>
        self.parse_module_instantiation()
    }

    /// <module_instantiation> ::=
    ///   '!' <module_instantiation>
    ///   '#' <module_instantiation>
    ///   '%' <module_instantiation>
    ///   '*' <module_instantiation>
    ///   <ifelse_statement>
    ///   <single_module_instantiation> <child_statement>
    fn parse_module_instantiation(&mut self) -> Result<StatementWithPosition> {
        // TODO '!' <module_instantiation>
        // TODO '#' <module_instantiation>
        // TODO '%' <module_instantiation>
        // TODO '*' <module_instantiation>

        // <ifelse_statement>
        if self.current_matches(Token::If) {
            return self.parse_ifelse_statement();
        }

        // <single_module_instantiation> <child_statement>
        self.parse_single_module_instantiation()
    }

    /// <single_module_instantiation> <child_statement>
    /// <single_module_instantiation> ::=
    ///   <module_id> '(' <call_arguments> ')'
    fn parse_single_module_instantiation(&mut self) -> Result<StatementWithPosition> {
        let pos = self.get_current()?;

        // <module_id> '(' <call_arguments> ')'
        let module_id = self.parse_module_id()?;
        let call_arguments = self.parse_call_arguments()?;
        let child_statements = self.parse_child_statements()?;

        Ok(StatementWithPosition::new(
            Statement::ModuleInstantiation {
                module_id,
                call_arguments,
                child_statements,
            },
            Position {
                start: pos.start,
                end: self.current_token_start(),
                source: pos.source,
            },
        ))
    }

    /// <child_statement> ::=
    ///   ';'
    ///   '{' <child_statements> '}'
    ///   <module_instantiation>
    fn parse_child_statements(&mut self) -> Result<Vec<StatementWithPosition>> {
        // ';'
        if self.current_matches(Token::Semicolon) {
            self.expect(Token::Semicolon)?;
            return Ok(vec![]);
        }

        if self.current_matches(Token::LeftCurlyBracket) {
            self.expect(Token::LeftCurlyBracket)?;
            let mut child_statments: Vec<StatementWithPosition> = vec![];
            while !self.current_matches(Token::RightCurlyBracket) {
                let stmt = self.parse_statement()?;
                child_statments.push(stmt);
            }
            self.expect(Token::RightCurlyBracket)?;
            return Ok(child_statments);
        }

        // <module_instantiation>
        let module_instantiation = self.parse_module_instantiation()?;
        Ok(vec![module_instantiation])
    }

    /// <module_id> ::=
    ///   "for"
    ///   <identifier>
    fn parse_module_id(&mut self) -> Result<ModuleIdWithPosition> {
        let pos = self.get_current()?;

        if let Some(current) = self.current() {
            let module_id = match &current.item {
                Token::For => "for".to_owned(),
                Token::Identifier(identifier) => identifier.to_owned(),
                other => {
                    let other = other.clone();
                    self.advance();
                    return Err(Message {
                        level: MessageLevel::Error,
                        message: format!("Expected for or identifier but found: {other:?}"),
                        position: pos,
                    });
                }
            };
            self.advance();
            return Ok(ModuleIdWithPosition::new(
                module_id,
                Position {
                    start: pos.start,
                    end: self.current_token_start(),
                    source: pos.source,
                },
            ));
        }

        todo!("parse_module_id: {:?}", self.current());
    }

    /// <call_arguments> ::=
    ///   '(' ')'
    ///   '(' <argument_call> (',' <optional_commas> <argument_call>)* ')'
    fn parse_call_arguments(&mut self) -> Result<Vec<CallArgumentWithPosition>> {
        self.expect(Token::LeftParen)?;

        let mut argument_calls: Vec<CallArgumentWithPosition> = vec![];
        loop {
            if self.current_matches(Token::RightParen) {
                break;
            }

            let argument_call = self.parse_argument_call()?;
            argument_calls.push(argument_call);

            // ',' <optional_commas>
            while self.current_matches(Token::Comma) {
                self.advance();
            }
        }

        self.expect(Token::RightParen)?;

        Ok(argument_calls)
    }

    /// <call_argument> ::=
    ///   <identifier> '=' <expr>
    ///   <expr>
    fn parse_argument_call(&mut self) -> Result<CallArgumentWithPosition> {
        let pos = self.get_current()?;

        // <identifier> '=' <expr>
        if let Some(identifier) = self.current_matches_identifier()
            && self.peek_matches(1, Token::Equals)
        {
            let identifier = identifier.to_owned();
            self.advance(); // identifier
            self.expect(Token::Equals)?;

            let expr = self.parse_expr()?;
            return Ok(CallArgumentWithPosition::new(
                CallArgument::NamedArgument { identifier, expr },
                Position {
                    start: pos.start,
                    end: self.current_token_start(),
                    source: pos.source,
                },
            ));
        }

        // <expr>
        let expr = self.parse_expr()?;
        Ok(CallArgumentWithPosition::new(
            CallArgument::Expr { expr },
            Position {
                start: pos.start,
                end: self.current_token_start(),
                source: pos.source,
            },
        ))
    }

    /// <expr> '.' <identifier>
    /// <expr> '?' <expr> ':' <expr>
    /// <expr> '[' <expr> ']'
    /// <binary expression>
    fn parse_expr(&mut self) -> Result<ExprWithPosition> {
        let pos = self.get_current()?;
        let lhs = self.parse_binary_expr(0)?;

        // <expr> '?' <expr> ':' <expr>
        if self.current_matches(Token::QuestionMark) {
            self.expect(Token::QuestionMark)?;
            let true_expr = self.parse_binary_expr(0)?;
            self.expect(Token::Colon)?;
            let false_expr = self.parse_binary_expr(0)?;
            return Ok(ExprWithPosition::new(
                Expr::Ternary {
                    condition: Box::new(lhs),
                    true_expr: Box::new(true_expr),
                    false_expr: Box::new(false_expr),
                },
                Position {
                    start: pos.start,
                    end: self.current_token_start(),
                    source: pos.source,
                },
            ));
        }

        Ok(lhs)
    }

    /// <expr> '*' <expr>
    /// <expr> '/' <expr>
    /// <expr> '%' <expr>
    /// <expr> '+' <expr>
    /// <expr> '-' <expr>
    /// <expr> '<' <expr>
    /// <expr> "<=" <expr>
    /// <expr> "==" <expr>
    /// <expr> "!=" <expr>
    /// <expr> ">=" <expr>
    /// <expr> '>' <expr>
    /// <expr> "&&" <expr>
    /// <expr> "||" <expr>
    /// <unary expression>
    fn parse_binary_expr(&mut self, min_precedence: u8) -> Result<ExprWithPosition> {
        let pos = self.get_current()?;

        let mut lhs = self.parse_unary_expr()?;

        while let Some(operator) = self.current_to_binary_operator() {
            if operator.precedence() < min_precedence {
                break; // Stop if operator has lower precedence
            }

            self.advance(); // consume operator

            let rhs = self.parse_binary_expr(operator.precedence() + 1)?;
            lhs = ExprWithPosition::new(
                Expr::Binary {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                Position {
                    start: pos.start,
                    end: self.current_token_start(),
                    source: pos.source.clone(),
                },
            );
        }

        // TODO '(' <expr> ')'

        Ok(lhs)
    }

    /// "true"
    /// "false"
    /// "undef"
    /// <identifier>
    /// <string>
    /// <number>
    /// "let" <call_arguments> <expr>
    /// '[' <expr> ':' <expr> ']'
    /// '[' <expr> ':' <expr> ':' <expr> ']'
    /// '[' <list_comprehension_elements> ']'
    /// '[' <optional_commas> ']'
    /// '[' (<expr> ',' <optional_commas>)* ']'
    /// '+' <expr>
    /// '-' <expr>
    /// '!' <expr>
    /// '(' <expr> ')'
    /// <identifier> <call_arguments>
    fn parse_unary_expr(&mut self) -> Result<ExprWithPosition> {
        let pos = self.get_current()?;

        // TODO '+' <expr>
        // TODO '!' <expr>

        let token = if let Some(token) = self.current() {
            token
        } else {
            todo!("error missing token");
        };

        let mut lhs: ExprWithPosition = match &token.item {
            Token::LeftBracket => {
                // '[' (<expr> ',' <optional_commas>)* ']'
                // '[' <expr> ':' <expr> ']'
                // '[' <expr> ':' <expr> ':' <expr> ']'
                // '[' <list_comprehension_elements> ']'
                // '[' <optional_commas> ']'

                self.advance();
                let mut expressions = vec![];
                let mut found_comma = false;
                let mut found_colon = false;
                while !self.current_matches(Token::RightBracket) {
                    if !found_colon && self.current_matches(Token::Comma) {
                        found_comma = true;
                        self.expect(Token::Comma)?;
                        continue;
                    }

                    if !found_comma && self.current_matches(Token::Colon) {
                        if expressions.is_empty() {
                            todo!("add error since leading colon is not allowed");
                        }
                        found_colon = true;
                        self.expect(Token::Colon)?;
                        continue;
                    }

                    let expr = self.parse_expr()?;
                    expressions.push(expr);
                }
                self.expect(Token::RightBracket)?;

                if found_colon {
                    let (start_expr, end_expr, increment_expr) = if expressions.len() == 2 {
                        let start_expr = Box::new(expressions.remove(0));
                        let end_expr = Box::new(expressions.remove(0));
                        (start_expr, end_expr, None)
                    } else if expressions.len() == 3 {
                        let start_expr = Box::new(expressions.remove(0));
                        let increment_expr = Some(Box::new(expressions.remove(0)));
                        let end_expr = Box::new(expressions.remove(0));
                        (start_expr, end_expr, increment_expr)
                    } else {
                        todo!("add error since colon statements must be 2 or 3 expressions");
                    };

                    ExprWithPosition::new(
                        Expr::Range {
                            start: start_expr,
                            end: end_expr,
                            increment: increment_expr,
                        },
                        Position {
                            start: pos.start,
                            end: self.current_token_start(),
                            source: pos.source.clone(),
                        },
                    )
                } else {
                    ExprWithPosition::new(
                        Expr::Vector { items: expressions },
                        Position {
                            start: pos.start,
                            end: self.current_token_start(),
                            source: pos.source.clone(),
                        },
                    )
                }
            }

            Token::True => {
                // "true"
                self.advance();
                ExprWithPosition::new(
                    Expr::True,
                    Position {
                        start: pos.start,
                        end: self.current_token_start(),
                        source: pos.source.clone(),
                    },
                )
            }

            Token::False => {
                // "false"
                self.advance();
                ExprWithPosition::new(
                    Expr::False,
                    Position {
                        start: pos.start,
                        end: self.current_token_start(),
                        source: pos.source.clone(),
                    },
                )
            }

            Token::Identifier(identifier) => {
                if self.peek_matches(1, Token::LeftParen) {
                    // <identifier> <call_arguments>
                    let name = identifier.clone();

                    self.advance(); // identifier
                    let arguments = self.parse_call_arguments()?;
                    ExprWithPosition::new(
                        Expr::FunctionCall { name, arguments },
                        Position {
                            start: pos.start,
                            end: self.current_token_start(),
                            source: pos.source.clone(),
                        },
                    )
                } else {
                    // <identifier>
                    let name = identifier.clone();
                    self.advance(); // identifier
                    ExprWithPosition::new(
                        Expr::Identifier { name },
                        Position {
                            start: pos.start,
                            end: self.current_token_start(),
                            source: pos.source.clone(),
                        },
                    )
                }
            }

            Token::Number(number) => {
                // <number>
                let number = *number;
                self.advance();
                ExprWithPosition::new(
                    Expr::Number(number),
                    Position {
                        start: pos.start,
                        end: self.current_token_start(),
                        source: pos.source.clone(),
                    },
                )
            }

            Token::String(str) => {
                // <string>
                let str = str.clone();
                self.advance();
                ExprWithPosition::new(
                    Expr::String(str),
                    Position {
                        start: pos.start,
                        end: self.current_token_start(),
                        source: pos.source.clone(),
                    },
                )
            }

            Token::Minus => {
                // '-' <expr>
                self.expect(Token::Minus)?;
                let rhs = self.parse_expr()?;
                ExprWithPosition::new(
                    Expr::Unary {
                        operator: UnaryOperator::Minus,
                        rhs: Box::new(rhs),
                    },
                    Position {
                        start: pos.start,
                        end: self.current_token_start(),
                        source: pos.source.clone(),
                    },
                )
            }

            Token::ExclamationMark => {
                // '!' <expr>
                self.expect(Token::ExclamationMark)?;
                let rhs = self.parse_expr()?;
                ExprWithPosition::new(
                    Expr::Unary {
                        operator: UnaryOperator::Negation,
                        rhs: Box::new(rhs),
                    },
                    Position {
                        start: pos.start,
                        end: self.current_token_start(),
                        source: pos.source.clone(),
                    },
                )
            }

            Token::LeftParen => {
                // '(' <expr> ')'
                self.expect(Token::LeftParen)?;
                let expr = self.parse_expr()?;
                self.expect(Token::RightParen)?;
                expr
            }

            other => {
                // TODO "let" <call_arguments> <expr>
                return Err(Message {
                    level: MessageLevel::Error,
                    message: format!("unhandled: {other:?}"),
                    position: pos,
                });
            }
        };

        // <expr> '[' <expr> ']'
        // <expr> '.' <identifier>
        loop {
            if self.current_matches(Token::LeftBracket) {
                self.expect(Token::LeftBracket)?;

                let index = self.parse_expr()?;

                self.expect(Token::RightBracket)?;

                lhs = ExprWithPosition::new(
                    Expr::Index {
                        lhs: Box::new(lhs),
                        index: Box::new(index),
                    },
                    Position {
                        start: pos.start,
                        end: self.current_token_start(),
                        source: pos.source.clone(),
                    },
                );
            } else if self.current_matches(Token::Period) {
                self.expect(Token::Period)?;

                if let Some(identifier) = self.current_matches_identifier() {
                    self.advance();

                    lhs = ExprWithPosition::new(
                        Expr::FieldAccess {
                            lhs: Box::new(lhs),
                            field: identifier,
                        },
                        Position {
                            start: pos.start,
                            end: self.current_token_start(),
                            source: pos.source.clone(),
                        },
                    );
                } else {
                    todo!("expected identifier");
                }
            } else {
                break;
            }
        }

        Ok(lhs)
    }

    pub fn parse(mut self) -> ParseResult {
        let mut statements = vec![];

        while let Some(tok) = self.current() {
            if tok.item == Token::Eof {
                break;
            }
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => self.messages.push(err),
            }
        }

        ParseResult {
            statements: Some(statements),
            messages: self.messages,
        }
    }

    /// <assignment> ::=
    ///   <identifier> '=' <expr> ';'
    fn parse_assignment(&mut self) -> Result<StatementWithPosition> {
        let pos = self.get_current()?;

        // <identifier>
        let identifier = if let Some(identifier) = self.current_matches_identifier() {
            self.advance();
            identifier
        } else {
            todo!("expected identifier")
        };

        // '='
        self.expect(Token::Equals)?;

        // <expr>
        let expr = self.parse_expr()?;

        // ';'
        self.expect(Token::Semicolon)?;

        Ok(StatementWithPosition::new(
            Statement::Assignment { identifier, expr },
            Position {
                start: pos.start,
                end: self.current_token_start(),
                source: pos.source,
            },
        ))
    }

    fn current_to_binary_operator(&self) -> Option<BinaryOperator> {
        if let Some(current) = self.current() {
            match current.item {
                Token::Caret => Some(BinaryOperator::Exponentiation),
                Token::Percent => Some(BinaryOperator::Modulus),
                Token::Plus => Some(BinaryOperator::Add),
                Token::Minus => Some(BinaryOperator::Subtract),
                Token::Asterisk => Some(BinaryOperator::Multiply),
                Token::ForwardSlash => Some(BinaryOperator::Divide),
                Token::LessThan => Some(BinaryOperator::LessThan),
                Token::LessThanEqual => Some(BinaryOperator::LessThanEqual),
                Token::GreaterThan => Some(BinaryOperator::GreaterThan),
                Token::GreaterThanEqual => Some(BinaryOperator::GreaterThanEqual),
                Token::EqualEqual => Some(BinaryOperator::EqualEqual),
                Token::NotEqual => Some(BinaryOperator::NotEqual),
                Token::AmpersandAmpersand => Some(BinaryOperator::And),
                Token::PipePipe => Some(BinaryOperator::Or),
                _ => None,
            }
        } else {
            None
        }
    }

    /// <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
    fn parse_function_decl(&mut self) -> Result<StatementWithPosition> {
        let pos = self.get_current()?;

        let function_name = self.expect_identifier()?;

        let arguments = self.parse_decl_arguments()?;

        self.expect(Token::Equals)?;

        let expr = self.parse_expr()?;

        self.expect(Token::Semicolon)?;

        Ok(StatementWithPosition::new(
            Statement::FunctionDecl {
                function_name,
                arguments,
                expr,
            },
            Position {
                start: pos.start,
                end: self.current_token_start(),
                source: pos.source,
            },
        ))
    }

    /// <empty>
    /// <argument_decl>
    /// <arguments_decl> ',' <optional_commas> <argument_decl>
    fn parse_decl_arguments(&mut self) -> Result<Vec<DeclArgumentWithPosition>> {
        self.expect(Token::LeftParen)?;

        let mut arguments: Vec<DeclArgumentWithPosition> = vec![];

        while let Some(argument) = self.parse_decl_argument()? {
            arguments.push(argument);

            if self.current_matches(Token::RightParen) {
                break;
            }

            // ',' <optional_commas>
            while self.current_matches(Token::Comma) {
                self.advance();
            }
        }

        self.expect(Token::RightParen)?;

        Ok(arguments)
    }

    /// <identifier>
    /// <identifier> '=' <expr>
    fn parse_decl_argument(&mut self) -> Result<Option<DeclArgumentWithPosition>> {
        let pos = self.get_current()?;

        let identifier = if let Some(identifier) = self.current_matches_identifier() {
            self.advance();
            identifier
        } else {
            return Ok(None);
        };

        if self.current_matches(Token::Equals) {
            todo!("decl arg with default");
        } else {
            Ok(Some(DeclArgumentWithPosition::new(
                DeclArgument::Identifier { identifier },
                Position {
                    start: pos.start,
                    end: self.current_token_start(),
                    source: pos.source,
                },
            )))
        }
    }

    /// <ifelse_statement> ::=
    ///   <if_statement>
    ///   <if_statement> "else" <child_statement>
    /// <if_statement> ::=
    ///   "if" '(' <expr> ')' <child_statement>
    fn parse_ifelse_statement(&mut self) -> Result<StatementWithPosition> {
        let pos = self.get_current()?;

        self.expect(Token::If)?;
        self.expect(Token::LeftParen)?;

        let expr = self.parse_expr()?;

        self.expect(Token::RightParen)?;

        let true_statements = self.parse_child_statements()?;

        let false_statements = if self.current_matches(Token::Else) {
            self.expect(Token::Else)?;
            self.parse_child_statements()?
        } else {
            vec![]
        };

        let stmt = Statement::If {
            expr,
            true_statements,
            false_statements,
        };
        Ok(StatementWithPosition::new(
            stmt,
            Position {
                start: pos.start,
                end: self.current_token_start(),
                source: pos.source,
            },
        ))
    }
}

pub fn openscad_parse(tokens: Vec<TokenWithPosition>, source: Arc<Box<dyn Source>>) -> ParseResult {
    let parser = Parser::new(tokens, source);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        source::{Source, StringSource},
        tokenizer::openscad_tokenize,
    };

    use super::*;

    fn parse(source: Arc<Box<dyn Source>>) -> ParseResult {
        let tokens = openscad_tokenize(source.clone()).tokens.unwrap();
        openscad_parse(tokens, source)
    }

    #[test]
    fn test_empty_statement() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(";")));
        let result = parse(source.clone());
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(
            result.statements.unwrap(),
            vec![StatementWithPosition::new(
                Statement::Empty,
                Position {
                    start: 0,
                    end: 1,
                    source
                }
            )]
        );
    }

    #[test]
    fn test_cube() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("cube(10);")));
        let result = parse(source.clone());
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(
            result.statements.unwrap(),
            vec![StatementWithPosition::new(
                Statement::ModuleInstantiation {
                    module_id: ModuleIdWithPosition::new(
                        "cube".to_string(),
                        Position {
                            start: 0,
                            end: 4,
                            source: source.clone()
                        }
                    ),
                    call_arguments: vec![CallArgumentWithPosition::new(
                        CallArgument::Expr {
                            expr: ExprWithPosition::new(
                                Expr::Number(10.0),
                                Position {
                                    start: 5,
                                    end: 7,
                                    source: source.clone()
                                }
                            )
                        },
                        Position {
                            start: 5,
                            end: 7,
                            source: source.clone()
                        }
                    )],
                    child_statements: vec![]
                },
                Position {
                    start: 0,
                    end: 9,
                    source: source.clone()
                }
            )]
        );
    }

    #[test]
    fn test_cube_vector() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("cube([20,30,50]);")));
        let result = parse(source.clone());
        assert_eq!(Vec::<Message>::new(), result.messages);
        let statements = result.statements.unwrap();
        assert_eq!(1, statements.len());

        // Statement::ModuleInstantiation
        let stmt = &statements[0].item;

        let (module_id, call_arguments, child_statements) =
            if let Statement::ModuleInstantiation {
                module_id,
                call_arguments,
                child_statements,
            } = &stmt
            {
                (module_id, call_arguments, child_statements)
            } else {
                panic!("expected SingleModuleInstantiation::Module");
            };

        // module_id
        if "cube" != module_id.item {
            panic!("expected ModuleId::Cube");
        };

        // call_arguments
        assert_eq!(1, call_arguments.len());
        if let CallArgument::Expr { expr } = &call_arguments[0].item {
            if let Expr::Vector { items } = &expr.item {
                assert_eq!(3, items.len());
            } else {
                panic!("expected Expr::Vector");
            }
        } else {
            panic!("expected CallArgument::Expr");
        }

        // child_statement
        if !child_statements.is_empty() {
            panic!("expected ChildStatement::Empty");
        }
    }

    #[test]
    fn test_cube_vector_and_named_parameter() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("cube([20,30,50],center=true);")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_translate_cube_vector_and_named_parameter() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(
            "translate([0,0,5]) cube([20,30,50],center=true);",
        )));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_binary_expression() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("cube(20 - 0.1);")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_binary_expression_divide() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("color([0,125,255]/255);")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_unary_expression() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("cube(-20);")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_negate_parens() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("echo(-(20 + 3));")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_set_fa() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("$fa = 1;")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_include() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("include <caustic.scad>")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_function_call() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(
            "
            lambertian(checker(scale=0.32, even=[0.2, 0.3, 0.1], odd=[0.9, 0.9, 0.9]))
                translate([0.0, -1.0, -100.5])
                    sphere(r=100);
            ",
        )));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_for_loop() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("for(a=[0:10]) sphere(r=a);")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_for_loop_increment() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("for(a=[0:2:10]) sphere(r=a);")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_variable_assignment() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new("a = 1;")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_rands() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("choose_mat = rands(0,1,1)[0];")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_subtract_indexed() {
        let source: Arc<Box<dyn Source>> =
            Arc::new(Box::new(StringSource::new("v = pt2[0][1] - pt1[0];")));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_function_decl() {
        let s = "function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));";
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(s)));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_if_else() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(
            r#"
            if (1 > 2) {
              echo("false");
            } else if (5 > 2) {
              echo("ok");
            } else {
              echo("fail");
            }
        "#,
        )));
        let result = parse(source);
        assert_eq!(Vec::<Message>::new(), result.messages);
        assert_eq!(1, result.statements.unwrap().len());
    }

    #[test]
    fn test_unexpected_identifier() {
        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(
            r#"
            union() {
                circle(r=15);
                translate([12, 0, 0]) circle(r=15);
            }
            "#,
        )));
        let result = parse(source);
        assert_eq!(0, result.messages.len());
        assert_eq!(1, result.statements.unwrap().len());
    }
}
