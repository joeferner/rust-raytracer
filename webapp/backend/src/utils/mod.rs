use anyhow::{Result, anyhow};

use crate::repository::project_repository::CONTENT_TYPE_OPENSCAD;

pub mod google;

pub fn mime_type_from_path(path: &str) -> Result<String> {
    let guess = mime_guess::from_path(path).first();
    if let Some(mime_type) = guess {
        Ok(mime_type.to_string())
    } else if path.ends_with(".scad") {
        Ok(CONTENT_TYPE_OPENSCAD.to_string())
    } else {
        Err(anyhow!("could not guess mime type of: {path}"))
    }
}
