use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::{
    BinaryOperator, CallArgument, CallArgumentWithPosition, ChildStatement,
    ChildStatementWithPosition, Expr, ExprWithPosition, ModuleId, ModuleInstantiation,
    ModuleInstantiationWithPosition, SingleModuleInstantiation,
    SingleModuleInstantiationWithPosition, Statement, StatementWithPosition, UnaryOperator,
};

#[derive(Debug, Clone, Copy)]
pub enum Module {
    // 3d
    Cube,
    Cylinder,

    // transformations
    Translate,
}

#[derive(Debug)]
pub struct ModuleInstance {
    pub module: Module,
    pub arguments: Vec<ModuleArgument>,
}

#[derive(Debug)]
pub struct ModuleInstanceTree {
    pub instance: ModuleInstance,
    pub children: RefCell<Vec<Rc<ModuleInstanceTree>>>,
}

#[derive(Debug)]
pub enum ModuleArgument {
    Positional(ModuleArgumentValue),
    NamedArgument {
        name: String,
        value: ModuleArgumentValue,
    },
}

#[derive(Debug)]
pub enum ModuleArgumentValue {
    Number(f64),
    Vector { items: Vec<ModuleArgumentValue> },
    True,
    False,
}

#[derive(Debug, PartialEq)]
pub struct InterpreterError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct InterpreterResults {
    pub trees: Vec<Rc<ModuleInstanceTree>>,
    pub errors: Vec<InterpreterError>,
}

struct Interpreter {
    modules: HashMap<String, Module>,
    stack: Vec<Rc<ModuleInstanceTree>>,
    results: Vec<Rc<ModuleInstanceTree>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut modules = HashMap::new();

        // 3d
        modules.insert("cube".to_string(), Module::Cube);
        modules.insert("cylinder".to_string(), Module::Cylinder);

        // transformations
        modules.insert("translate".to_string(), Module::Translate);

        Self {
            modules,
            stack: vec![],
            results: vec![],
        }
    }

    fn interpret(mut self, statements: Vec<StatementWithPosition>) -> InterpreterResults {
        for statement in statements {
            self.process_statement(statement);
        }
        InterpreterResults {
            trees: self.results,
            errors: vec![], // TODO
        }
    }

    fn process_statement(&mut self, statement: StatementWithPosition) {
        match statement.item {
            Statement::Empty => (),
            Statement::ModuleInstantiation {
                module_instantiation,
            } => self.process_module_instantiation(&module_instantiation),
        }
    }

    fn process_module_instantiation(
        &mut self,
        module_instantiation: &ModuleInstantiationWithPosition,
    ) {
        match &module_instantiation.item {
            ModuleInstantiation::SingleModuleInstantiation {
                single_module_instantiation,
                child_statement,
            } => {
                self.process_single_module_instantiation(single_module_instantiation);
                self.process_child_statement(child_statement)
            }
        }
    }

    fn process_single_module_instantiation(
        &mut self,
        single_module_instantiation: &SingleModuleInstantiationWithPosition,
    ) {
        match &single_module_instantiation.item {
            SingleModuleInstantiation::Module {
                module_id,
                call_arguments,
            } => match &module_id.item {
                ModuleId::For => todo!(),
                ModuleId::Identifier(identifier) => {
                    if let Some(module) = self.modules.get(identifier) {
                        let instance = ModuleInstance {
                            module: *module,
                            arguments: self.process_call_arguments(call_arguments),
                        };
                        self.append_instance(instance);
                    } else {
                        todo!("handle unknown module \"{identifier}\"");
                    }
                }
            },
        }
    }

    fn expr_to_module_argument_value(&self, expr: &ExprWithPosition) -> ModuleArgumentValue {
        match &expr.item {
            Expr::Number(number) => ModuleArgumentValue::Number(*number),
            Expr::Vector { items } => ModuleArgumentValue::Vector {
                items: items
                    .iter()
                    .map(|v| self.expr_to_module_argument_value(v))
                    .collect(),
            },
            Expr::True => ModuleArgumentValue::True,
            Expr::False => ModuleArgumentValue::False,
            Expr::Binary { operator, lhs, rhs } => {
                self.evaluate_binary_expression(operator, lhs, rhs)
            }
            Expr::Unary { operator, rhs } => self.evaluate_unary_expression(operator, rhs),
        }
    }

    fn evaluate_binary_expression(
        &self,
        operator: &BinaryOperator,
        lhs: &ExprWithPosition,
        rhs: &ExprWithPosition,
    ) -> ModuleArgumentValue {
        let left = self.expr_to_module_argument_value(lhs);
        let right = self.expr_to_module_argument_value(rhs);

        if let ModuleArgumentValue::Number(left) = left
            && let ModuleArgumentValue::Number(right) = right
        {
            match operator {
                BinaryOperator::Minus => ModuleArgumentValue::Number(left - right),
            }
        } else {
            todo!("{left:?} {operator:?} {right:?}");
        }
    }

    fn evaluate_unary_expression(
        &self,
        operator: &UnaryOperator,
        rhs: &ExprWithPosition,
    ) -> ModuleArgumentValue {
        let right = self.expr_to_module_argument_value(rhs);

        if let ModuleArgumentValue::Number(right) = right {
            match operator {
                UnaryOperator::Minus => ModuleArgumentValue::Number(-right),
            }
        } else {
            todo!("{operator:?} {right:?}");
        }
    }

    fn process_call_arguments(
        &self,
        call_arguments: &Vec<CallArgumentWithPosition>,
    ) -> Vec<ModuleArgument> {
        let mut results: Vec<ModuleArgument> = vec![];

        for call_argument in call_arguments {
            match &call_argument.item {
                CallArgument::Expr { expr } => results.push(ModuleArgument::Positional(
                    self.expr_to_module_argument_value(expr),
                )),
                CallArgument::NamedArgument { identifier, expr } => {
                    let value = self.expr_to_module_argument_value(expr);
                    results.push(ModuleArgument::NamedArgument {
                        name: identifier.to_string(),
                        value,
                    })
                }
            }
        }

        results
    }

    fn process_child_statement(&mut self, child_statement: &ChildStatementWithPosition) {
        match &child_statement.item {
            ChildStatement::Empty => {
                self.stack.clear();
            }
            ChildStatement::ModuleInstantiation {
                module_instantiation,
            } => self.process_module_instantiation(module_instantiation),
        }
    }

    fn append_instance(&mut self, instance: ModuleInstance) {
        let tree = Rc::new(ModuleInstanceTree {
            instance,
            children: RefCell::new(vec![]),
        });

        if let Some(last) = self.stack.last_mut() {
            last.children.borrow_mut().push(tree.clone());
        } else {
            // empty stack we need to push to results
            self.results.push(tree.clone());
        }

        self.stack.push(tree);
    }
}

pub fn openscad_interpret(statements: Vec<StatementWithPosition>) -> InterpreterResults {
    let it = Interpreter::new();
    it.interpret(statements)
}

#[cfg(test)]
mod tests {
    use crate::{parser::openscad_parse, tokenizer::openscad_tokenize};

    use super::*;

    #[test]
    fn test_binary_expression() {
        let result = openscad_parse(openscad_tokenize("cube(20 - 0.1);"));
        let result = openscad_interpret(result.statements);

        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
        assert_eq!(1, result.trees.len());
    }

    #[test]
    fn test_unary_expression() {
        let result = openscad_parse(openscad_tokenize("cube(-20);"));
        let result = openscad_interpret(result.statements);

        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
        assert_eq!(1, result.trees.len());
    }
}
