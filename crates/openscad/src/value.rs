use std::{fmt::Display, sync::Arc};

use rust_raytracer_core::{Color, Vector3, texture::Texture};

#[derive(Debug)]
pub struct ValueConversionError {}

pub type Result<T> = std::result::Result<T, ValueConversionError>;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
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
}

impl Value {
    pub fn to_number(&self) -> Result<f64> {
        match self {
            Value::Number(value) => Ok(*value),
            _ => todo!(),
        }
    }

    pub fn to_u64(&self) -> Result<u64> {
        self.to_number().map(|v| v as u64)
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
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{number}"),
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
        }
    }
}
