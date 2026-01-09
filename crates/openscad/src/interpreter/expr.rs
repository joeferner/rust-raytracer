use crate::interpreter::{Interpreter, Result};

use crate::{
    parser::{BinaryOperator, Expr, ExprWithPosition, UnaryOperator},
    value::Value,
};

impl Interpreter {
    pub(super) fn expr_to_value(&mut self, expr: &ExprWithPosition) -> Result<Value> {
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
                self.evaluate_function_call(name, arguments)?
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
        Ok(match lhs {
            Value::Number(left) => match rhs {
                Value::Number(right) => self.eval_number_number(operator, *left, *right),
                Value::Vector { items } => self.eval_vector_number(operator, items, *left),
                Value::String(str) => todo!("{left:?} {operator:?} {str}"),
                Value::Boolean(b) => todo!("{left:?} {operator:?} {b}"),
                Value::Texture(texture) => todo!("{left:?} {operator:?} {texture:?}"),
                Value::Range {
                    start,
                    end,
                    increment,
                } => todo!("{left:?} {operator:?} range({start:?}, {end:?}, {increment:?})"),
                Value::Undef => todo!("{left:?} undef"),
            },
            Value::Vector { items: lhs_items } => match rhs {
                Value::Number(right) => self.eval_vector_number(operator, lhs_items, *right),
                Value::Vector { items: rhs_items } => {
                    self.eval_vector_vector(operator, lhs_items, rhs_items)?
                }
                Value::Boolean(b) => todo!("{lhs_items:?} {operator:?} {b}"),
                Value::String(str) => todo!("{lhs_items:?} {operator:?} {str}"),
                Value::Texture(texture) => todo!("{lhs_items:?} {operator:?} {texture:?}"),
                Value::Range {
                    start,
                    end,
                    increment,
                } => todo!("{lhs_items:?} {operator:?} range({start:?}, {end:?}, {increment:?})"),
                Value::Undef => todo!("{lhs_items:?} undef"),
            },
            Value::Boolean(b) => todo!("{b}"),
            Value::String(str) => todo!("{str}"),
            Value::Texture(texture) => todo!("texture {texture:?}"),
            Value::Range {
                start,
                end,
                increment,
            } => todo!("range: {start:?}, {end:?}, {increment:?}"),
            Value::Undef => todo!("undef"),
        })
    }

    fn eval_number_number(&self, operator: &BinaryOperator, left: f64, right: f64) -> Value {
        match operator {
            BinaryOperator::Add => Value::Number(left + right),
            BinaryOperator::Subtract => Value::Number(left - right),
            BinaryOperator::Multiply => Value::Number(left * right),
            BinaryOperator::Divide => Value::Number(left / right),
            BinaryOperator::LessThan => Value::Boolean(left < right),
            BinaryOperator::LessThanEqual => Value::Boolean(left <= right),
            BinaryOperator::GreaterThan => Value::Boolean(left > right),
            BinaryOperator::GreaterThanEqual => Value::Boolean(left >= right),
            BinaryOperator::Modulus => Value::Number(left % right),
            BinaryOperator::Exponentiation => Value::Number(left.powf(right)),
            BinaryOperator::EqualEqual => Value::Boolean(left == right),
            BinaryOperator::NotEqual => Value::Boolean(left != right),
        }
    }

    fn eval_vector_number(&self, operator: &BinaryOperator, lhs: &[Value], rhs: f64) -> Value {
        Value::Vector {
            items: lhs
                .iter()
                .map(|item| match item {
                    Value::Number(v) => match operator {
                        BinaryOperator::Subtract => Value::Number(v - rhs),
                        BinaryOperator::Divide => Value::Number(v / rhs),
                        BinaryOperator::Add => todo!(),
                        BinaryOperator::Multiply => Value::Number(v * rhs),
                        BinaryOperator::LessThan => todo!(),
                        BinaryOperator::LessThanEqual => todo!(),
                        BinaryOperator::GreaterThan => todo!(),
                        BinaryOperator::GreaterThanEqual => todo!(),
                        BinaryOperator::Modulus => todo!(),
                        BinaryOperator::Exponentiation => todo!(),
                        BinaryOperator::EqualEqual => todo!(),
                        BinaryOperator::NotEqual => todo!(),
                    },
                    Value::Vector { items } => todo!("items {items:?}"),
                    Value::Boolean(b) => todo!("{b}"),
                    Value::String(str) => todo!("{str}"),
                    Value::Texture(texture) => todo!("texture {texture:?}"),
                    Value::Range {
                        start,
                        end,
                        increment,
                    } => todo!("range: {start:?}, {end:?}, {increment:?}"),
                    Value::Undef => todo!("undef"),
                })
                .collect(),
        }
    }

    fn evaluate_unary_expression(
        &mut self,
        operator: &UnaryOperator,
        rhs: &ExprWithPosition,
    ) -> Result<Value> {
        let right = self.expr_to_value(rhs)?;

        if let Value::Number(right) = right {
            match operator {
                UnaryOperator::Minus => Ok(Value::Number(-right)),
            }
        } else {
            todo!("{operator:?} {right:?}");
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
