use std::vec;

use thiserror::Error;

use crate::{
    WithPosition,
    tokenizer::{Token, TokenWithPosition},
};

#[derive(Error, Debug, PartialEq)]
#[error("Tokenizer error: {message} [{start}:{end}]")]
pub struct ParserError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

pub type Result<T> = std::result::Result<T, ParserError>;

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

#[derive(Debug, PartialEq)]
pub enum ModuleId {
    For,
    Echo,
    Cube,
    Sphere,
    Cylinder,
    Translate,
    Rotate,
    Scale,
    Color,
    Camera,
    Lambertian,
    Dielectric,
    Metal,
    /// <identifier>
    Identifier(String),
}

pub type ModuleIdWithPosition = WithPosition<ModuleId>;

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
    // TODO <expr> '.' <identifier>
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
    // TODO <expr> '?' <expr> ':' <expr>
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
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::LessThan
            | BinaryOperator::LessThanEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanEqual => 1,
            BinaryOperator::Add | BinaryOperator::Subtract => 2,
            BinaryOperator::Multiply | BinaryOperator::Divide => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UnaryOperator {
    Minus,
}

pub type ExprWithPosition = WithPosition<Expr>;

#[derive(Debug)]
pub struct ParseResult {
    pub statements: Vec<StatementWithPosition>,
    pub errors: Vec<ParserError>,
}

