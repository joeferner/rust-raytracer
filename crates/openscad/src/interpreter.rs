use std::{cell::RefCell, collections::HashMap, mem::swap, rc::Rc, sync::Arc};

use rand_mt::Mt64;
use rust_raytracer_core::{
    Camera, CameraBuilder, Color, Node, SceneData, Vector3,
    material::{Dielectric, Lambertian, Material, Metal},
    object::{
        BoundingVolumeHierarchy, BoxPrimitive, ConeFrustum, Group, Rotate, Scale, Sphere, Translate,
    },
    texture::{CheckerTexture, SolidColor, Texture},
};

use crate::{
    parser::{
        BinaryOperator, CallArgument, CallArgumentWithPosition, ChildStatement,
        ChildStatementWithPosition, Expr, ExprWithPosition, ModuleId, ModuleInstantiation,
        ModuleInstantiationWithPosition, SingleModuleInstantiation, Statement,
        StatementWithPosition, UnaryOperator,
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

struct Interpreter {
    _modules: HashMap<String, Module>,
    stack: Vec<Rc<ModuleInstance>>,

    camera: Option<Arc<Camera>>,
    world: Vec<Arc<dyn Node>>,
    lights: Vec<Arc<dyn Node>>,
    material_stack: Vec<Arc<dyn Material>>,
    variables: HashMap<String, Value>,
    output: String,
    rng: RefCell<Mt64>,
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
            stack: vec![],
            variables,
            camera: None,
            world: vec![],
            lights: vec![],
            material_stack: vec![],
            output: String::new(),
            rng: RefCell::new(Mt64::new_unseeded()),
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
                module_instantiation,
            } => self.process_module_instantiation(module_instantiation),
            Statement::Assignment { identifier, expr } => {
                self.process_assignment(identifier, expr).map(|_| None)
            }
            Statement::Include { filename } => self.process_include(filename),
        }
    }

    fn process_module_instantiation(
        &mut self,
        module_instantiation: &ModuleInstantiationWithPosition,
    ) -> Result<Option<Arc<dyn Node>>> {
        match &module_instantiation.item {
            ModuleInstantiation::SingleModuleInstantiation {
                single_module_instantiation,
                child_statement,
            } => match &single_module_instantiation.item {
                SingleModuleInstantiation::Module {
                    module_id,
                    call_arguments: arguments,
                } => {
                    if module_id.item == ModuleId::Color {
                        let color = self.create_color(arguments)?;
                        self.material_stack.push(color);
                    } else if module_id.item == ModuleId::Lambertian {
                        let color = self.create_lambertian(arguments)?;
                        self.material_stack.push(color);
                    } else if module_id.item == ModuleId::Dielectric {
                        let color = self.create_dielectric(arguments)?;
                        self.material_stack.push(color);
                    } else if module_id.item == ModuleId::Metal {
                        let color = self.create_metal(arguments)?;
                        self.material_stack.push(color);
                    } else if module_id.item == ModuleId::For {
                        return self
                            .process_for_loop(arguments, child_statement)
                            .map(|_| None);
                    }

                    let child = self.process_child_statement(child_statement)?;

                    match &module_id.item {
                        ModuleId::Cube => self.create_cube(arguments, child).map(Some),
                        ModuleId::Sphere => self.create_sphere(arguments, child).map(Some),
                        ModuleId::Cylinder => self.create_cylinder(arguments, child).map(Some),
                        ModuleId::Translate => self.create_translate(arguments, child).map(Some),
                        ModuleId::Rotate => self.create_rotate(arguments, child).map(Some),
                        ModuleId::Scale => self.create_scale(arguments, child).map(Some),
                        ModuleId::Camera => self.create_camera(arguments, child).map(|_| None),
                        ModuleId::Color
                        | ModuleId::Lambertian
                        | ModuleId::Dielectric
                        | ModuleId::Metal => {
                            self.material_stack.pop();
                            Ok(None)
                        }
                        ModuleId::For => panic!("already handled"),
                        ModuleId::Echo => self.evaluate_echo(arguments, child).map(|_| None),
                        ModuleId::Identifier(identifier) => {
                            todo!("ModuleId::Identifier {identifier}")
                        }
                    }
                }
            },
        }
    }

    fn current_material(&self) -> Arc<dyn Material> {
        if let Some(mat) = self.material_stack.last() {
            mat.clone()
        } else {
            Arc::new(Lambertian::new_from_color(Color::new(0.99, 0.85, 0.26)))
        }
    }

    fn create_cube(
        &self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child.is_some() {
            todo!("should not have children");
        }

        let mut size = Vector3::new(0.0, 0.0, 0.0);
        let mut center = false;

        let arguments = self.convert_args(&["size", "center"], arguments)?;

        if let Some(arg) = arguments.get("size") {
            size = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("center") {
            center = arg.to_boolean()?;
        }

        let mut a = Vector3::new(0.0, 0.0, 0.0);
        let mut b = size;
        if center {
            a = a - (size / 2.0);
            b = b - (size / 2.0);
        }

        Ok(Arc::new(BoxPrimitive::new(a, b, self.current_material())))
    }

    fn create_sphere(
        &self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child.is_some() {
            todo!("should not have children");
        }

        let mut radius = 1.0;

        let arguments = self.convert_args(&["r", "d"], arguments)?;

        if let Some(arg) = arguments.get("r") {
            radius = arg.to_number()?;
        } else if let Some(arg) = arguments.get("d") {
            radius = arg.to_number()? / 2.0;
        }

        Ok(Arc::new(Sphere::new(
            Vector3::ZERO,
            radius,
            self.current_material(),
        )))
    }

    fn create_cylinder(
        &self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child.is_some() {
            todo!("should not have children");
        }

        let mut height = 1.0;
        let mut radius1 = 1.0;
        let mut radius2 = 1.0;
        let mut center = false;

        let arguments = self.convert_args(
            &["h", "r1", "r2", "center", "r", "d", "d1", "d2"],
            arguments,
        )?;

        if let Some(arg) = arguments.get("h") {
            height = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("r1") {
            radius1 = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("r2") {
            radius2 = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("r") {
            let r = arg.to_number()?;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("d1") {
            radius1 = arg.to_number()? / 2.0;
        }

        if let Some(arg) = arguments.get("d2") {
            radius2 = arg.to_number()? / 2.0;
        }

        if let Some(arg) = arguments.get("d") {
            let r = arg.to_number()? / 2.0;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("center") {
            center = arg.to_boolean()?;
        }

        let mut center_vec = Vector3::new(0.0, 0.0, 0.0);
        if center {
            center_vec.y -= height / 2.0;
        }

        Ok(Arc::new(ConeFrustum::new(
            center_vec,
            height,
            radius1,
            radius2,
            self.current_material(),
        )))
    }

    fn create_translate(
        &self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        let child = child.unwrap_or_else(|| todo!("should have children"));

        let mut offset = Vector3::new(0.0, 0.0, 0.0);

        let arguments = self.convert_args(&["v"], arguments)?;

        if let Some(arg) = arguments.get("v") {
            offset = arg.to_vector3()?;
        }

        let translate = Translate::new(child, offset);
        Ok(Arc::new(translate))
    }

    fn create_rotate(
        &self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        let child = child.unwrap_or_else(|| todo!("should have children"));

        let arguments = self.convert_args(&["a", "v"], arguments)?;

        if let Some(arg) = arguments.get("a") {
            match arg {
                Value::Number(_deg_a) => todo!(),
                Value::Vector { items } => {
                    let a = Value::values_to_vector3(items)?;
                    let mut result: Arc<dyn Node> = child;
                    if a.x != 0.0 {
                        result = Arc::new(Rotate::rotate_x(result, a.x));
                    }
                    if a.y != 0.0 {
                        result = Arc::new(Rotate::rotate_y(result, a.y));
                    }
                    if a.z != 0.0 {
                        result = Arc::new(Rotate::rotate_z(result, a.z));
                    }
                    return Ok(result);
                }
                _ => todo!("add error"),
            }
        }

        if let Some(_arg) = arguments.get("v") {
            todo!();
        }

        todo!();
    }

    fn create_scale(
        &self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        let child = child.unwrap_or_else(|| todo!("should have children"));

        let arguments = self.convert_args(&["v"], arguments)?;

        if let Some(arg) = arguments.get("v") {
            let v = arg.to_vector3()?;
            return Ok(Arc::new(Scale::new(child, v.x, v.y, v.z)));
        }

        todo!("missing arg");
    }

    fn create_camera(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<()> {
        if child.is_some() {
            todo!("should not have children");
        }

        let arguments = self.convert_args(
            &[
                "image_width",
                "image_height",
                "samples_per_pixel",
                "max_depth",
                "vertical_fov",
                "look_from",
                "look_at",
                "up",
                "defocus_angle",
                "focus_distance",
                "background",
                "aspect_ratio",
            ],
            arguments,
        )?;

        let mut camera_builder = CameraBuilder::new();

        let mut seen_aspect_ratio = false;
        let mut seen_image_width = false;

        if let Some(arg) = arguments.get("aspect_ratio") {
            camera_builder.aspect_ratio = arg.to_number()?;
            seen_aspect_ratio = true;
        }

        if let Some(arg) = arguments.get("image_width") {
            camera_builder.image_width = arg.to_number()? as u32;
            seen_image_width = true;
        }

        if let Some(arg) = arguments.get("samples_per_pixel") {
            camera_builder.samples_per_pixel = arg.to_number()? as u32;
        }

        if let Some(arg) = arguments.get("max_depth") {
            camera_builder.max_depth = arg.to_number()? as u32;
        }

        if let Some(arg) = arguments.get("vertical_fov") {
            camera_builder.vertical_fov = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("defocus_angle") {
            camera_builder.defocus_angle = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("focus_distance") {
            camera_builder.focus_distance = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("image_height") {
            let height = arg.to_number()?;
            if seen_image_width {
                camera_builder.aspect_ratio = camera_builder.image_width as f64 / height;
            } else if seen_aspect_ratio {
                camera_builder.image_width = (camera_builder.aspect_ratio * height) as u32;
            } else {
                camera_builder.aspect_ratio = 1.0;
                camera_builder.image_width = height as u32;
            }
        }

        if let Some(arg) = arguments.get("look_from") {
            camera_builder.look_from = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("look_at") {
            camera_builder.look_at = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("up") {
            camera_builder.up = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("background") {
            camera_builder.background = arg.to_color()?;
        }

        self.camera = Some(Arc::new(camera_builder.build()));

        Ok(())
    }

    fn evaluate_echo(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<()> {
        if child.is_some() {
            todo!("should not have children");
        }

        let mut output = String::new();
        for (i, arg) in arguments.iter().enumerate() {
            if i > 0 {
                output += ", ";
            }
            match &arg.item {
                CallArgument::Expr { expr } => output += &self.expr_to_string(expr)?,
                CallArgument::NamedArgument { identifier, expr } => {
                    output += &format!("{identifier} = {}", self.expr_to_string(expr)?);
                }
            };
        }

        self.output += &format!("{output}\n");

        Ok(())
    }

    fn expr_to_string(&self, expr: &ExprWithPosition) -> Result<String> {
        let value = self.expr_to_value(expr)?;
        Ok(format!("{value}"))
    }

    fn create_color(&self, arguments: &[CallArgumentWithPosition]) -> Result<Arc<dyn Material>> {
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
        &self,
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
        &self,
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

    fn create_metal(&self, arguments: &[CallArgumentWithPosition]) -> Result<Arc<dyn Material>> {
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
        child_statement: &ChildStatementWithPosition,
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

            self.variables.insert(name.to_owned(), Value::Number(i));
            self.process_child_statement(child_statement)?;

            i += increment;
        }

        Ok(())
    }

    fn expr_to_value(&self, expr: &ExprWithPosition) -> Result<Value> {
        Ok(match &expr.item {
            Expr::Number(number) => Value::Number(*number),
            Expr::Vector { items } => {
                let items: Result<Vec<Value>> =
                    items.iter().map(|v| self.expr_to_value(v)).collect();
                Value::Vector { items: items? }
            }
            Expr::True => Value::True,
            Expr::False => Value::False,
            Expr::Binary { operator, lhs, rhs } => {
                self.evaluate_binary_expression(operator, lhs, rhs)?
            }
            Expr::Unary { operator, rhs } => self.evaluate_unary_expression(operator, rhs)?,
            Expr::FunctionCall { name, arguments } => {
                self.evaluate_function_call(name, arguments)?
            }
            Expr::Range {
                start,
                end,
                increment,
            } => self.evaluate_range_expression(start, end, increment)?,
            Expr::Identifier { name } => self.evaluate_identifier(name)?,
            Expr::Index { lhs, index } => self.evaluate_index(lhs, index)?,
        })
    }

    fn evaluate_binary_expression(
        &self,
        operator: &BinaryOperator,
        lhs: &ExprWithPosition,
        rhs: &ExprWithPosition,
    ) -> Result<Value> {
        let left = self.expr_to_value(lhs)?;
        let right = self.expr_to_value(rhs)?;

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
                        Value::Range {
                            start,
                            end,
                            increment,
                        } => todo!("range: {start:?}, {end:?}, {increment:?}"),
                    })
                    .collect(),
            }
        }

        Ok(match left {
            Value::Number(left) => match right {
                Value::Number(right) => eval_number_number(operator, left, right),
                Value::Vector { items } => todo!("{left:?} {operator:?} {items:?}"),
                Value::True => todo!("{left:?} {operator:?} True"),
                Value::False => todo!("{left:?} {operator:?} False"),
                Value::Texture(texture) => todo!("{left:?} {operator:?} {texture:?}"),
                Value::Range {
                    start,
                    end,
                    increment,
                } => todo!("{left:?} {operator:?} range({start:?}, {end:?}, {increment:?})"),
            },
            Value::Vector { items } => match right {
                Value::Number(right) => eval_vector_number(operator, items, right),
                Value::Vector { items } => todo!("{items:?} {operator:?} {items:?}"),
                Value::True => todo!("{items:?} {operator:?} true"),
                Value::False => todo!("{items:?} {operator:?} false"),
                Value::Texture(texture) => todo!("{items:?} {operator:?} {texture:?}"),
                Value::Range {
                    start,
                    end,
                    increment,
                } => todo!("{items:?} {operator:?} range({start:?}, {end:?}, {increment:?})"),
            },
            Value::True => todo!("true"),
            Value::False => todo!("false"),
            Value::Texture(texture) => todo!("texture {texture:?}"),
            Value::Range {
                start,
                end,
                increment,
            } => todo!("range: {start:?}, {end:?}, {increment:?}"),
        })
    }

    fn evaluate_unary_expression(
        &self,
        operator: &UnaryOperator,
        rhs: &ExprWithPosition,
    ) -> Result<Value> {
        let right = self.expr_to_value(rhs)?;

        if let Value::Number(right) = right {
            match operator {
                UnaryOperator::Minus => Ok(Value::Number(-right)),
            }
        } else {
            todo!("{operator:?} {right:?}");
        }
    }

    fn process_child_statement(
        &mut self,
        child_statement: &ChildStatementWithPosition,
    ) -> Result<Option<Arc<dyn Node>>> {
        Ok(match &child_statement.item {
            ChildStatement::Empty => {
                self.stack.clear();
                None
            }
            ChildStatement::ModuleInstantiation {
                module_instantiation,
            } => self.process_module_instantiation(module_instantiation)?,
            ChildStatement::MultipleStatements { statements } => {
                let mut nodes = vec![];
                for statement in statements {
                    if let Some(node) = self.process_statement(statement.as_ref())? {
                        nodes.push(node);
                    }
                }
                Some(Arc::new(Group::from_list(&nodes)))
            }
        })
    }

    fn process_assignment(&mut self, identifier: &str, expr: &ExprWithPosition) -> Result<()> {
        let value = self.expr_to_value(expr)?;

        if identifier.starts_with("$") {
            match value {
                Value::Number(_) => {}
                _ => todo!("expected number but found {value:?}"),
            }
        }

        self.variables.insert(identifier.to_owned(), value);
        Ok(())
    }

    fn process_include(&self, filename: &str) -> Result<Option<Arc<dyn Node>>> {
        if filename.ends_with("ray_trace.scad") {
            return Ok(None);
        }

        todo!("include {filename}")
    }

    fn evaluate_function_call(
        &self,
        name: &str,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Value> {
        if name == "checker" {
            self.evaluate_checker_function_call(arguments)
        } else if name == "rands" {
            self.evaluate_rands_function_call(arguments)
        } else {
            todo!("evaluate_function_call {name} {arguments:?}")
        }
    }

    fn evaluate_rands_function_call(
        &self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Value> {
        let arguments = self.convert_args(
            &["min_value", "max_value", "value_count", "seed_value"],
            arguments,
        )?;

        let mut min_value = if let Some(arg) = arguments.get("min_value") {
            arg.to_number()?
        } else {
            todo!("min_value required");
        };

        let mut max_value = if let Some(arg) = arguments.get("max_value") {
            arg.to_number()?
        } else {
            todo!("max_value required");
        };

        let value_count = if let Some(arg) = arguments.get("value_count") {
            arg.to_u64()?
        } else {
            todo!("value_count required");
        };

        let seed_value = if let Some(arg) = arguments.get("seed_value") {
            Some(arg.to_number()?)
        } else {
            None
        };

        if max_value < min_value {
            swap(&mut min_value, &mut max_value);
        }

        let mut items = vec![];
        for _ in 0..value_count {
            let rand_value = if let Some(seed_value) = seed_value {
                todo!("rands with seed {seed_value}");
            } else {
                self.rng.borrow_mut().next_u64()
            };

            let normalized = rand_value as f64 / u64::MAX as f64;
            let v = min_value + normalized * (max_value - min_value);
            items.push(Value::Number(v));
        }
        Ok(Value::Vector { items })
    }

    fn evaluate_checker_function_call(
        &self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Value> {
        let arguments = self.convert_args(&["scale", "even", "odd"], arguments)?;

        let mut scale: f64 = 0.0;
        let mut even: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0)));
        let mut odd: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0)));

        if let Some(arg) = arguments.get("scale") {
            scale = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("even") {
            even = Arc::new(SolidColor::new(arg.to_color()?));
        }

        if let Some(arg) = arguments.get("odd") {
            odd = Arc::new(SolidColor::new(arg.to_color()?));
        }

        Ok(Value::Texture(Arc::new(CheckerTexture::new(
            scale, even, odd,
        ))))
    }

    fn convert_args(
        &self,
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
                        todo!("arg past end of list {pos}");
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

    fn evaluate_range_expression(
        &self,
        start: &ExprWithPosition,
        end: &ExprWithPosition,
        increment: &Option<Box<ExprWithPosition>>,
    ) -> Result<Value> {
        let start = Box::new(self.expr_to_value(start)?);
        let end = Box::new(self.expr_to_value(end)?);
        let increment = if let Some(increment) = increment {
            Some(Box::new(self.expr_to_value(increment)?))
        } else {
            None
        };

        Ok(Value::Range {
            start,
            end,
            increment,
        })
    }

    fn evaluate_identifier(&self, name: &str) -> Result<Value> {
        if let Some(v) = self.variables.get(name) {
            Ok(v.clone())
        } else {
            todo!("missing variable {name}");
        }
    }

    fn evaluate_index(&self, lhs: &ExprWithPosition, index: &ExprWithPosition) -> Result<Value> {
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
            Value::True => todo!("evaluate_index {lhs:?} true"),
            Value::False => todo!("evaluate_index {lhs:?} false"),
            Value::Texture(texture) => todo!("evaluate_index {lhs:?} {texture:?}"),
            Value::Range {
                start,
                end,
                increment,
            } => todo!("evaluate_index {lhs:?} {start:?} {end:?} {increment:?}"),
        };

        Ok(value)
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
    }

    #[test]
    fn test_unary_expression() {
        let result = openscad_parse(openscad_tokenize("cube(-20);"));
        let result = openscad_interpret(result.statements);

        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
    }

    #[test]
    fn test_set_fa() {
        let result = openscad_parse(openscad_tokenize("$fa = 1;"));
        let result = openscad_interpret(result.statements);

        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
    }

    #[test]
    fn test_for_loop() {
        let result = openscad_parse(openscad_tokenize(
            "
                for(a = [-1 : 1])
                    for(b = [0 : 2])
                        echo(a,b);
            ",
        ));
        let result = openscad_interpret(result.statements);

        assert_eq!(result.output, "-1, 0\n-1, 1\n0, 0\n0, 1\n");
    }

    #[test]
    fn test_rands() {
        let result = openscad_parse(openscad_tokenize("choose_mat = rands(0,1,1)[0];"));
        let result = openscad_interpret(result.statements);
        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
    }
}
