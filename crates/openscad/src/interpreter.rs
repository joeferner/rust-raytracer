use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use rust_raytracer_core::{
    Color, Vector3,
    texture::{CheckerTexture, SolidColor, Texture},
};

use crate::parser::{
    BinaryOperator, CallArgument, CallArgumentWithPosition, ChildStatement,
    ChildStatementWithPosition, Expr, ExprWithPosition, ModuleId, ModuleInstantiation,
    ModuleInstantiationWithPosition, SingleModuleInstantiation,
    SingleModuleInstantiationWithPosition, Statement, StatementWithPosition, UnaryOperator,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Module {
    Camera,

    // 3d
    Cube,
    Sphere,
    Cylinder,

    // transformations
    Translate,
    Rotate,
    Scale,
    Color,
    Lambertian,
    Dielectric,
    Metal,
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
    Positional(Value),
    NamedArgument { name: String, value: Value },
}

#[derive(Debug)]
pub enum Value {
    Number(f64),
    Vector { items: Vec<Value> },
    True,
    False,
    Texture(Arc<dyn Texture>),
}

impl Value {
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(value) => Some(*value),
            _ => todo!(),
        }
    }

    pub fn to_vector3(&self) -> Option<Vector3> {
        match self {
            Value::Number(value) => Some(Vector3::new(-*value, *value, *value)),
            Value::Vector { items } => Self::values_to_vector3(items),
            _ => todo!(),
        }
    }

    pub fn to_color(&self) -> Option<Color> {
        match self {
            Value::Number(value) => Some(Color::new(*value, *value, *value)),
            Value::Vector { items } => Self::values_to_color(items),
            _ => todo!(),
        }
    }

    pub fn to_boolean(&self) -> Option<bool> {
        match self {
            Value::True => Some(true),
            Value::False => Some(false),
            _ => todo!(),
        }
    }

    pub fn values_to_vector3(items: &[Value]) -> Option<Vector3> {
        if items.len() != 3 {
            todo!();
        }

        let x = if let Value::Number(x) = items[0] {
            x
        } else {
            todo!();
        };

        let y = if let Value::Number(y) = items[1] {
            y
        } else {
            todo!();
        };

        let z = if let Value::Number(z) = items[2] {
            z
        } else {
            todo!();
        };

        // OpenSCAD x,y,z is different than ours so flip z and y
        Some(Vector3::new(-x, z, y))
    }

    pub fn values_to_color(items: &[Value]) -> Option<Color> {
        if items.len() != 3 {
            todo!();
        }

        let r = if let Value::Number(r) = items[0] {
            r
        } else {
            todo!();
        };

        let g = if let Value::Number(g) = items[1] {
            g
        } else {
            todo!();
        };

        let b = if let Value::Number(b) = items[2] {
            b
        } else {
            todo!();
        };

        Some(Color::new(r, g, b))
    }
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
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        let variables = {
            let mut variables = HashMap::new();

            variables.insert("$fn".to_owned(), Value::Number(0.0));
            variables.insert("$fs".to_owned(), Value::Number(2.0));
            variables.insert("$fa".to_owned(), Value::Number(12.0));
            variables.insert("$t".to_owned(), Value::Number(0.0));

            variables
        };

        Self {
            modules: HashMap::new(),
            stack: vec![],
            results: vec![],
            variables,
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
            Statement::Assignment { identifier, expr } => self.process_assignment(identifier, expr),
            Statement::Include { filename } => self.process_include(filename),
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
                        self.append_instance(ModuleInstance {
                            module: *module,
                            arguments: self.process_call_arguments(call_arguments),
                        });
                    } else {
                        todo!("handle unknown module \"{identifier}\"");
                    }
                }
                built_in => {
                    let module = match built_in {
                        ModuleId::Cube => Module::Cube,
                        ModuleId::Sphere => Module::Sphere,
                        ModuleId::Cylinder => Module::Cylinder,
                        ModuleId::Translate => Module::Translate,
                        ModuleId::Rotate => Module::Rotate,
                        ModuleId::Scale => Module::Scale,
                        ModuleId::Color => Module::Color,
                        ModuleId::Lambertian => Module::Lambertian,
                        ModuleId::Dielectric => Module::Dielectric,
                        ModuleId::Metal => Module::Metal,
                        ModuleId::Camera => Module::Camera,
                        ModuleId::For => todo!("already handled"),
                        ModuleId::Identifier(_) => todo!("already handled"),
                    };
                    self.append_instance(ModuleInstance {
                        module,
                        arguments: self.process_call_arguments(call_arguments),
                    });
                }
            },
        }
    }

    fn expr_to_value(&self, expr: &ExprWithPosition) -> Value {
        match &expr.item {
            Expr::Number(number) => Value::Number(*number),
            Expr::Vector { items } => Value::Vector {
                items: items.iter().map(|v| self.expr_to_value(v)).collect(),
            },
            Expr::True => Value::True,
            Expr::False => Value::False,
            Expr::Binary { operator, lhs, rhs } => {
                self.evaluate_binary_expression(operator, lhs, rhs)
            }
            Expr::Unary { operator, rhs } => self.evaluate_unary_expression(operator, rhs),
            Expr::FunctionCall { name, arguments } => self.evaluate_function_call(name, arguments),
        }
    }

    fn evaluate_binary_expression(
        &self,
        operator: &BinaryOperator,
        lhs: &ExprWithPosition,
        rhs: &ExprWithPosition,
    ) -> Value {
        let left = self.expr_to_value(lhs);
        let right = self.expr_to_value(rhs);

        fn eval_number_number(operator: &BinaryOperator, left: f64, right: f64) -> Value {
            match operator {
                BinaryOperator::Minus => Value::Number(left - right),
                BinaryOperator::Divide => Value::Number(left / right),
            }
        }

        fn eval_vector_number(operator: &BinaryOperator, left: Vec<Value>, right: f64) -> Value {
            Value::Vector {
                items: left
                    .iter()
                    .map(|item| match item {
                        Value::Number(v) => match operator {
                            BinaryOperator::Minus => Value::Number(v - right),
                            BinaryOperator::Divide => Value::Number(v / right),
                        },
                        Value::Vector { items } => todo!("items {items:?}"),
                        Value::True => todo!("true"),
                        Value::False => todo!("false"),
                        Value::Texture(texture) => todo!("texture {texture:?}"),
                    })
                    .collect(),
            }
        }

        match left {
            Value::Number(left) => match right {
                Value::Number(right) => eval_number_number(operator, left, right),
                Value::Vector { items } => todo!("{left:?} {operator:?} {items:?}"),
                Value::True => todo!("{left:?} {operator:?} True"),
                Value::False => todo!("{left:?} {operator:?} False"),
                Value::Texture(texture) => todo!("{left:?} {operator:?} {texture:?}"),
            },
            Value::Vector { items } => match right {
                Value::Number(right) => eval_vector_number(operator, items, right),
                Value::Vector { items } => todo!("{items:?} {operator:?} {items:?}"),
                Value::True => todo!("{items:?} {operator:?} true"),
                Value::False => todo!("{items:?} {operator:?} false"),
                Value::Texture(texture) => todo!("{items:?} {operator:?} {texture:?}"),
            },
            Value::True => todo!("true"),
            Value::False => todo!("false"),
            Value::Texture(texture) => todo!("texture {texture:?}"),
        }
    }

    fn evaluate_unary_expression(&self, operator: &UnaryOperator, rhs: &ExprWithPosition) -> Value {
        let right = self.expr_to_value(rhs);

        if let Value::Number(right) = right {
            match operator {
                UnaryOperator::Minus => Value::Number(-right),
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
                CallArgument::Expr { expr } => {
                    results.push(ModuleArgument::Positional(self.expr_to_value(expr)))
                }
                CallArgument::NamedArgument { identifier, expr } => {
                    let value = self.expr_to_value(expr);
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

    fn process_assignment(&mut self, identifier: String, expr: ExprWithPosition) {
        let value = self.expr_to_value(&expr);

        if identifier.starts_with("$") {
            match value {
                Value::Number(_) => {}
                _ => todo!("expected number but found {value:?}"),
            }
        }

        self.variables.insert(identifier, value);
    }

    fn process_include(&self, filename: String) {
        if filename.ends_with("ray_trace.scad") {
            return;
        }

        todo!("include {filename}")
    }

    fn evaluate_function_call(&self, name: &str, arguments: &[CallArgumentWithPosition]) -> Value {
        if name == "checker" {
            self.evaluate_checker_function_call(arguments)
        } else {
            todo!("evaluate_function_call {name} {arguments:?}")
        }
    }

    fn evaluate_checker_function_call(&self, arguments: &[CallArgumentWithPosition]) -> Value {
        let arguments = self.convert_args(&["scale", "even", "odd"], &arguments);

        let mut scale: f64 = 0.0;
        let mut even: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0)));
        let mut odd: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0)));

        if let Some(arg) = arguments.get("scale") {
            if let Some(value) = arg.to_number() {
                scale = value;
            }
        }

        if let Some(arg) = arguments.get("even") {
            if let Some(value) = arg.to_color() {
                even = Arc::new(SolidColor::new(value));
            }
        }

        if let Some(arg) = arguments.get("odd") {
            if let Some(value) = arg.to_color() {
                odd = Arc::new(SolidColor::new(value));
            }
        }

        Value::Texture(Arc::new(CheckerTexture::new(scale, even, odd)))
    }

    fn convert_args(
        &self,
        arg_names: &[&str],
        arguments: &[CallArgumentWithPosition],
    ) -> HashMap<String, Value> {
        let mut results: HashMap<String, Value> = HashMap::new();

        let mut found_named_arg = false;
        for (pos, arg) in arguments.iter().enumerate() {
            match &arg.item {
                CallArgument::Expr { expr } => {
                    if found_named_arg {
                        todo!("add error, no positional args after named arg {pos}");
                    }
                    if let Some(arg_name) = arg_names.get(pos) {
                        let value = self.expr_to_value(&expr);
                        results.insert(arg_name.to_string(), value);
                    } else {
                        todo!("arg past end of list {pos}");
                    }
                }
                CallArgument::NamedArgument { identifier, expr } => {
                    found_named_arg = true;
                    if arg_names.contains(&identifier.as_str()) {
                        let value = self.expr_to_value(&expr);
                        results.insert(identifier.to_string(), value);
                    } else {
                        todo!("unknown arg name: {identifier}");
                    }
                }
            }
        }

        results
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

    #[test]
    fn test_set_fa() {
        let result = openscad_parse(openscad_tokenize("$fa = 1;"));
        let result = openscad_interpret(result.statements);

        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
        assert_eq!(0, result.trees.len());
    }
}
