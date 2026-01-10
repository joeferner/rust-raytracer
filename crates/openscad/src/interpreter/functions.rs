use std::{mem::swap, sync::Arc};

use caustic_core::{
    Color,
    texture::{CheckerTexture, ImageTexture, PerlinTurbulenceTexture, SolidColor, Texture},
};

use crate::{
    interpreter::{Interpreter, InterpreterError, Result},
    parser::CallArgumentWithPosition,
    value::Value,
};

impl Interpreter {
    pub(super) fn evaluate_function_call(
        &mut self,
        name: &str,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Value> {
        match name {
            "checker" => self.evaluate_checker(arguments),
            "perlin_turbulence" => self.evaluate_perlin_turbulence(arguments),
            "abs" => self.evaluate_abs(arguments),
            "sign" => self.evaluate_sign(arguments),
            "sin" => self.evaluate_sin(arguments),
            "pow" => self.evaluate_pow(arguments),
            "sqrt" => self.evaluate_sqrt(arguments),
            "rands" => self.evaluate_rands(arguments),
            "image" => self.evaluate_image(arguments),
            "is_undef" => self.evaluate_is_undef(arguments),
            "is_bool" => self.evaluate_is_bool(arguments),
            "is_num" => self.evaluate_is_num(arguments),
            "is_string" => self.evaluate_is_string(arguments),
            "is_list" => self.evaluate_is_list(arguments),
            "is_function" => self.evaluate_is_function(arguments),
            other => self.evaluate_non_built_in(other, arguments),
        }
    }

    fn evaluate_abs(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.abs())
    }

