use caustic_openscad::{Message, MessageLevel};
use serde::{Deserialize, Serialize};

use crate::types::position::WasmPosition;

#[derive(Debug, Serialize, Deserialize)]
pub enum WasmMessageLevel {
    Echo,
    Warning,
    Error,
}

impl From<&MessageLevel> for WasmMessageLevel {
    fn from(value: &MessageLevel) -> Self {
        match value {
            MessageLevel::Echo => WasmMessageLevel::Echo,
            MessageLevel::Warning => WasmMessageLevel::Warning,
            MessageLevel::Error => WasmMessageLevel::Error,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmMessage {
    pub level: WasmMessageLevel,
    pub message: String,
    pub position: WasmPosition,
}

impl From<&Message> for WasmMessage {
    fn from(value: &Message) -> Self {
        Self {
            level: (&value.level).into(),
            message: value.message.clone(),
            position: (&value.position).into(),
        }
    }
}
