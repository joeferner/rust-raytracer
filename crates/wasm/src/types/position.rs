use caustic_openscad::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmPosition {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

impl From<&Position> for WasmPosition {
    fn from(value: &Position) -> Self {
        Self {
            start: value.start,
            end: value.end,
            filename: value.source.get_filename().to_owned(),
        }
    }
}