    fn evaluate_sign(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| if v == 0.0 { 0.0 } else { v.signum() })
    }

    fn evaluate_sin(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "degrees", |v| v.to_radians().sin())
    }

    fn evaluate_pow(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func2(arguments, "base", "exponent", |base, exponent| {
            base.powf(exponent)
        })
    }

    fn evaluate_sqrt(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.sqrt())
    }

    fn evaluate_is_undef(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_func1(arguments, "x", |v| {
            Ok(Value::Boolean(matches!(v, Value::Undef)))
        })
    }

    fn evaluate_is_bool(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_func1(arguments, "x", |v| {
            Ok(Value::Boolean(matches!(v, Value::Boolean(_))))
        })
    }

    fn evaluate_is_num(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_func1(arguments, "x", |v| {
            Ok(Value::Boolean(matches!(v, Value::Number(_))))
        })
    }

    fn evaluate_is_string(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_func1(arguments, "x", |v| {
            Ok(Value::Boolean(matches!(v, Value::String(_))))
        })
    }

    fn evaluate_is_list(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_func1(arguments, "x", |v| {
            Ok(Value::Boolean(matches!(v, Value::Vector { items: _ })))
        })
    }

    fn evaluate_is_function(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_func1(arguments, "x", |v| {
            Ok(Value::Boolean(matches!(
                v,
                Value::FunctionRef { function_name: _ }
            )))
        })
    }

    fn evaluate_func1<F>(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        arg_name: &str,
        func: F,
    ) -> Result<Value>
    where
        F: Fn(&Value) -> Result<Value>,
    {
        let arguments = self.convert_args(&[arg_name], arguments)?;

        let arg = if let Some(arg) = arguments.get(arg_name) {
            &arg.item
        } else {
            todo!("missing arg");
        };

        func(arg)
    }

    fn evaluate_math_func1<F>(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        arg_name: &str,
        func: F,
    ) -> Result<Value>
    where
        F: Fn(f64) -> f64,
    {
        self.evaluate_func1(arguments, arg_name, |arg| {
            let num = arg.to_number()?;
            Ok(Value::Number(func(num)))
        })
    }

    fn evaluate_math_func2<F>(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        arg1_name: &str,
        arg2_name: &str,
        func: F,
    ) -> Result<Value>
    where
        F: Fn(f64, f64) -> f64,
    {
        let arguments = self.convert_args(&[arg1_name, arg2_name], arguments)?;

        let arg1 = if let Some(arg1) = arguments.get(arg1_name) {
            arg1.item.to_number()?
        } else {
            todo!("missing {arg1_name}");
        };

        let arg2 = if let Some(arg2) = arguments.get(arg2_name) {
            arg2.item.to_number()?
        } else {
            todo!("missing {arg2_name}");
        };

        let result = func(arg1, arg2);

        Ok(Value::Number(result))
    }

    fn evaluate_checker(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        let arguments = self.convert_args(&["scale", "even", "odd"], arguments)?;

        let mut scale: f64 = 0.0;
        let mut even: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0)));
        let mut odd: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0)));

        if let Some(arg) = arguments.get("scale") {
            scale = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("even") {
            even = Arc::new(SolidColor::new(arg.item.to_color()?));
        }

        if let Some(arg) = arguments.get("odd") {
            odd = Arc::new(SolidColor::new(arg.item.to_color()?));
        }

        Ok(Value::Texture(Arc::new(CheckerTexture::new(
            scale, even, odd,
        ))))
    }

    fn evaluate_perlin_turbulence(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Value> {
        let arguments = self.convert_args(&["scale", "turbulence_depth"], arguments)?;

        let mut scale: f64 = 1.0;
        let mut turbulence_depth: u32 = 1;

        if let Some(arg) = arguments.get("scale") {
            scale = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("turbulence_depth") {
            turbulence_depth = arg.item.to_number()? as u32;
        }

        Ok(Value::Texture(Arc::new(PerlinTurbulenceTexture::new(
            self.random.as_ref(),
            scale,
            turbulence_depth,
        ))))
    }

    fn evaluate_image(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        let arguments = self.convert_args(&["filename"], arguments)?;

        let image = if let Some(arg) = arguments.get("filename") {
            let start = arg.start;
            let end = arg.end;
            let filename = arg.item.to_unescaped_string()?;
            arg.source
                .get_image(&filename)
                .map_err(|err| InterpreterError {
                    start,
                    end,
                    message: format!("failed to get image \"{filename}\": {err:?}"),
                })?
        } else {
            todo!("filename required");
        };

        Ok(Value::Texture(Arc::new(ImageTexture::new(image))))
    }

    fn evaluate_rands(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        let arguments = self.convert_args(
            &["min_value", "max_value", "value_count", "seed_value"],
            arguments,
        )?;

        let mut min_value = if let Some(arg) = arguments.get("min_value") {
            arg.item.to_number()?
        } else {
            todo!("min_value required");
        };

        let mut max_value = if let Some(arg) = arguments.get("max_value") {
            arg.item.to_number()?
        } else {
            todo!("max_value required");
        };

        let value_count = if let Some(arg) = arguments.get("value_count") {
            arg.item.to_u64()?
        } else {
            todo!("value_count required");
        };

        let seed_value = if let Some(arg) = arguments.get("seed_value") {
            Some(arg.item.to_number()?)
        } else {
            None
        };

        if max_value < min_value {
            swap(&mut min_value, &mut max_value);
        }

        let mut items = vec![];
        for _ in 0..value_count {
            let rand_value = if let Some(seed_value) = seed_value {
                todo!("rands with seed {seed_value}");
            } else {
                self.rng.next_u64()
            };

            let normalized = rand_value as f64 / u64::MAX as f64;
            let v = min_value + normalized * (max_value - min_value);
            items.push(Value::Number(v));
        }
        Ok(Value::Vector { items })
    }

    fn evaluate_non_built_in(
        &mut self,
        name: &str,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Value> {
        let (arg_names, expr) = if let Some(function) = self.functions.get(name) {
            let arg_names = function.get_argument_names();
            (arg_names, function.expr.clone())
        } else {
            todo!("missing function {name}");
        };
        let arg_names: Vec<&str> = arg_names.iter().map(|s| s.as_str()).collect();

        let arguments = self.convert_args(&arg_names, arguments)?;

        {
            let _scope = self.create_scope();

            for (name, value) in arguments {
                self.set_variable(&name, value.item);
            }

            self.expr_to_value(&expr)
        }
    }
}
