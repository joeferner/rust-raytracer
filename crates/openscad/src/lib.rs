pub mod tokenizer;

use std::fs;

use rust_raytracer_core::SceneData;

use crate::tokenizer::OpenscadTokenizer;

#[derive(Debug)]
pub enum OpenscadError {
    FileReadError(String, String),
}

pub fn openscad_read_from_file(filename: &str) -> Result<SceneData, OpenscadError> {
    match fs::read_to_string(filename) {
        Ok(contents) => openscad_read_from_string(&contents),
        Err(err) => Err(OpenscadError::FileReadError(
            filename.to_owned(),
            err.to_string(),
        )),
    }
}

pub fn openscad_read_from_string(contents: &str) -> Result<SceneData, OpenscadError> {
    let mut tokenizer = OpenscadTokenizer::new(contents);
    println!("{:?}", tokenizer.next_token());

    todo!();
}