struct Parser {
    tokens: Vec<TokenWithPosition>,
    pos: usize,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithPosition>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: vec![],
        }
    }

    fn current(&self) -> Option<&TokenWithPosition> {
        self.peek(0)
    }

    fn peek(&self, n: usize) -> Option<&TokenWithPosition> {
        self.tokens.get(self.pos + n)
    }

    fn current_token_start(&self) -> usize {
        self.current().map(|t| t.start).unwrap_or(0)
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
            None => Err(ParserError {
                message: format!("Expected {:?}, found EOF", expected),
                start: 0,
                end: 0,
            }),
            Some(tok) => {
                if tok.item == expected {
                    self.advance();
                    Ok(())
                } else {
                    Err(ParserError {
                        message: format!("Expected {:?}, found {:?}", expected, tok.item),
                        start: tok.start,
                        end: tok.end,
                    })
                }
            }
        }
    }

    fn expect_identifier(&mut self) -> Result<String> {
        match self.current() {
            None => Err(ParserError {
                message: "Expected identifier, found EOF".to_string(),
                start: 0,
                end: 0,
            }),
            Some(tok) => {
                if let Token::Identifier(identifier) = &tok.item {
                    let identifier = identifier.clone();
                    self.advance();
                    Ok(identifier)
                } else {
                    Err(ParserError {
                        message: format!("Expected identifier, found {:?}", tok.item),
                        start: tok.start,
                        end: tok.end,
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
        let start = self.current_token_start();

        // ';'
        if self.current_matches(Token::Semicolon) {
            self.advance();
            return Ok(StatementWithPosition::new(
                Statement::Empty,
                start,
                self.current_token_start(),
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
                start,
                self.current_token_start(),
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
        let start = self.current_token_start();

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
            start,
            self.current_token_start(),
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
        let start = self.current_token_start();

        if let Some(current) = self.current() {
            let module_id = match &current.item {
                Token::For => ModuleId::For,
                Token::Identifier(identifier) => {
                    if identifier == "echo" {
                        ModuleId::Echo
                    } else if identifier == "cube" {
                        ModuleId::Cube
                    } else if identifier == "sphere" {
                        ModuleId::Sphere
                    } else if identifier == "cylinder" {
                        ModuleId::Cylinder
                    } else if identifier == "translate" {
                        ModuleId::Translate
                    } else if identifier == "rotate" {
                        ModuleId::Rotate
                    } else if identifier == "scale" {
                        ModuleId::Scale
                    } else if identifier == "color" {
                        ModuleId::Color
                    } else if identifier == "camera" {
                        ModuleId::Camera
                    } else if identifier == "lambertian" {
                        ModuleId::Lambertian
                    } else if identifier == "dielectric" {
                        ModuleId::Dielectric
                    } else if identifier == "metal" {
                        ModuleId::Metal
                    } else {
                        ModuleId::Identifier(identifier.to_owned())
                    }
                }
                _ => todo!("throw error {:?}", current.item),
            };
            self.advance();
            return Ok(ModuleIdWithPosition::new(
                module_id,
                start,
                self.current_token_start(),
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
            let argument_call = self.parse_argument_call()?;
            argument_calls.push(argument_call);

            if self.current_matches(Token::RightParen) {
                break;
            }

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
        let start = self.current_token_start();

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
                start,
                self.current_token_start(),
            ));
        }

        // <expr>
        let expr = self.parse_expr()?;
        Ok(CallArgumentWithPosition::new(
            CallArgument::Expr { expr },
            start,
            self.current_token_start(),
        ))
    }

    /// <expr> '.' <identifier>
    /// <expr> '?' <expr> ':' <expr>
    /// <expr> '[' <expr> ']'
    /// <binary expression>
    fn parse_expr(&mut self) -> Result<ExprWithPosition> {
        self.parse_binary_expr(0)
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
        let start = self.current_token_start();

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
                start,
                self.current_token_start(),
            );
        }

        // TODO '(' <expr> ')'
        // TODO <expr> '?' <expr> ':' <expr>

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
        let start = self.current_token_start();

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
                        start,
                        self.current_token_start(),
                    )
                } else {
                    ExprWithPosition::new(
                        Expr::Vector { items: expressions },
                        start,
                        self.current_token_start(),
                    )
                }
            }

            Token::True => {
                // "true"
                self.advance();
                ExprWithPosition::new(Expr::True, start, self.current_token_start())
            }

            Token::False => {
                // "false"
                self.advance();
                ExprWithPosition::new(Expr::False, start, self.current_token_start())
            }

            Token::Identifier(identifier) => {
                if self.peek_matches(1, Token::LeftParen) {
                    // <identifier> <call_arguments>
                    let name = identifier.clone();

                    self.advance(); // identifier
                    let arguments = self.parse_call_arguments()?;
                    ExprWithPosition::new(
                        Expr::FunctionCall { name, arguments },
                        start,
                        self.current_token_start(),
                    )
                } else {
                    // <identifier>
                    let name = identifier.clone();
                    self.advance(); // identifier
                    ExprWithPosition::new(
                        Expr::Identifier { name },
                        start,
                        self.current_token_start(),
                    )
                }
            }

            Token::Number(number) => {
                // <number>
                let number = *number;
                self.advance();
                ExprWithPosition::new(Expr::Number(number), start, self.current_token_start())
            }

            Token::String(str) => {
                // <string>
                let str = str.clone();
                self.advance();
                ExprWithPosition::new(Expr::String(str), start, self.current_token_start())
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
                    start,
                    self.current_token_start(),
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
                // TODO "undef"
                // TODO <expr> '.' <identifier>
                todo!("{:?}", other);
            }
        };

        // <expr> '[' <expr> ']'
        while self.current_matches(Token::LeftBracket) {
            self.expect(Token::LeftBracket)?;

            let index = self.parse_expr()?;

            self.expect(Token::RightBracket)?;

            lhs = ExprWithPosition::new(
                Expr::Index {
                    lhs: Box::new(lhs),
                    index: Box::new(index),
                },
                start,
                self.current_token_start(),
            );
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
                Err(err) => self.errors.push(err),
            }
        }

        ParseResult {
            statements,
            errors: self.errors,
        }
    }

    /// <assignment> ::=
    ///   <identifier> '=' <expr> ';'
    fn parse_assignment(&mut self) -> Result<StatementWithPosition> {
        let start = self.current_token_start();

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
            start,
            self.current_token_start(),
        ))
    }

    fn current_to_binary_operator(&self) -> Option<BinaryOperator> {
        if let Some(current) = self.current() {
            match current.item {
                // TODO <expr> '%' <expr>
                // TODO <expr> "==" <expr>
                // TODO <expr> "!=" <expr>
                // TODO <expr> "&&" <expr>
                // TODO <expr> "||" <expr>
                Token::Plus => Some(BinaryOperator::Add),
                Token::Minus => Some(BinaryOperator::Subtract),
                Token::Asterisk => Some(BinaryOperator::Multiply),
                Token::ForwardSlash => Some(BinaryOperator::Divide),
                Token::LessThan => Some(BinaryOperator::LessThan),
                Token::LessThanEqual => Some(BinaryOperator::LessThanEqual),
                Token::GreaterThan => Some(BinaryOperator::GreaterThan),
                Token::GreaterThanEqual => Some(BinaryOperator::GreaterThanEqual),
                _ => None,
            }
        } else {
            None
        }
    }

    /// <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
    fn parse_function_decl(&mut self) -> Result<StatementWithPosition> {
        let start = self.current_token_start();

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
            start,
            self.current_token_start(),
        ))
    }

    /// <empty>
    /// <argument_decl>
    /// <arguments_decl> ',' <optional_commas> <argument_decl>
    fn parse_decl_arguments(&mut self) -> Result<Vec<DeclArgumentWithPosition>> {
        self.expect(Token::LeftParen)?;

        let mut arguments: Vec<DeclArgumentWithPosition> = vec![];

        while let Some(argument) = self.parse_decl_argument() {
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
    fn parse_decl_argument(&mut self) -> Option<DeclArgumentWithPosition> {
        let start = self.current_token_start();

        let identifier = if let Some(identifier) = self.current_matches_identifier() {
            self.advance();
            identifier
        } else {
            return None;
        };

        if self.current_matches(Token::Equals) {
            todo!("decl arg with default");
        } else {
            Some(DeclArgumentWithPosition::new(
                DeclArgument::Identifier { identifier },
                start,
                self.current_token_start(),
            ))
        }
    }

    /// <ifelse_statement> ::=
    ///   <if_statement>
    ///   <if_statement> "else" <child_statement>
    /// <if_statement> ::=
    ///   "if" '(' <expr> ')' <child_statement>
    fn parse_ifelse_statement(&mut self) -> Result<StatementWithPosition> {
        let start = self.current_token_start();

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
            start,
            self.current_token_start(),
        ))
    }
}

