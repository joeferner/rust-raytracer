pub mod interpreter;
pub mod parser;
pub mod tokenizer;
pub mod value;

use std::fs;

use thiserror::Error;

use crate::{
    interpreter::{InterpreterResults, openscad_interpret},
    parser::openscad_parse,
    tokenizer::{TokenizerError, openscad_tokenize},
};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Error, Debug)]
pub enum OpenscadError {
    #[error("File read error \"{filename}\": {message}")]
    FileReadError { filename: String, message: String },
    #[error("Tokenizer error: {0:?}")]
    TokenizerError(#[from] TokenizerError),
}

pub fn openscad_file_to_scene_data(filename: &str) -> Result<InterpreterResults, OpenscadError> {
    match fs::read_to_string(filename) {
        Ok(contents) => openscad_string_to_scene_data(&contents),
        Err(err) => Err(OpenscadError::FileReadError {
            filename: filename.to_owned(),
            message: err.to_string(),
        }),
    }
}

pub fn openscad_string_to_scene_data(input: &str) -> Result<InterpreterResults, OpenscadError> {
    let tokens = openscad_tokenize(input)?;
    let parse_results = openscad_parse(tokens);

    if !parse_results.errors.is_empty() {
        todo!("{:?}", parse_results.errors);
    }

    let interpret_results = openscad_interpret(parse_results.statements);
    if !interpret_results.errors.is_empty() {
        todo!("{:?}", interpret_results.errors);
    }

    Ok(interpret_results)
}
