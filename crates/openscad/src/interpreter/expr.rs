use crate::interpreter::{Interpreter, Result};

use crate::{
    parser::{BinaryOperator, Expr, ExprWithPosition, UnaryOperator},
    value::Value,
};

impl Interpreter {
    pub(super) fn expr_to_value(&mut self, expr: &ExprWithPosition) -> Result<Value> {
        let start = expr.start;
        let end = expr.end;
        Ok(match &expr.item {
            Expr::Number(number) => Value::Number(*number),
            Expr::String(str) => Value::String(str.clone()),
            Expr::Vector { items } => {
                let items: Result<Vec<Value>> =
                    items.iter().map(|v| self.expr_to_value(v)).collect();
                Value::Vector { items: items? }
            }
            Expr::True => Value::Boolean(true),
            Expr::False => Value::Boolean(false),
            Expr::Binary { operator, lhs, rhs } => {
                self.evaluate_binary_expression(operator, lhs, rhs)?
            }
            Expr::Unary { operator, rhs } => self.evaluate_unary_expression(operator, rhs)?,
            Expr::FunctionCall { name, arguments } => {
                self.evaluate_function_call(name, arguments, start, end)?
            }
            Expr::Range {
                start,
                end,
                increment,
            } => self.evaluate_range_expression(start, end, increment)?,
            Expr::Identifier { name } => self.evaluate_identifier(name)?,
            Expr::Index { lhs, index } => self.evaluate_index(lhs, index)?,
            Expr::Ternary {
                condition,
                true_expr,
                false_expr,
            } => self.evaluate_ternary(condition, true_expr, false_expr)?,
            Expr::FieldAccess { lhs, field } => self.evaluate_field_access(lhs, field)?,
        })
    }

    fn evaluate_range_expression(
        &mut self,
        start: &ExprWithPosition,
        end: &ExprWithPosition,
        increment: &Option<Box<ExprWithPosition>>,
    ) -> Result<Value> {
        let start = Box::new(self.expr_to_value(start)?);
        let end = Box::new(self.expr_to_value(end)?);
        let increment = if let Some(increment) = increment {
            Some(Box::new(self.expr_to_value(increment)?))
        } else {
            None
        };

        Ok(Value::Range {
            start,
            end,
            increment,
        })
    }

    fn evaluate_binary_expression(
        &mut self,
        operator: &BinaryOperator,
        lhs: &ExprWithPosition,
        rhs: &ExprWithPosition,
    ) -> Result<Value> {
        let lhs = self.expr_to_value(lhs)?;
        let rhs = self.expr_to_value(rhs)?;
        self.evaluate_binary_expression_values(operator, &lhs, &rhs)
    }

    fn evaluate_binary_expression_values(
        &self,
        operator: &BinaryOperator,
        lhs: &Value,
        rhs: &Value,
    ) -> Result<Value> {
        match operator {
            BinaryOperator::Exponentiation => {
                self.evaluate_binary_expression_exponentiation(lhs, rhs)
            }
            BinaryOperator::Modulus => self.evaluate_binary_expression_modulus(lhs, rhs),
            BinaryOperator::Add => self.evaluate_binary_expression_add(lhs, rhs),
            BinaryOperator::Subtract => self.evaluate_binary_expression_subtract(lhs, rhs),
            BinaryOperator::Multiply => self.evaluate_binary_expression_multiply(lhs, rhs),
            BinaryOperator::Divide => self.evaluate_binary_expression_divide(lhs, rhs),
            BinaryOperator::LessThan => self.evaluate_binary_expression_less_than(lhs, rhs),
            BinaryOperator::LessThanEqual => {
                self.evaluate_binary_expression_less_than_equal(lhs, rhs)
            }
            BinaryOperator::GreaterThan => self.evaluate_binary_expression_greater_than(lhs, rhs),
            BinaryOperator::GreaterThanEqual => {
                self.evaluate_binary_expression_greater_than_equal(lhs, rhs)
            }
            BinaryOperator::EqualEqual => self.evaluate_binary_expression_equal_equal(lhs, rhs),
            BinaryOperator::NotEqual => self.evaluate_binary_expression_not_equals(lhs, rhs),
            BinaryOperator::And => Ok(Value::Boolean(lhs.is_truthy() && rhs.is_truthy())),
            BinaryOperator::Or => Ok(Value::Boolean(lhs.is_truthy() || rhs.is_truthy())),
        }
    }

