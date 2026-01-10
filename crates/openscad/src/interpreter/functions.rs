use std::{mem::swap, sync::Arc};

use caustic_core::{
    Color,
    texture::{CheckerTexture, ImageTexture, PerlinTurbulenceTexture, SolidColor, Texture},
};

use crate::{
    interpreter::{Interpreter, InterpreterError, Result},
    parser::CallArgumentWithPosition,
    value::{Value, values_to_numbers},
};

impl Interpreter {
    pub(super) fn evaluate_function_call(
        &mut self,
        name: &str,
        arguments: &[CallArgumentWithPosition],
        start: usize,
        end: usize,
    ) -> Result<Value> {
        match name {
            "checker" => self.evaluate_checker(arguments),
            "perlin_turbulence" => self.evaluate_perlin_turbulence(arguments),
            "concat" => self.evaluate_concat(arguments),
            "lookup" => self.evaluate_lookup(arguments),
            "abs" => self.evaluate_abs(arguments),
            "sign" => self.evaluate_sign(arguments),
            "sin" => self.evaluate_sin(arguments),
            "cos" => self.evaluate_cos(arguments),
            "tan" => self.evaluate_tan(arguments),
            "asin" => self.evaluate_asin(arguments),
            "acos" => self.evaluate_acos(arguments),
            "atan" => self.evaluate_atan(arguments),
            "atan2" => self.evaluate_atan2(arguments),
            "floor" => self.evaluate_floor(arguments),
            "round" => self.evaluate_round(arguments),
            "ceil" => self.evaluate_ceil(arguments),
            "ln" => self.evaluate_ln(arguments),
            "log" => self.evaluate_log(arguments),
            "pow" => self.evaluate_pow(arguments),
            "sqrt" => self.evaluate_sqrt(arguments),
            "exp" => self.evaluate_exp(arguments),
            "min" => self.evaluate_min(arguments),
            "max" => self.evaluate_max(arguments),
            "norm" => self.evaluate_norm(arguments),
            "cross" => self.evaluate_cross(arguments, start, end),
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

    fn evaluate_concat(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        let values = self.convert_arguments_to_values(arguments)?;
        let items: Vec<Value> = values
            .iter()
            .flat_map(|v| {
                if let Value::Vector { items } = &v.item {
                    items.clone()
                } else {
                    vec![v.item.clone()]
                }
            })
            .collect();
        Ok(Value::Vector { items })
    }

    fn evaluate_lookup(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        let args = self.convert_args(&["key", "table"], arguments)?;

        let key = if let Some(key) = args.get("key") {
            &key.item
        } else {
            todo!("missing key");
        };

        let key = key.to_number()?;

        let (table, table_start, table_end) = if let Some(table) = args.get("table") {
            (&table.item, table.start, table.end)
        } else {
            todo!("missing table");
        };

        let table = if let Value::Vector { items } = table {
            items
        } else {
            todo!("table must be a vector");
        };

        let table: Result<Vec<(f64, f64)>> = table
            .iter()
            .map(|row| {
                if let Value::Vector { items } = row {
                    if items.len() != 2 {
                        Err(InterpreterError {
                            start: table_start,
                            end: table_end,
                            message: "table row must be list of 2 elements".to_string(),
                        })
                    } else {
                        let key = items[0].to_number()?;
                        let value = items[1].to_number()?;
                        Ok((key, value))
                    }
                } else {
                    Err(InterpreterError {
                        start: table_start,
                        end: table_end,
                        message: "table must be a list of lists".to_string(),
                    })
                }
            })
            .collect();

        let table = table?;

        if table.is_empty() {
            Err(InterpreterError {
                start: table_start,
                end: table_end,
                message: "table must have at least 1 row".to_string(),
            })
        } else if key <= table[0].0 {
            Ok(Value::Number(table[0].1))
        } else if key >= table.last().unwrap().0 {
            Ok(Value::Number(table.last().unwrap().1))
        } else {
            let mut last = table[0];
            for row in table {
                if key == row.0 {
                    return Ok(Value::Number(row.1));
                } else if key <= row.0 {
                    let p = (key - last.0) / (row.0 - last.0);
                    let value_delta = row.1 - last.1;
                    return Ok(Value::Number(last.1 + (p * value_delta)));
                }
                last = row;
            }
            todo!("this should not happen")
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

    fn evaluate_cos(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "degrees", |v| v.to_radians().cos())
    }

    fn evaluate_tan(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "degrees", |v| v.to_radians().tan())
    }

    fn evaluate_asin(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.asin().to_degrees())
    }

    fn evaluate_acos(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.acos().to_degrees())
    }

