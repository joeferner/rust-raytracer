use std::{fmt::Display, sync::Arc};

use caustic_core::{Color, Vector3, texture::Texture};

use crate::WithPosition;

#[derive(Debug)]
pub struct ValueConversionError {}

pub type Result<T> = std::result::Result<T, ValueConversionError>;

pub type ValueWithPosition = WithPosition<Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Vector {
        items: Vec<Value>,
    },
    Boolean(bool),
    Texture(Arc<dyn Texture>),
    Range {
        start: Box<Value>,
        end: Box<Value>,
        increment: Option<Box<Value>>,
    },
    Undef,
    FunctionRef {
        function_name: String,
    },
}

impl Value {
    pub fn to_number(&self) -> Result<f64> {
        match self {
            Value::Number(value) => Ok(*value),
            _ => todo!("to_number {self}"),
        }
    }

    pub fn to_u64(&self) -> Result<u64> {
        self.to_number().map(|v| v as u64)
    }

    pub fn to_i64(&self) -> Result<i64> {
        self.to_number().map(|v| v as i64)
    }

    pub fn to_vector3(&self) -> Result<Vector3> {
        match self {
            Value::Number(value) => Ok(Vector3::new(-*value, *value, *value)),
            Value::Vector { items } => Self::values_to_vector3(items),
            _ => todo!(),
        }
    }

    pub fn to_color(&self) -> Result<Color> {
        match self {
            Value::Number(value) => Ok(Color::new(*value, *value, *value)),
            Value::Vector { items } => Self::values_to_color(items),
            _ => todo!(),
        }
    }

    pub fn to_boolean(&self) -> Result<bool> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => todo!(),
        }
    }

    pub fn to_unescaped_string(&self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s.to_owned()),
            _ => todo!(),
        }
    }

    pub fn values_to_vector3(items: &[Value]) -> Result<Vector3> {
        if items.len() != 3 {
            todo!();
        }

        let x = if let Value::Number(x) = items[0] {
            x
        } else {
            todo!();
        };

        let y = if let Value::Number(y) = items[1] {
            y
        } else {
            todo!();
        };

        let z = if let Value::Number(z) = items[2] {
            z
        } else {
            todo!();
        };

        // OpenSCAD x,y,z is different than ours so flip z and y
        Ok(Vector3::new(-x, z, y))
    }

    pub fn values_to_color(items: &[Value]) -> Result<Color> {
        if items.len() != 3 {
            todo!();
        }

        let r = if let Value::Number(r) = items[0] {
            r
        } else {
            todo!();
        };

        let g = if let Value::Number(g) = items[1] {
            g
        } else {
            todo!();
        };

        let b = if let Value::Number(b) = items[2] {
            b
        } else {
            todo!();
        };

        Ok(Color::new(r, g, b))
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(number) => *number != 0.0,
            Value::String(str) => todo!("is_truthy {str}"),
            Value::Vector { items } => todo!("is_truthy {items:?}"),
            Value::Boolean(b) => *b,
            Value::Texture(texture) => todo!("is_truthy {texture:?}"),
            Value::Range {
                start,
                end,
                increment,
            } => todo!("is_truthy {start:?} {end:?} {increment:?}"),
            Value::Undef => false,
            Value::FunctionRef {
                function_name: _function_name,
            } => true,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => {
                let formatted = format!("{number:.6}");
                let formatted = formatted.trim_end_matches('0').trim_end_matches('.');
                write!(f, "{formatted}")
            }
            Value::String(str) => write!(f, "{str:?}"),
            Value::Vector { items } => {
                let mut output = String::new();
                output += "[";
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        output += ", ";
                    }
                    output += &item.to_string();
                }
                output += "]";
                write!(f, "{output}")
            }
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Texture(texture) => todo!("texture {texture:?}"),
            Value::Range {
                start,
                end,
                increment,
            } => todo!("range: {start:?} {end:?} {increment:?}"),
            Value::Undef => write!(f, "undef"),
            Value::FunctionRef { function_name } => write!(f, "fn({function_name})"),
        }
    }
}

pub fn values_to_numbers(items: &[Value]) -> Result<Vec<f64>> {
    items.iter().map(|i| i.to_number()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_string() {
        let v = Value::String("Test\nLine 2".to_owned());
        assert_eq!("\"Test\\nLine 2\"", v.to_string());
    }
}
