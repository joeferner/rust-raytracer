use std::{mem::swap, sync::Arc};

use caustic_core::{
    Color,
    texture::{CheckerTexture, SolidColor, Texture},
};

use crate::{
    interpreter::{Interpreter, Result},
    parser::CallArgumentWithPosition,
    value::Value,
};

impl Interpreter {
    pub(super) fn evaluate_function_call(
        &mut self,
        name: &str,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Value> {
        if name == "checker" {
            self.evaluate_checker(arguments)
        } else if name == "pow" {
            self.evaluate_pow(arguments)
        } else if name == "sqrt" {
            self.evaluate_sqrt(arguments)
        } else if name == "rands" {
            self.evaluate_rands(arguments)
        } else {
            self.evaluate_non_built_in(name, arguments)
        }
    }

    fn evaluate_pow(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func2(arguments, |base, exponent| base.powf(exponent))
    }

    fn evaluate_sqrt(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, |v| v.sqrt())
    }

    fn evaluate_math_func1<F>(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        func: F,
    ) -> Result<Value>
    where
        F: Fn(f64) -> f64,
    {
        let arguments = self.convert_args(&["arg"], arguments)?;

        let arg = if let Some(arg) = arguments.get("arg") {
            arg.to_number()?
        } else {
            todo!("missing arg");
        };

        let result = func(arg);

        Ok(Value::Number(result))
    }

    fn evaluate_math_func2<F>(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        func: F,
    ) -> Result<Value>
    where
        F: Fn(f64, f64) -> f64,
    {
        let arguments = self.convert_args(&["arg1", "arg2"], arguments)?;

        let arg1 = if let Some(arg1) = arguments.get("arg1") {
            arg1.to_number()?
        } else {
            todo!("missing arg1");
        };

        let arg2 = if let Some(arg2) = arguments.get("arg2") {
            arg2.to_number()?
        } else {
            todo!("missing arg2");
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
            scale = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("even") {
            even = Arc::new(SolidColor::new(arg.to_color()?));
        }

        if let Some(arg) = arguments.get("odd") {
            odd = Arc::new(SolidColor::new(arg.to_color()?));
        }

        Ok(Value::Texture(Arc::new(CheckerTexture::new(
            scale, even, odd,
        ))))
    }

    fn evaluate_rands(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        let arguments = self.convert_args(
            &["min_value", "max_value", "value_count", "seed_value"],
            arguments,
        )?;

        let mut min_value = if let Some(arg) = arguments.get("min_value") {
            arg.to_number()?
        } else {
            todo!("min_value required");
        };

        let mut max_value = if let Some(arg) = arguments.get("max_value") {
            arg.to_number()?
        } else {
            todo!("max_value required");
        };

        let value_count = if let Some(arg) = arguments.get("value_count") {
            arg.to_u64()?
        } else {
            todo!("value_count required");
        };

        let seed_value = if let Some(arg) = arguments.get("seed_value") {
            Some(arg.to_number()?)
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
                self.set_variable(&name, value);
            }

            self.expr_to_value(&expr)
        }
    }
}
