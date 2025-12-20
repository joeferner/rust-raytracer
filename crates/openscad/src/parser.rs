use std::vec;

use crate::{
    WithPosition,
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
    /// <module_instantiation>
    ModuleInstantiation {
        module_instantiation: ModuleInstantiationWithPosition,
    },
}

pub type StatementWithPosition = WithPosition<Statement>;

#[derive(Debug, PartialEq)]
pub enum ModuleInstantiation {
    // TODO '!' <module_instantiation>
    // TODO '#' <module_instantiation>
    // TODO '%' <module_instantiation>
    // TODO '*' <module_instantiation>
    // TODO <ifelse_statement>
    /// <single_module_instantiation> <child_statement>
    SingleModuleInstantiation {
        single_module_instantiation: SingleModuleInstantiationWithPosition,
        child_statement: ChildStatementWithPosition,
    },
}

pub type ModuleInstantiationWithPosition = WithPosition<ModuleInstantiation>;

#[derive(Debug, PartialEq)]
pub enum SingleModuleInstantiation {
    Module {
        module_id: ModuleIdWithPosition,
        call_arguments: Vec<CallArgumentWithPosition>,
    },
}

pub type SingleModuleInstantiationWithPosition = WithPosition<SingleModuleInstantiation>;

#[derive(Debug, PartialEq)]
pub enum ChildStatement {
    // ';'
    Empty,
    // '{' <child_statements> '}'
    MultipleStatements {
        statements: Vec<Box<StatementWithPosition>>,
    },
    // <module_instantiation>
    ModuleInstantiation {
        module_instantiation: Box<ModuleInstantiationWithPosition>,
    },
}

pub type ChildStatementWithPosition = WithPosition<ChildStatement>;

#[derive(Debug, PartialEq)]
pub enum ModuleId {
    /// "for"
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
    // TODO <string>
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

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct ParseResult {
    pub statements: Vec<StatementWithPosition>,
    pub errors: Vec<ParseError>,
}

struct Parser {
    tokens: Vec<TokenWithPosition>,
    pos: usize,
    errors: Vec<ParseError>,
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

    #[must_use]
    fn expect(&mut self, expected: Token) -> bool {
        match self.current() {
            None => {
                self.errors.push(ParseError {
                    message: format!("Expected {:?}, found EOF", expected),
                    start: 0,
                    end: 0,
                });
                false
            }
            Some(tok) => {
                if tok.item == expected {
                    self.advance();
                    true
                } else {
                    self.errors.push(ParseError {
                        message: format!("Expected {:?}, found {:?}", expected, tok.item),
                        start: tok.start,
                        end: tok.end,
                    });
                    false
                }
            }
        }
    }