    fn evaluate_binary_expression_exponentiation(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => {
                self.evaluate_binary_expression_exponentiation_number_value(*lhs, rhs)
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_exponentiation_number_value(
        &self,
        lhs: f64,
        rhs: &Value,
    ) -> Result<Value> {
        match rhs {
            Value::Number(rhs) => Ok(Value::Number(lhs.powf(*rhs))),
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_modulus(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => self.evaluate_binary_expression_modulus_number_value(*lhs, rhs),
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_modulus_number_value(
        &self,
        lhs: f64,
        rhs: &Value,
    ) -> Result<Value> {
        match rhs {
            Value::Number(rhs) => Ok(Value::Number(lhs % rhs)),
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_add(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => self.evaluate_binary_expression_add_number_value(*lhs, rhs),
            Value::Vector { items: lhs } => {
                self.evaluate_binary_expression_vector_value(&BinaryOperator::Add, lhs, rhs)
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_add_number_value(&self, lhs: f64, rhs: &Value) -> Result<Value> {
        match rhs {
            Value::Number(rhs) => Ok(Value::Number(lhs + rhs)),
            Value::Vector { items: rhs } => {
                let items: Result<Vec<Value>> = rhs
                    .iter()
                    .map(|rhs_v| self.evaluate_binary_expression_add_number_value(lhs, rhs_v))
                    .collect();
                Ok(Value::Vector { items: items? })
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_subtract(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => self.evaluate_binary_expression_subtract_number_value(*lhs, rhs),
            Value::Vector { items: lhs } => {
                self.evaluate_binary_expression_vector_value(&BinaryOperator::Subtract, lhs, rhs)
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_subtract_number_value(
        &self,
        lhs: f64,
        rhs: &Value,
    ) -> Result<Value> {
        match rhs {
            Value::Number(rhs) => Ok(Value::Number(lhs - rhs)),
            Value::Vector { items: rhs } => {
                let items: Result<Vec<Value>> = rhs
                    .iter()
                    .map(|rhs_v| self.evaluate_binary_expression_subtract_number_value(lhs, rhs_v))
                    .collect();
                Ok(Value::Vector { items: items? })
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_multiply(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => self.evaluate_binary_expression_multiply_number_value(*lhs, rhs),
            Value::Vector { items: lhs } => {
                self.evaluate_binary_expression_vector_value(&BinaryOperator::Multiply, lhs, rhs)
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_multiply_number_value(
        &self,
        lhs: f64,
        rhs: &Value,
    ) -> Result<Value> {
        match rhs {
            Value::Number(rhs) => Ok(Value::Number(lhs * rhs)),
            Value::Vector { items: rhs } => {
                let items: Result<Vec<Value>> = rhs
                    .iter()
                    .map(|rhs_v| self.evaluate_binary_expression_multiply_number_value(lhs, rhs_v))
                    .collect();
                Ok(Value::Vector { items: items? })
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_divide(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => self.evaluate_binary_expression_divide_number_value(*lhs, rhs),
            Value::Vector { items: lhs } => {
                self.evaluate_binary_expression_vector_value(&BinaryOperator::Divide, lhs, rhs)
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_divide_number_value(
        &self,
        lhs: f64,
        rhs: &Value,
    ) -> Result<Value> {
        match rhs {
            Value::Number(rhs) => Ok(Value::Number(lhs / rhs)),
            Value::Vector { items: rhs } => {
                let items: Result<Vec<Value>> = rhs
                    .iter()
                    .map(|rhs_v| self.evaluate_binary_expression_divide_number_value(lhs, rhs_v))
                    .collect();
                Ok(Value::Vector { items: items? })
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_vector_value(
        &self,
        operator: &BinaryOperator,
        lhs_items: &[Value],
        rhs: &Value,
    ) -> Result<Value> {
        match rhs {
            Value::Number(rhs) => {
                let items: Result<Vec<Value>> = lhs_items
                    .iter()
                    .map(|lhs_v| {
                        self.evaluate_binary_expression_values(
                            operator,
                            lhs_v,
                            &Value::Number(*rhs),
                        )
                    })
                    .collect();
                Ok(Value::Vector { items: items? })
            }
            Value::Vector { items: rhs_items } => {
                self.eval_vector_vector(operator, lhs_items, rhs_items)
            }
            _ => todo!("unsupported"),
        }
    }

    fn evaluate_binary_expression_less_than(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => {
                if let Value::Number(rhs) = rhs {
                    Ok(Value::Boolean(lhs < rhs))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::String(lhs) => todo!("{lhs} < {rhs}"),
            Value::Vector { items: lhs_items } => {
                if let Value::Vector { items: rhs_items } = rhs {
                    self.eval_vector_vector(&BinaryOperator::LessThan, lhs_items, rhs_items)
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::Boolean(lhs) => todo!("{lhs} < {rhs}"),
            Value::Texture(lhs) => todo!("{lhs:?} < {rhs}"),
            Value::Range {
                start: lhs_start,
                end: lhs_end,
                increment: lhs_increment,
            } => todo!("{lhs_start} {lhs_end} {lhs_increment:?} < {rhs}"),
            Value::Undef => todo!("{lhs} < {rhs}"),
            Value::FunctionRef { function_name } => todo!("{lhs} < {function_name}"),
        }
    }

    fn evaluate_binary_expression_less_than_equal(
        &self,
        lhs: &Value,
        rhs: &Value,
    ) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => {
                if let Value::Number(rhs) = rhs {
                    Ok(Value::Boolean(lhs <= rhs))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::String(lhs) => todo!("{lhs} <= {rhs}"),
            Value::Vector { items: lhs_items } => {
                if let Value::Vector { items: rhs_items } = rhs {
                    self.eval_vector_vector(&BinaryOperator::LessThanEqual, lhs_items, rhs_items)
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::Boolean(lhs) => todo!("{lhs} <= {rhs}"),
            Value::Texture(lhs) => todo!("{lhs:?} <= {rhs}"),
            Value::Range {
                start: lhs_start,
                end: lhs_end,
                increment: lhs_increment,
            } => todo!("{lhs_start} {lhs_end} {lhs_increment:?} <= {rhs}"),
            Value::Undef => todo!("{lhs} <= {rhs}"),
            Value::FunctionRef { function_name } => todo!("{lhs} <= {function_name}"),
        }
    }

    fn evaluate_binary_expression_greater_than(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => {
                if let Value::Number(rhs) = rhs {
                    Ok(Value::Boolean(lhs > rhs))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::String(lhs) => todo!("{lhs} > {rhs}"),
            Value::Vector { items: lhs_items } => {
                if let Value::Vector { items: rhs_items } = rhs {
                    self.eval_vector_vector(&BinaryOperator::GreaterThan, lhs_items, rhs_items)
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::Boolean(lhs) => todo!("{lhs} > {rhs}"),
            Value::Texture(lhs) => todo!("{lhs:?} > {rhs}"),
            Value::Range {
                start: lhs_start,
                end: lhs_end,
                increment: lhs_increment,
            } => todo!("{lhs_start} {lhs_end} {lhs_increment:?} > {rhs}"),
            Value::Undef => todo!("{lhs} > {rhs}"),
            Value::FunctionRef { function_name } => todo!("{lhs} > {function_name}"),
        }
    }

    fn evaluate_binary_expression_greater_than_equal(
        &self,
        lhs: &Value,
        rhs: &Value,
    ) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => {
                if let Value::Number(rhs) = rhs {
                    Ok(Value::Boolean(lhs >= rhs))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::String(lhs) => todo!("{lhs} >= {rhs}"),
            Value::Vector { items: lhs_items } => {
                if let Value::Vector { items: rhs_items } = rhs {
                    self.eval_vector_vector(&BinaryOperator::GreaterThanEqual, lhs_items, rhs_items)
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::Boolean(lhs) => todo!("{lhs} >= {rhs}"),
            Value::Texture(lhs) => todo!("{lhs:?} >= {rhs}"),
            Value::Range {
                start: lhs_start,
                end: lhs_end,
                increment: lhs_increment,
            } => todo!("{lhs_start} {lhs_end} {lhs_increment:?} >= {rhs}"),
            Value::Undef => todo!("{lhs} >= {rhs}"),
            Value::FunctionRef { function_name } => todo!("{lhs} >= {function_name}"),
        }
    }

    fn evaluate_binary_expression_equal_equal(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => {
                if let Value::Number(rhs) = rhs {
                    Ok(Value::Boolean(lhs == rhs))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::String(lhs) => todo!("{lhs} == {rhs}"),
            Value::Vector { items: lhs_items } => {
                if let Value::Vector { items: rhs_items } = rhs {
                    self.eval_vector_vector(&BinaryOperator::EqualEqual, lhs_items, rhs_items)
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::Boolean(lhs) => todo!("{lhs} == {rhs}"),
            Value::Texture(lhs) => todo!("{lhs:?} == {rhs}"),
            Value::Range {
                start: lhs_start,
                end: lhs_end,
                increment: lhs_increment,
            } => todo!("{lhs_start} {lhs_end} {lhs_increment:?} == {rhs}"),
            Value::Undef => todo!("{lhs} == {rhs}"),
            Value::FunctionRef { function_name } => todo!("{lhs} == {function_name}"),
        }
    }

    fn evaluate_binary_expression_not_equals(&self, lhs: &Value, rhs: &Value) -> Result<Value> {
        match lhs {
            Value::Number(lhs) => {
                if let Value::Number(rhs) = rhs {
                    Ok(Value::Boolean(lhs != rhs))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::String(lhs) => todo!("{lhs} != {rhs}"),
            Value::Vector { items: lhs_items } => {
                if let Value::Vector { items: rhs_items } = rhs {
                    self.eval_vector_vector(&BinaryOperator::NotEqual, lhs_items, rhs_items)
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            Value::Boolean(lhs) => todo!("{lhs} != {rhs}"),
            Value::Texture(lhs) => todo!("{lhs:?} != {rhs}"),
            Value::Range {
                start: lhs_start,
                end: lhs_end,
                increment: lhs_increment,
            } => todo!("{lhs_start} {lhs_end} {lhs_increment:?} != {rhs}"),
            Value::Undef => todo!("{lhs} != {rhs}"),
            Value::FunctionRef { function_name } => todo!("{lhs} != {function_name}"),
        }
    }

    fn evaluate_unary_expression(
        &mut self,
        operator: &UnaryOperator,
        rhs: &ExprWithPosition,
    ) -> Result<Value> {
        let right = self.expr_to_value(rhs)?;

        match operator {
            UnaryOperator::Minus => match right {
                Value::Number(right) => Ok(Value::Number(-right)),
                Value::String(_) => todo!(),
                Value::Vector { items: _items } => todo!(),
                Value::Boolean(_) => todo!(),
                Value::Texture(_texture) => todo!(),
                Value::Range {
                    start: _start,
                    end: _end,
                    increment: _increment,
                } => todo!(),
                Value::Undef => todo!(),
                Value::FunctionRef {
                    function_name: _function_name,
                } => todo!(),
            },
            UnaryOperator::Negation => Ok(Value::Boolean(!right.is_truthy())),
        }
    }

    fn eval_vector_vector(
        &self,
        operator: &BinaryOperator,
        lhs_items: &[Value],
        rhs_items: &[Value],
    ) -> Result<Value> {
        let min_item_len = lhs_items.len().min(rhs_items.len());
        let mut results = vec![];

        for i in 0..min_item_len {
            let lhs = &lhs_items[i];
            let rhs = &rhs_items[i];
            let result = self.evaluate_binary_expression_values(operator, lhs, rhs)?;
            results.push(result);
        }

        match operator {
            BinaryOperator::EqualEqual => Ok(Value::Boolean(results.iter().all(|t| t.is_truthy()))),
            BinaryOperator::NotEqual => Ok(Value::Boolean(!results.iter().all(|t| !t.is_truthy()))),
            _ => Ok(Value::Vector { items: results }),
        }
    }
}
