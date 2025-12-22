pub mod expr;
pub mod functions;
pub mod modules;
#[cfg(test)]
pub mod tests;

use std::{cell::RefCell, collections::HashMap, sync::Arc};

use rand_mt::Mt64;
use rust_raytracer_core::{
    Camera, CameraBuilder, Color, Node, SceneData, Vector3,
    material::{Dielectric, Lambertian, Material, Metal},
    object::{BoundingVolumeHierarchy, Group},
};

use crate::{
    parser::{
        CallArgument, CallArgumentWithPosition, DeclArgument, DeclArgumentWithPosition,
        ExprWithPosition, Statement, StatementWithPosition,
    },
    value::{Value, ValueConversionError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Module {
    Camera,

    // flow control
    For,

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

    // other
    Echo,
}

#[derive(Debug)]
pub struct ModuleInstance {
    pub module: Module,
    pub arguments: Vec<ModuleArgument>,
}

#[derive(Debug)]
pub enum ModuleArgument {
    Positional(Value),
    NamedArgument { name: String, value: Value },
}

#[derive(Debug, PartialEq)]
pub struct InterpreterError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

impl From<ValueConversionError> for InterpreterError {
    fn from(value: ValueConversionError) -> Self {
        todo!("From<ValueConversionError> {value:?}");
    }
}

type Result<T> = std::result::Result<T, InterpreterError>;

#[derive(Debug)]
pub struct InterpreterResults {
    pub scene_data: SceneData,
    pub output: String,
    pub errors: Vec<InterpreterError>,
}

#[derive(Debug)]
struct Function {
    pub arguments: Vec<DeclArgumentWithPosition>,
    pub expr: ExprWithPosition,
}

struct Scope {
    variables: RefCell<Vec<HashMap<String, Value>>>,
}

impl Drop for Scope {
    fn drop(&mut self) {
        self.variables.borrow_mut().pop();
    }
}

impl Function {
    pub fn get_argument_names(&self) -> Vec<String> {
        self.arguments
            .iter()
            .map(|arg| match &arg.item {
                DeclArgument::WithDefault {
                    identifier,
                    default_expr: _,
                } => identifier.to_owned(),
                DeclArgument::Identifier { identifier } => identifier.to_owned(),
            })
            .collect()
    }
}

struct Interpreter {
    _modules: HashMap<String, Module>,

    camera: Option<Arc<Camera>>,
    world: Vec<Arc<dyn Node>>,
    lights: Vec<Arc<dyn Node>>,
    material_stack: Vec<Arc<dyn Material>>,
    variables: RefCell<Vec<HashMap<String, Value>>>,
    functions: HashMap<String, Function>,
    output: String,
    rng: Mt64,
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
            _modules: HashMap::new(),
            variables: RefCell::new(vec![variables]),
            functions: HashMap::new(),
            camera: None,
            world: vec![],
            lights: vec![],
            material_stack: vec![],
            output: String::new(),
            rng: Mt64::new_unseeded(),
        }
    }

    fn interpret(mut self, statements: Vec<StatementWithPosition>) -> InterpreterResults {
        for statement in statements {
            if let Err(err) = self.process_statement(&statement) {
                todo!("error {err:?}");
            }
        }

        let camera = if let Some(camera) = self.camera {
            camera
        } else {
            let mut camera_builder = CameraBuilder::new();
            camera_builder.aspect_ratio = 1.0;
            camera_builder.image_width = 600;
            camera_builder.samples_per_pixel = 10;
            camera_builder.max_depth = 50;
            camera_builder.defocus_angle = 0.0;
            camera_builder.background = Color::new(0.7, 0.8, 1.0);
            camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
            camera_builder.look_from = Vector3::new(-50.0, 70.0, -50.0);
            camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
            Arc::new(camera_builder.build())
        };

        let scene_data = SceneData {
            camera,
            world: Arc::new(BoundingVolumeHierarchy::new(&self.world)),
            lights: if self.lights.is_empty() {
                None
            } else {
                Some(Arc::new(BoundingVolumeHierarchy::new(&self.lights)))
            },
        };

        InterpreterResults {
            scene_data,
            output: self.output,
            errors: vec![], // TODO
        }
    }

    fn process_statement(
        &mut self,
        statement: &StatementWithPosition,
    ) -> Result<Option<Arc<dyn Node>>> {
        match &statement.item {
            Statement::Empty => Ok(None),
            Statement::ModuleInstantiation {
                module_id,
                call_arguments,
                child_statements,
            } => self.process_module_instantiation(module_id, call_arguments, child_statements),
            Statement::Assignment { identifier, expr } => {
                self.process_assignment(identifier, expr).map(|_| None)
            }
            Statement::Include { filename } => self.process_include(filename),
            Statement::FunctionDecl {
                function_name,
                arguments,
                expr,
            } => self
                .process_function_decl(function_name, arguments, expr)
                .map(|_| None),
            Statement::If {
                expr,
                true_statements,
                false_statements,
            } => self.process_if(expr, true_statements, false_statements),
        }
    }

    fn current_material(&self) -> Arc<dyn Material> {
        if let Some(mat) = self.material_stack.last() {
            mat.clone()
        } else {
            Arc::new(Lambertian::new_from_color(Color::new(0.99, 0.85, 0.26)))
        }
    }

    fn expr_to_string(&mut self, expr: &ExprWithPosition) -> Result<String> {
        let value = self.expr_to_value(expr)?;
        Ok(format!("{value}"))
    }

    fn create_color(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "alpha"], arguments)?;

        if let Some(arg) = arguments.get("alpha") {
            todo!("handle alpha {arg:?}");
        }

        if let Some(arg) = arguments.get("c") {
            let color = arg.to_color()?;
            return Ok(Arc::new(Lambertian::new_from_color(color)));
        }

        todo!("missing arg");
    }

    fn create_lambertian(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "t"], arguments)?;

        if let Some(arg) = arguments.get("c") {
            let color = arg.to_color()?;
            Ok(Arc::new(Lambertian::new_from_color(color)))
        } else if let Some(arg) = arguments.get("t") {
            match arg {
                Value::Texture(texture) => Ok(Arc::new(Lambertian::new(texture.clone()))),
                _ => todo!("unhandled {arg:?}"),
            }
        } else {
            todo!("missing arg");
        }
    }

    fn create_dielectric(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["n"], arguments)?;

        if let Some(arg) = arguments.get("n") {
            let refraction_index = arg.to_number()?;
            Ok(Arc::new(Dielectric::new(refraction_index)))
        } else {
            todo!("missing arg");
        }
    }

    fn create_metal(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "fuzz"], arguments)?;

        let mut color = Color::WHITE;
        let mut fuzz = 0.2;

        if let Some(arg) = arguments.get("c") {
            color = arg.to_color()?;
        }

        if let Some(arg) = arguments.get("fuzz") {
            fuzz = arg.to_number()?;
        }

        Ok(Arc::new(Metal::new(color, fuzz)))
    }

    fn process_for_loop(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_statements: &[StatementWithPosition],
    ) -> Result<()> {
        if arguments.len() != 1 {
            todo!("for loop should only have one argument");
        }

        let arg = &arguments[0];
        let (name, value) = match &arg.item {
            CallArgument::Expr { expr } => {
                todo!("for loop should have named argument {expr:?}")
            }
            CallArgument::NamedArgument { identifier, expr } => {
                (identifier, self.expr_to_value(expr)?)
            }
        };

        let (start, end, increment) = match value {
            Value::Range {
                start,
                end,
                increment,
            } => (start, end, increment),
            other => todo!("for loop should have range argument {other:?}"),
        };

        let start = start.to_number()?;
        let end = end.to_number()?;
        let increment = if let Some(increment) = increment {
            increment.to_number()?
        } else {
            1.0
        };

        if end >= start && increment <= 0.0 {
            todo!("increment should be greater than 0");
        } else if end < start && increment >= 0.0 {
            todo!("increment should be less than 0");
        }

        let mut i = start;
        loop {
            if (end >= start && i >= end) || (end < start && i <= end) {
                break;
            }

            self.set_variable(name, Value::Number(i));
            self.process_child_statements(child_statements)?;

            i += increment;
        }

        Ok(())
    }

    fn process_child_statements(
        &mut self,
        child_statements: &[StatementWithPosition],
    ) -> Result<Option<Arc<dyn Node>>> {
        if child_statements.is_empty() {
            return Ok(None);
        }

        let mut nodes = vec![];
        for statement in child_statements {
            if let Some(node) = self.process_statement(statement)? {
                nodes.push(node);
            }
        }
        Ok(Some(Arc::new(Group::from_list(&nodes))))
    }

    fn process_assignment(&mut self, identifier: &str, expr: &ExprWithPosition) -> Result<()> {
        let value = self.expr_to_value(expr)?;

        if identifier.starts_with("$") {
            match value {
                Value::Number(_) => {}
                _ => todo!("expected number but found {value:?}"),
            }
        }

        self.set_variable(identifier, value);
        Ok(())
    }

    fn process_include(&self, filename: &str) -> Result<Option<Arc<dyn Node>>> {
        if filename.ends_with("ray_trace.scad") {
            return Ok(None);
        }

        todo!("include {filename}")
    }

    fn convert_args(
        &mut self,
        arg_names: &[&str],
        arguments: &[CallArgumentWithPosition],
    ) -> Result<HashMap<String, Value>> {
        let mut results: HashMap<String, Value> = HashMap::new();

        let mut found_named_arg = false;
        for (pos, arg) in arguments.iter().enumerate() {
            match &arg.item {
                CallArgument::Expr { expr } => {
                    if found_named_arg {
                        todo!("add error, no positional args after named arg {pos}");
                    }
                    if let Some(arg_name) = arg_names.get(pos) {
                        let value = self.expr_to_value(expr)?;
                        results.insert(arg_name.to_string(), value);
                    } else {
                        todo!(
                            "arg past end of list {pos} {arg_names:?} {}",
                            arguments.len()
                        );
                    }
                }
                CallArgument::NamedArgument { identifier, expr } => {
                    found_named_arg = true;
                    if arg_names.contains(&identifier.as_str()) {
                        let value = self.expr_to_value(expr)?;
                        results.insert(identifier.to_string(), value);
                    } else {
                        todo!("unknown arg name: {identifier}");
                    }
                }
            }
        }

        Ok(results)
    }

    fn evaluate_identifier(&self, name: &str) -> Result<Value> {
        if let Some(v) = self.get_variable(name) {
            Ok(v.clone())
        } else {
            todo!("missing variable {name}");
        }
    }

    fn evaluate_index(
        &mut self,
        lhs: &ExprWithPosition,
        index: &ExprWithPosition,
    ) -> Result<Value> {
        let lhs = self.expr_to_value(lhs)?;
        let index = self.expr_to_value(index)?.to_u64()? as usize;

        let value: Value = match &lhs {
            Value::Number(index) => todo!("evaluate_index {lhs:?} {index:?}"),
            Value::Vector { items } => {
                if let Some(item) = items.get(index) {
                    item.clone()
                } else {
                    todo!("index out of range");
                }
            }
            Value::Boolean(b) => todo!("evaluate_index {lhs:?} {b}"),
            Value::String(str) => todo!("evaluate_index {lhs:?} {str}"),
            Value::Texture(texture) => todo!("evaluate_index {lhs:?} {texture:?}"),
            Value::Range {
                start,
                end,
                increment,
            } => todo!("evaluate_index {lhs:?} {start:?} {end:?} {increment:?}"),
        };

        Ok(value)
    }

    fn process_function_decl(
        &mut self,
        function_name: &str,
        arguments: &[DeclArgumentWithPosition],
        expr: &ExprWithPosition,
    ) -> Result<()> {
        self.functions.insert(
            function_name.to_owned(),
            Function {
                arguments: arguments.to_vec(),
                expr: expr.clone(),
            },
        );
        Ok(())
    }

    fn set_variable(&self, name: &str, value: Value) {
        let mut variables = self.variables.borrow_mut();
        if let Some(scope) = variables.last_mut() {
            scope.insert(name.to_owned(), value);
        } else {
            let mut scope = HashMap::new();
            scope.insert(name.to_owned(), value);
            variables.push(scope);
        }
    }

    fn get_variable(&self, name: &str) -> Option<Value> {
        let variables = self.variables.borrow();
        for scope in variables.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    fn create_scope(&mut self) -> Scope {
        self.variables.borrow_mut().push(HashMap::new());
        Scope {
            variables: self.variables.clone(),
        }
    }

    fn process_if(
        &mut self,
        expr: &ExprWithPosition,
        true_statements: &[StatementWithPosition],
        false_statements: &[StatementWithPosition],
    ) -> Result<Option<Arc<dyn Node>>> {
        let v = self.expr_to_value(expr)?;
        if v.is_truthy() {
            self.process_child_statements(true_statements)
        } else {
            self.process_child_statements(false_statements)
        }
    }
}

pub fn openscad_interpret(statements: Vec<StatementWithPosition>) -> InterpreterResults {
    let it = Interpreter::new();
    it.interpret(statements)
}