    fn synchronize(&mut self) {
        // Skip tokens until we find a semicolon or EOF
        while let Some(tok) = self.current() {
            if tok.item == Token::Semicolon {
                self.advance();
                break;
            }
            self.advance();
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
    fn parse_statement(&mut self) -> Option<StatementWithPosition> {
        let start = self.current_token_start();

        // ';'
        if self.current_matches(Token::Semicolon) {
            self.advance();
            return Some(StatementWithPosition::new(
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
            return Some(StatementWithPosition::new(
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
        if let Some(module_instantiation) = self.parse_module_instantiation() {
            return Some(StatementWithPosition::new(
                Statement::ModuleInstantiation {
                    module_instantiation,
                },
                start,
                self.current_token_start(),
            ));
        }

        todo!();
    }

    /// <module_instantiation> ::=
    ///   '!' <module_instantiation>
    ///   '#' <module_instantiation>
    ///   '%' <module_instantiation>
    ///   '*' <module_instantiation>
    ///   <ifelse_statement>
    ///   <single_module_instantiation> <child_statement>
    fn parse_module_instantiation(&mut self) -> Option<ModuleInstantiationWithPosition> {
        let start = self.current_token_start();

        // TODO '!' <module_instantiation>
        // TODO '#' <module_instantiation>
        // TODO '%' <module_instantiation>
        // TODO '*' <module_instantiation>
        // TODO <ifelse_statement>

        // <single_module_instantiation> <child_statement>
        if let Some(single_module_instantiation) = self.parse_single_module_instantiation() {
            if let Some(child_statement) = self.parse_child_statement() {
                return Some(ModuleInstantiationWithPosition::new(
                    ModuleInstantiation::SingleModuleInstantiation {
                        single_module_instantiation,
                        child_statement,
                    },
                    start,
                    self.current_token_start(),
                ));
            } else {
                todo!("write error");
            }
        }

        todo!();
    }

    /// <single_module_instantiation> ::=
    ///   <module_id> '(' <call_arguments> ')'
    fn parse_single_module_instantiation(
        &mut self,
    ) -> Option<SingleModuleInstantiationWithPosition> {
        let start = self.current_token_start();

        // <module_id> '(' <call_arguments> ')'
        if let Some(module_id) = self.parse_module_id() {
            if let Some(call_arguments) = self.parse_call_arguments() {
                Some(SingleModuleInstantiationWithPosition::new(
                    SingleModuleInstantiation::Module {
                        module_id,
                        call_arguments,
                    },
                    start,
                    self.current_token_start(),
                ))
            } else {
                todo!("write error");
            }
        } else {
            todo!("write error");
        }
    }

    /// <child_statement> ::=
    ///   ';'
    ///   '{' <child_statements> '}'
    ///   <module_instantiation>
    fn parse_child_statement(&mut self) -> Option<ChildStatementWithPosition> {
        let start = self.current_token_start();

        // ';'
        if self.current_matches(Token::Semicolon) {
            if !self.expect(Token::Semicolon) {
                return None;
            }
            return Some(ChildStatementWithPosition::new(
                ChildStatement::Empty,
                start,
                self.current_token_start(),
            ));
        }

        if self.current_matches(Token::LeftCurlyBracket) {
            if !self.expect(Token::LeftCurlyBracket) {
                return None;
            }
            let mut child_statments: Vec<Box<StatementWithPosition>> = vec![];
            while !self.current_matches(Token::RightCurlyBracket) {
                if let Some(stmt) = self.parse_statement() {
                    child_statments.push(Box::new(stmt));
                }
            }
            if !self.expect(Token::RightCurlyBracket) {
                return None;
            }
            return Some(ChildStatementWithPosition::new(
                ChildStatement::MultipleStatements {
                    statements: child_statments,
                },
                start,
                self.current_token_start(),
            ));
        }

        // <module_instantiation>
        if let Some(module_instantiation) = self.parse_module_instantiation() {
            return Some(ChildStatementWithPosition::new(
                ChildStatement::ModuleInstantiation {
                    module_instantiation: Box::new(module_instantiation),
                },
                start,
                self.current_token_start(),
            ));
        }

        None
    }

    /// <module_id> ::=
    ///   "for"
    ///   <identifier>
    fn parse_module_id(&mut self) -> Option<ModuleIdWithPosition> {
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
            return Some(ModuleIdWithPosition::new(
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
    fn parse_call_arguments(&mut self) -> Option<Vec<CallArgumentWithPosition>> {
        if !self.expect(Token::LeftParen) {
            self.synchronize();
            return None;
        }

        let mut argument_calls: Vec<CallArgumentWithPosition> = vec![];
        while let Some(argument_call) = self.parse_argument_call() {
            argument_calls.push(argument_call);

            if self.current_matches(Token::RightParen) {
                break;
            }

            // ',' <optional_commas>
            while self.current_matches(Token::Comma) {
                self.advance();
            }
        }

        if !self.expect(Token::RightParen) {
            self.synchronize();
            return None;
        }

        Some(argument_calls)
    }

    /// <call_argument> ::=
    ///   <identifier> '=' <expr>
    ///   <expr>
    fn parse_argument_call(&mut self) -> Option<CallArgumentWithPosition> {
        let start = self.current_token_start();

        // <identifier> '=' <expr>
        if let Some(identifier) = self.current_matches_identifier()
            && self.peek_matches(1, Token::Equals)
        {
            let identifier = identifier.to_owned();
            self.advance(); // identifier
            self.advance(); // equals
            if let Some(expr) = self.parse_expr() {
                return Some(CallArgumentWithPosition::new(
                    CallArgument::NamedArgument { identifier, expr },
                    start,
                    self.current_token_start(),
                ));
            } else {
                return None;
            }
        }

        // <expr>
        if let Some(expr) = self.parse_expr() {
            return Some(CallArgumentWithPosition::new(
                CallArgument::Expr { expr },
                start,
                self.current_token_start(),
            ));
        }

        todo!();
    }

    /// <expr> '.' <identifier>
    /// <expr> '?' <expr> ':' <expr>
    /// <expr> '[' <expr> ']'
    /// <binary expression>
    fn parse_expr(&mut self) -> Option<ExprWithPosition> {
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
    fn parse_binary_expr(&mut self, min_precedence: u8) -> Option<ExprWithPosition> {
        let start = self.current_token_start();

        let mut lhs = self.parse_unary_expr()?;

        while let Some(operator) = self.current_to_binary_operator() {
            if operator.precedence() < min_precedence {
                break; // Stop if operator has lower precedence
            }

            self.advance(); // consume operator

            if let Some(rhs) = self.parse_binary_expr(operator.precedence() + 1) {
                lhs = ExprWithPosition::new(
                    Expr::Binary {
                        operator,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                    start,
                    self.current_token_start(),
                );
            } else {
                return None;
            }
        }

        // TODO '(' <expr> ')'
        // TODO <expr> '?' <expr> ':' <expr>

        Some(lhs)
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
    fn parse_unary_expr(&mut self) -> Option<ExprWithPosition> {
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
                        if !self.expect(Token::Comma) {
                            return None;
                        }
                        continue;
                    }

                    if !found_comma && self.current_matches(Token::Colon) {
                        if expressions.is_empty() {
                            todo!("add error since leading colon is not allowed");
                        }
                        found_colon = true;
                        if !self.expect(Token::Colon) {
                            return None;
                        }
                        continue;
                    }

                    if let Some(expr) = self.parse_expr() {
                        expressions.push(expr);
                    } else if found_colon {
                        todo!("add error, expression is required in colon statements")
                    }
                }
                if !self.expect(Token::RightBracket) {
                    return None;
                }

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

            Token::Minus => {
                // '-' <expr>
                self.advance();
                if let Some(rhs) = self.parse_expr() {
                    ExprWithPosition::new(
                        Expr::Unary {
                            operator: UnaryOperator::Minus,
                            rhs: Box::new(rhs),
                        },
                        start,
                        self.current_token_start(),
                    )
                } else {
                    return None;
                }
            }

            other => {
                // TODO "let" <call_arguments> <expr>
                // TODO "undef"
                // TODO <expr> '.' <identifier>
                // TODO <string>
                todo!("{:?}", other);
            }
        };

        // <expr> '[' <expr> ']'
        while self.current_matches(Token::LeftBracket) {
            if !self.expect(Token::LeftBracket) {
                return None;
            }

            let index = self.parse_expr()?;

            if !self.expect(Token::RightBracket) {
                return None;
            }
            lhs = ExprWithPosition::new(
                Expr::Index {
                    lhs: Box::new(lhs),
                    index: Box::new(index),
                },
                start,
                self.current_token_start(),
            );
        }

        Some(lhs)
    }

    pub fn parse(mut self) -> ParseResult {
        let mut statements = vec![];

        while let Some(tok) = self.current() {
            if tok.item == Token::Eof {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }

        ParseResult {
            statements,
            errors: self.errors,
        }
    }

    /// <assignment> ::=
    ///   <identifier> '=' <expr> ';'
    fn parse_assignment(&mut self) -> Option<StatementWithPosition> {
        let start = self.current_token_start();

        // <identifier>
        let identifier = if let Some(identifier) = self.current_matches_identifier() {
            self.advance();
            identifier
        } else {
            todo!("expected identifier")
        };

        // '='
        if !self.expect(Token::Equals) {
            todo!("expected equal");
        }

        // <expr>
        let expr = self.parse_expr()?;

        // ';'
        if !self.expect(Token::Semicolon) {
            todo!("expected semicolon");
        }

        Some(StatementWithPosition::new(
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
    fn parse_function_decl(&mut self) -> Option<StatementWithPosition> {
        let start = self.current_token_start();

        let function_name = if let Some(identifier) = self.current_matches_identifier() {
            self.advance(); // identifier
            identifier
        } else {
            return None;
        };

        let arguments = self.parse_decl_arguments()?;

        if !self.expect(Token::Equals) {
            return None;
        }

        let expr = self.parse_expr()?;

        if !self.expect(Token::Semicolon) {
            return None;
        }

        Some(StatementWithPosition::new(
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
    fn parse_decl_arguments(&mut self) -> Option<Vec<DeclArgumentWithPosition>> {
        if !self.expect(Token::LeftParen) {
            return None;
        }

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

        if !self.expect(Token::RightParen) {
            return None;
        }

        Some(arguments)
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
}

pub fn openscad_parse(tokens: Vec<TokenWithPosition>) -> ParseResult {
    let parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::openscad_tokenize;

    use super::*;

    #[test]
    fn test_empty_statement() {
        let result = openscad_parse(openscad_tokenize(";"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(
            result.statements,
            vec![StatementWithPosition::new(Statement::Empty, 0, 1)]
        );
    }

    #[test]
    fn test_cube() {
        let result = openscad_parse(openscad_tokenize("cube(10);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(
            result.statements,
            vec![StatementWithPosition::new(
                Statement::ModuleInstantiation {
                    module_instantiation: ModuleInstantiationWithPosition::new(
                        ModuleInstantiation::SingleModuleInstantiation {
                            single_module_instantiation: SingleModuleInstantiationWithPosition::new(
                                SingleModuleInstantiation::Module {
                                    module_id: ModuleIdWithPosition::new(ModuleId::Cube, 0, 4),
                                    call_arguments: vec![CallArgumentWithPosition::new(
                                        CallArgument::Expr {
                                            expr: ExprWithPosition::new(Expr::Number(10.0), 5, 7)
                                        },
                                        5,
                                        7
                                    )]
                                },
                                0,
                                8
                            ),
                            child_statement: ChildStatementWithPosition::new(
                                ChildStatement::Empty,
                                8,
                                9
                            )
                        },
                        0,
                        9
                    )
                },
                0,
                9
            )]
        );
    }

    #[test]
    fn test_cube_vector() {
        let result = openscad_parse(openscad_tokenize("cube([20,30,50]);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());

        // Statement::ModuleInstantiation
        let stmt = &result.statements[0].item;
        let module_instantiation_item = if let Statement::ModuleInstantiation {
            module_instantiation,
        } = stmt
        {
            &module_instantiation.item
        } else {
            panic!("expected Statement::ModuleInstantiation");
        };

        // ModuleInstantiation::SingleModuleInstantiation
        let (single_module_instantiation, child_statement) =
            if let ModuleInstantiation::SingleModuleInstantiation {
                single_module_instantiation,
                child_statement,
            } = module_instantiation_item
            {
                (single_module_instantiation, child_statement)
            } else {
                panic!("expected ModuleInstantiation::SingleModuleInstantiation")
            };

        // SingleModuleInstantiation::Module
        let (module_id, call_arguments) = if let SingleModuleInstantiation::Module {
            module_id,
            call_arguments,
        } = &single_module_instantiation.item
        {
            (module_id, call_arguments)
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
        if child_statement.item != ChildStatement::Empty {
            panic!("expected ChildStatement::Empty");
        }
    }

    #[test]
    fn test_cube_vector_and_named_parameter() {
        let result = openscad_parse(openscad_tokenize("cube([20,30,50],center=true);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_translate_cube_vector_and_named_parameter() {
        let result = openscad_parse(openscad_tokenize(
            "translate([0,0,5]) cube([20,30,50],center=true);",
        ));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_binary_expression() {
        let result = openscad_parse(openscad_tokenize("cube(20 - 0.1);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_binary_expression_divide() {
        let result = openscad_parse(openscad_tokenize("color([0,125,255]/255);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_unary_expression() {
        let result = openscad_parse(openscad_tokenize("cube(-20);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_set_fa() {
        let result = openscad_parse(openscad_tokenize("$fa = 1;"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_include() {
        let result = openscad_parse(openscad_tokenize("include <ray_trace.scad>"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_function_call() {
        let result = openscad_parse(openscad_tokenize(
            "
            lambertian(checker(scale=0.32, even=[0.2, 0.3, 0.1], odd=[0.9, 0.9, 0.9]))
                translate([0.0, -1.0, -100.5])
                    sphere(r=100);
    ",
        ));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_for_loop() {
        let result = openscad_parse(openscad_tokenize("for(a=[0:10]) sphere(r=a);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_for_loop_increment() {
        let result = openscad_parse(openscad_tokenize("for(a=[0:2:10]) sphere(r=a);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_variable_assignment() {
        let result = openscad_parse(openscad_tokenize("a = 1;"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_rands() {
        let result = openscad_parse(openscad_tokenize("choose_mat = rands(0,1,1)[0];"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_subtract_indexed() {
        let result = openscad_parse(openscad_tokenize("v = pt2[0][1] - pt1[0];"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }

    #[test]
    fn test_function_decl() {
        let s = "function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));";
        let result = openscad_parse(openscad_tokenize(s));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(1, result.statements.len());
    }
}
