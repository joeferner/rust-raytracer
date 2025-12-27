use anyhow::{Result, anyhow};

pub mod google;
pub mod s3;

pub fn mime_type_from_path(path: &str) -> Result<String> {
    let guess = mime_guess::from_path(path).first();
    if let Some(mime_type) = guess {
        Ok(mime_type.to_string())
    } else if path.ends_with(".scad") {
        Ok("application/x-openscad".to_string())
    } else {
        Err(anyhow!("could not guess mime type of: {path}"))
    }
}
