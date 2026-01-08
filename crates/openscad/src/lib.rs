pub mod interpreter;
pub mod parser;
pub mod source;
pub mod tokenizer;
pub mod value;

use std::sync::Arc;

use caustic_core::Random;
use thiserror::Error;

use crate::interpreter::InterpreterError;
use crate::source::Source;
use crate::{
    interpreter::{InterpreterResults, openscad_interpret},
    parser::openscad_parse,
    tokenizer::{TokenizerError, openscad_tokenize},
};

#[derive(Debug, Clone)]
pub struct WithPosition<T: PartialEq> {
    pub item: T,
    pub start: usize,
    pub end: usize,
    pub source: Arc<dyn Source>,
}

impl<T: PartialEq> WithPosition<T> {
    pub fn new(item: T, start: usize, end: usize, source: Arc<dyn Source>) -> Self {
        Self {
            item,
            start,
            end,
            source,
        }
    }

    fn equals(&self, other: &WithPosition<T>) -> bool {
        self.item.eq(&other.item)
            && self.start == other.start
            && self.end == other.end
            && self.source.equals(other.source.as_ref())
    }
}

impl<T: PartialEq> PartialEq for WithPosition<T> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

#[derive(Error, Debug)]
pub enum OpenscadError {
    #[error("Tokenizer error: {0:?}")]
    TokenizerError(#[from] TokenizerError),
    #[error("Tokenizer error: {errors:?}")]
    InterpreterErrors { errors: Vec<InterpreterError> },
}

pub fn run_openscad(
    source: Arc<dyn Source>,
    random: Arc<dyn Random>,
) -> Result<InterpreterResults, OpenscadError> {
    let tokens = openscad_tokenize(source)?;
    let parse_results = openscad_parse(tokens);

    if !parse_results.errors.is_empty() {
        todo!("{:?}", parse_results.errors);
    }

    let interpret_results = openscad_interpret(parse_results.statements, random);
    if !interpret_results.errors.is_empty() {
        return Err(OpenscadError::InterpreterErrors {
            errors: interpret_results.errors,
        });
    }

    Ok(interpret_results)
}