pub fn openscad_parse(tokens: Vec<TokenWithPosition>) -> ParseResult {
    let parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::openscad_tokenize;

    use super::*;

    fn parse(value: &str) -> ParseResult {
        openscad_parse(openscad_tokenize(value).unwrap())
    }

    #[test]
    fn test_empty_statement() {
        let result = parse(";");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(
            result.statements,
            vec![StatementWithPosition::new(Statement::Empty, 0, 1)]
        );
    }

    #[test]
    fn test_cube() {
        let result = parse("cube(10);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(
            result.statements,
            vec![StatementWithPosition::new(
                Statement::ModuleInstantiation {
                    module_id: ModuleIdWithPosition::new(ModuleId::Cube, 0, 4),
                    call_arguments: vec![CallArgumentWithPosition::new(
                        CallArgument::Expr {
                            expr: ExprWithPosition::new(Expr::Number(10.0), 5, 7)
                        },
                        5,
                        7
                    )],
                    child_statements: vec![]
                },
                0,
                9
            )]
        );
    }

    #[test]
    fn test_cube_vector() {
        let result = parse("cube([20,30,50]);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());

        // Statement::ModuleInstantiation
        let stmt = &result.statements[0].item;

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
        if ModuleId::Cube != module_id.item {
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
        let result = parse("cube([20,30,50],center=true);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_translate_cube_vector_and_named_parameter() {
        let result = parse("translate([0,0,5]) cube([20,30,50],center=true);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_binary_expression() {
        let result = parse("cube(20 - 0.1);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_binary_expression_divide() {
        let result = parse("color([0,125,255]/255);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_unary_expression() {
        let result = parse("cube(-20);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_negate_parens() {
        let result = parse("echo(-(20 + 3));");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_set_fa() {
        let result = parse("$fa = 1;");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_include() {
        let result = parse("include <ray_trace.scad>");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_function_call() {
        let result = parse(
            "
            lambertian(checker(scale=0.32, even=[0.2, 0.3, 0.1], odd=[0.9, 0.9, 0.9]))
                translate([0.0, -1.0, -100.5])
                    sphere(r=100);
    ",
        );
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_for_loop() {
        let result = parse("for(a=[0:10]) sphere(r=a);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_for_loop_increment() {
        let result = parse("for(a=[0:2:10]) sphere(r=a);");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_variable_assignment() {
        let result = parse("a = 1;");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_rands() {
        let result = parse("choose_mat = rands(0,1,1)[0];");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_subtract_indexed() {
        let result = parse("v = pt2[0][1] - pt1[0];");
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_function_decl() {
        let s = "function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));";
        let result = parse(s);
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_if_else() {
        let result = parse(
            r#"
            if (1 > 2) {
              echo("false");
            } else if (5 > 2) {
              echo("ok");
            } else {
              echo("fail");
            }
        "#,
        );
        assert_eq!(Vec::<ParserError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }
}