    fn evaluate_atan(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.atan().to_degrees())
    }

    fn evaluate_atan2(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func2(arguments, "y", "x", |y, x| y.atan2(x).to_degrees())
    }

    fn evaluate_floor(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.floor())
    }

    fn evaluate_round(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.round())
    }

    fn evaluate_ceil(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.ceil())
    }

    fn evaluate_ln(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.ln())
    }

    fn evaluate_log(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.log10())
    }

    fn evaluate_pow(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func2(arguments, "base", "exponent", |base, exponent| {
            base.powf(exponent)
        })
    }

    fn evaluate_sqrt(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.sqrt())
    }

    fn evaluate_exp(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_math_func1(arguments, "x", |v| v.exp())
    }

    fn evaluate_min(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_min_max(arguments, |a, b| a < b)
    }

    fn evaluate_max(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_min_max(arguments, |a, b| a > b)
    }

    fn evaluate_min_max<F>(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        func: F,
    ) -> Result<Value>
    where
        F: Fn(f64, f64) -> bool,
    {
        let values = self.convert_arguments_to_values(arguments)?;
        if values.is_empty() {
            // TODO add warning
            Ok(Value::Undef)
        } else {
            match &values[0].item {
                Value::Number(num) => {
                    let mut min_max = *num;
                    for value in values {
                        let v = value.item.to_number()?;
                        if func(v, min_max) {
                            min_max = v;
                        }
                    }
                    Ok(Value::Number(min_max))
                }
                Value::Vector { items } => {
                    if items.is_empty() {
                        // TODO add warning
                        Ok(Value::Undef)
                    } else {
                        let mut min_max = items[0].to_number()?;
                        for item in items {
                            let v = item.to_number()?;
                            if func(v, min_max) {
                                min_max = v;
                            }
                        }
                        Ok(Value::Number(min_max))
                    }
                }
                _ => todo!("unsupported"),
            }
        }
    }

    fn evaluate_norm(&mut self, arguments: &[CallArgumentWithPosition]) -> Result<Value> {
        self.evaluate_func1(arguments, "v", |v| match v {
            Value::Vector { items } => {
                if items.is_empty() {
                    return Ok(Value::Number(0.0));
                }
                let numbers = values_to_numbers(items).map_err(|err| InterpreterError {
                    start: arguments[0].start,
                    end: arguments[0].end,
                    message: format!("failed to convert vector element to number: {err:?}"),
                })?;
                let sum_squared: f64 = numbers.iter().map(|n| n.powf(2.0)).sum();
                Ok(Value::Number(sum_squared.sqrt()))
            }
            _ => {
                // TODO add warning
                Ok(Value::Undef)
            }
        })
    }

    fn evaluate_cross(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        start: usize,
        end: usize,
    ) -> Result<Value> {
        let arguments = self.convert_args(&["v1", "v2"], arguments)?;

        let v1 = arguments.get("v1").ok_or_else(|| InterpreterError {
            start,
            end,
            message: "missing 1st argument".to_string(),
        })?;

        let v1 = if let Value::Vector { items } = &v1.item {
            values_to_numbers(items)?
        } else {
            // TODO add warning
            return Ok(Value::Undef);
        };

        let v2 = arguments.get("v2").ok_or_else(|| InterpreterError {
            start,
            end,
            message: "missing 2nd argument".to_string(),
        })?;

        let v2 = if let Value::Vector { items } = &v2.item {
            values_to_numbers(items)?
        } else {
            // TODO add warning
            return Ok(Value::Undef);
        };

        if v1.len() != v2.len() {
            // TODO add warning
            return Ok(Value::Undef);
        }

        if v1.len() == 2 {
            let x1 = v1[0];
            let y1 = v1[1];

            let x2 = v2[0];
            let y2 = v2[1];

            let cross = x1 * y2 - y1 * x2;
            Ok(Value::Number(cross))
        } else if v1.len() == 3 {
            let x1 = v1[0];
            let y1 = v1[1];
            let z1 = v1[2];

            let x2 = v2[0];
            let y2 = v2[1];
            let z2 = v2[2];

            let x = y1 * z2 - z1 * y2;
            let y = z1 * x2 - x1 * z2;
            let z = x1 * y2 - y1 * x2;
            Ok(Value::Vector {
                items: vec![Value::Number(x), Value::Number(y), Value::Number(z)],
            })
        } else {
            // TODO add warning
            Ok(Value::Undef)
        }
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
