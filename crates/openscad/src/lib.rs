pub mod docs;
pub mod interpreter;
pub mod language_server;
pub mod parser;
pub mod source;
pub mod tokenizer;
pub mod value;

use std::fmt::Display;
use std::sync::Arc;

use caustic_core::{Random, SceneData};

use crate::source::Source;
use crate::{
    interpreter::openscad_interpret, parser::openscad_parse, tokenizer::openscad_tokenize,
};

#[derive(Debug, Clone)]
pub struct WithPosition<T: PartialEq> {
    pub item: T,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub source: Arc<Box<dyn Source>>,
}

impl Position {
    pub fn contains_pos(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source.to_string(self.start, self.end))
    }
}

impl<T: PartialEq> WithPosition<T> {
    pub fn new(item: T, position: Position) -> Self {
        Self { item, position }
    }

    fn equals(&self, other: &WithPosition<T>) -> bool {
        self.item.eq(&other.item)
            && self.position.start == other.position.start
            && self.position.end == other.position.end
            && self
                .position
                .source
                .equals(other.position.source.as_ref().as_ref())
    }
}

impl<T: PartialEq> PartialEq for WithPosition<T> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageLevel {
    Echo,
    Warning,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub level: MessageLevel,
    pub message: String,
    pub position: Position,
}

pub type Result<T> = core::result::Result<T, Message>;

pub struct OpenscadResults {
    pub scene_data: Option<SceneData>,
    pub messages: Vec<Message>,
}

pub fn run_openscad(source: Arc<Box<dyn Source>>, random: Arc<dyn Random>) -> OpenscadResults {
    let mut messages: Vec<Message> = vec![];

    let mut tokenize_results = openscad_tokenize(source.clone());
    messages.append(&mut tokenize_results.messages);
    let tokens = if let Some(tokens) = tokenize_results.tokens {
        tokens
    } else {
        return OpenscadResults {
            scene_data: None,
            messages,
        };
    };

    let mut parse_results = openscad_parse(tokens, source);
    messages.append(&mut parse_results.messages);
    let statements = if let Some(statements) = parse_results.statements {
        statements
    } else {
        return OpenscadResults {
            scene_data: None,
            messages,
        };
    };

    let mut interpret_results = openscad_interpret(statements, random);
    messages.append(&mut interpret_results.messages);
    let scene_data = if let Some(scene_data) = interpret_results.scene_data {
        scene_data
    } else {
        return OpenscadResults {
            scene_data: None,
            messages,
        };
    };

    OpenscadResults {
        scene_data: Some(scene_data),
        messages,
    }
}
