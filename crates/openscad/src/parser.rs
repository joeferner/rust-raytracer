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
    // TODO "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
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
    // TODO '{' <child_statements> '}'
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Expr {
    // "true"
    True,
    // "false"
    False,
    // TODO "undef"
    // TODO <identifier>
    // TODO <expr> '.' <identifier>
    // TODO <string>
    /// <number>
    Number(f64),
    // TODO "let" <call_arguments> <expr>
    // TODO '[' <expr> ':' <expr> ']'
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
    // TODO <expr> '[' <expr> ']'
    // <identifier> <call_arguments>
    FunctionCall {
        name: String,
        arguments: Vec<CallArgumentWithPosition>,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Minus,
    Divide,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

        // TODO "use" <include_file>
        // TODO "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
        // TODO "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'

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
            self.advance();
            return Some(ChildStatementWithPosition::new(
                ChildStatement::Empty,
                start,
                self.current_token_start(),
            ));
        }

        if self.current_matches(Token::LeftCurlyBracket) {
            // TODO '{' <child_statements> '}'
            todo!()
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
                    if identifier == "cube" {
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

    /// <expr> ::=
    ///   "true"
    ///   "false"
    ///   "undef"
    ///   <identifier>
    ///   <expr> '.' <identifier>
    ///   <string>
    ///   <number>
    ///   "let" <call_arguments> <expr>
    ///   '[' <expr> ':' <expr> ']'
    ///   '[' <expr> ':' <expr> ':' <expr> ']'
    ///   '[' <list_comprehension_elements> ']'
    ///   '[' <optional_commas> ']'
    ///   '[' (<expr> ',' <optional_commas>)* ']'
    ///   <expr> '*' <expr>
    ///   <expr> '/' <expr>
    ///   <expr> '%' <expr>
    ///   <expr> '+' <expr>
    ///   <expr> '-' <expr>
    ///   <expr> '<' <expr>
    ///   <expr> "<=" <expr>
    ///   <expr> "==" <expr>
    ///   <expr> "!=" <expr>
    ///   <expr> ">=" <expr>
    ///   <expr> '>' <expr>
    ///   <expr> "&&" <expr>
    ///   <expr> "||" <expr>
    ///   '+' <expr>
    ///   '-' <expr>
    ///   '!' <expr>
    ///   '(' <expr> ')'
    ///   <expr> '?' <expr> ':' <expr>
    ///   <expr> '[' <expr> ']'
    ///   <identifier> <call_arguments>
    fn parse_expr(&mut self) -> Option<ExprWithPosition> {
        let start = self.current_token_start();

        // TODO "let" <call_arguments> <expr>
        // TODO '[' <expr> ':' <expr> ']'
        // TODO '[' <expr> ':' <expr> ':' <expr> ']'
        // TODO '[' <list_comprehension_elements> ']'
        // TODO '[' <optional_commas> ']'

        let lhs: ExprWithPosition = if self.current_matches(Token::LeftBracket) {
            // '[' (<expr> ',' <optional_commas>)* ']'
            self.advance();
            let mut expressions = vec![];
            while !self.current_matches(Token::RightBracket) {
                if self.current_matches(Token::Comma) {
                    self.advance();
                    continue;
                }
                if let Some(expr) = self.parse_expr() {
                    expressions.push(expr);
                }
            }
            self.expect(Token::RightBracket);
            ExprWithPosition::new(
                Expr::Vector { items: expressions },
                start,
                self.current_token_start(),
            )
        } else if self.current_matches(Token::True) {
            // "true"
            self.advance();
            ExprWithPosition::new(Expr::True, start, self.current_token_start())
        } else if self.current_matches(Token::False) {
            // "false"
            self.advance();
            ExprWithPosition::new(Expr::False, start, self.current_token_start())
        } else if let Some(tok) = self.current()
            && let Token::Identifier(identifier) = &tok.item
        {
            // TODO <identifier>

            // <identifier> <call_arguments>
            if self.peek_matches(1, Token::LeftParen) {
                let name = identifier.clone();

                self.advance(); // identifier
                let arguments = self.parse_call_arguments()?;
                ExprWithPosition::new(
                    Expr::FunctionCall { name, arguments },
                    start,
                    self.current_token_start(),
                )
            } else {
                todo!("add error");
            }
        }
        // TODO "undef"
        // TODO <expr> '.' <identifier>
        // TODO <string>
        else if let Some(tok) = self.current()
            && let Token::Number(number) = &tok.item
        {
            // <number>
            let number = *number;
            self.advance();
            ExprWithPosition::new(Expr::Number(number), start, self.current_token_start())
        } else if self.current_matches(Token::Minus) {
            // '-' <expr>
            self.advance();
            if let Some(rhs) = self.parse_expr() {
                return Some(ExprWithPosition::new(
                    Expr::Unary {
                        operator: UnaryOperator::Minus,
                        rhs: Box::new(rhs),
                    },
                    start,
                    self.current_token_start(),
                ));
            } else {
                return None;
            }
        } else {
            todo!("{:?}", self.current());
        };

        // TODO '+' <expr>
        // TODO '!' <expr>

        // TODO <expr> '*' <expr>
        // TODO <expr> '%' <expr>
        // TODO <expr> '+' <expr>
        // TODO <expr> '<' <expr>
        // TODO <expr> "<=" <expr>
        // TODO <expr> "==" <expr>
        // TODO <expr> "!=" <expr>
        // TODO <expr> ">=" <expr>
        // TODO <expr> '>' <expr>
        // TODO <expr> "&&" <expr>
        // TODO <expr> "||" <expr>

        // <expr> '/' <expr>
        // <expr> '-' <expr>
        if let Some(operator) = self.current_to_binary_operator() {
            self.advance();
            if let Some(rhs) = self.parse_expr() {
                return Some(ExprWithPosition::new(
                    Expr::Binary {
                        operator,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                    start,
                    self.current_token_start(),
                ));
            } else {
                return None;
            }
        }

        // TODO '(' <expr> ')'
        // TODO <expr> '?' <expr> ':' <expr>
        // TODO <expr> '[' <expr> ']'

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
                Token::Minus => Some(BinaryOperator::Minus),
                Token::ForwardSlash => Some(BinaryOperator::Divide),
                _ => None,
            }
        } else {
            None
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
}
