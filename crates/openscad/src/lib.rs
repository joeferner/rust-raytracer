pub mod parser;
pub mod tokenizer;

use std::{collections::HashMap, fs, sync::Arc};

use rust_raytracer_core::{Camera, Node, SceneData, object::BoundingVolumeHierarchy};

use crate::{
    parser::{
        CallArgument, CallArgumentWithPosition, ChildStatement, ChildStatementWithPosition, Expr,
        ModuleId, ModuleInstantiation, ModuleInstantiationWithPosition, SingleModuleInstantiation,
        SingleModuleInstantiationWithPosition, Statement, StatementWithPosition, openscad_parse,
    },
    tokenizer::openscad_tokenize,
};

#[derive(Debug, PartialEq)]
pub struct WithPosition<T: PartialEq> {
    pub item: T,
    pub start: usize,
    pub end: usize,
}

impl<T: PartialEq> WithPosition<T> {
    pub fn new(item: T, start: usize, end: usize) -> Self {
        Self { item, start, end }
    }
}

#[derive(Debug)]
pub enum OpenscadError {
    FileReadError(String, String),
}

pub fn openscad_file_to_scene_data(filename: &str) -> Result<SceneData, OpenscadError> {
    match fs::read_to_string(filename) {
        Ok(contents) => openscad_string_to_scene_data(&contents),
        Err(err) => Err(OpenscadError::FileReadError(
            filename.to_owned(),
            err.to_string(),
        )),
    }
}

struct ConvertState {
    camera: Option<Arc<Camera>>,
    world: Vec<Arc<dyn Node>>,
    lights: Vec<Arc<dyn Node>>,
    modules: HashMap<String, Module>,
    module_stack: Vec<ModuleInstance>,
}

#[derive(Debug, Clone, Copy)]
enum Module {
    Cube,
}

#[derive(Debug)]
struct ModuleInstance {
    module: Module,
    arguments: Vec<ModuleArgument>,
}

#[derive(Debug)]
enum ModuleArgument {
    Positional(ModuleArgumentValue),
}

#[derive(Debug)]
enum ModuleArgumentValue {
    Number(f64),
}

pub fn openscad_string_to_scene_data(input: &str) -> Result<SceneData, OpenscadError> {
    let tokens = openscad_tokenize(input);
    let tree = openscad_parse(tokens);

    if !tree.errors.is_empty() {
        todo!("{:?}", tree.errors);
    }

    let mut modules = HashMap::new();
    modules.insert("cube".to_string(), Module::Cube);

    let mut state = ConvertState {
        camera: None,
        world: vec![],
        lights: vec![],
        modules,
        module_stack: vec![],
    };

    for stmt in tree.statements {
        process_statement(&mut state, stmt)?;
    }

    Ok(SceneData {
        camera: state.camera.unwrap(),
        world: Arc::new(BoundingVolumeHierarchy::new(&state.world)),
        lights: if state.lights.is_empty() {
            None
        } else {
            Some(Arc::new(BoundingVolumeHierarchy::new(&state.lights)))
        },
    })
}

fn process_statement(
    state: &mut ConvertState,
    stmt: StatementWithPosition,
) -> Result<(), OpenscadError> {
    match stmt.item {
        Statement::Empty => Ok(()),
        Statement::ModuleInstantiation {
            module_instantiation,
        } => process_module_instantiation(state, module_instantiation),
    }
}

fn process_module_instantiation(
    state: &mut ConvertState,
    module_instantiation: ModuleInstantiationWithPosition,
) -> Result<(), OpenscadError> {
    match module_instantiation.item {
        ModuleInstantiation::SingleModuleInstantiation {
            single_module_instantiation,
            child_statement,
        } => {
            process_single_module_instantiation(state, single_module_instantiation)?;
            process_child_statement(state, child_statement)
        }
    }
}

fn process_single_module_instantiation(
    state: &mut ConvertState,
    single_module_instantiation: SingleModuleInstantiationWithPosition,
) -> Result<(), OpenscadError> {
    match single_module_instantiation.item {
        SingleModuleInstantiation::Module {
            module_id,
            call_arguments,
        } => match module_id.item {
            ModuleId::For => todo!(),
            ModuleId::Identifier(identifier) => {
                if let Some(module) = state.modules.get(&identifier) {
                    state.module_stack.push(ModuleInstance {
                        module: *module,
                        arguments: process_call_arguments(call_arguments)?,
                    });
                    Ok(())
                } else {
                    todo!("handle unknown module");
                }
            }
        },
    }
}

fn process_call_arguments(
    call_arguments: Vec<CallArgumentWithPosition>,
) -> Result<Vec<ModuleArgument>, OpenscadError> {
    let mut results: Vec<ModuleArgument> = vec![];

    for call_argument in call_arguments {
        match call_argument.item {
            CallArgument::Expr { expr } => match expr.item {
                Expr::Number(number) => results.push(ModuleArgument::Positional(
                    ModuleArgumentValue::Number(number),
                )),
            },
        }
    }

    Ok(results)
}

fn process_child_statement(
    _state: &mut ConvertState,
    child_statement: ChildStatementWithPosition,
) -> Result<(), OpenscadError> {
    match child_statement.item {
        ChildStatement::Empty => {
            Ok(())
        },
    }
}
