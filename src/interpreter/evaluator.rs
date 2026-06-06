use std::collections::HashMap;

use crate::{
    ast::nodes::{Expression, ExpressionKind},
    interpreter::{stdlib, values::Value},
    utils::{
        errors::{Error, Reason},
        source::SourceFile,
        span::Span,
        suggest::closest_match,
    },
};

pub struct Evaluator {
    pub environment: HashMap<String, (Value, bool)>,
    pub source_file: Option<SourceFile>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
            source_file: None,
        }
    }

    pub fn with_source_file(mut self, file: SourceFile) -> Self {
        self.source_file = Some(file);
        self
    }

    pub fn set_source_file(&mut self, file: SourceFile) {
        self.source_file = Some(file);
    }

    /// Build a [`Reason::Runtime`] error anchored at `span`, with source attached when known.
    pub fn err(&self, message: impl Into<String>, span: Span) -> Error {
        let err = Error::at(Reason::Runtime, message, span);
        match &self.source_file {
            Some(file) => err.with_source_file(file),
            None => err,
        }
    }

    pub fn evaluate(&mut self, expression: &Expression) -> Result<Value, Error> {
        let value = match &expression.kind {
            ExpressionKind::Integer(i) => Value::Integer(*i),
            ExpressionKind::String(s) => Value::String(s.clone()),
            ExpressionKind::Bool(b) => Value::Bool(*b),
            ExpressionKind::Float(f) => Value::Float(*f),
            ExpressionKind::Character(c) => Value::Char(*c),
            ExpressionKind::Index { target, index } => {
                let arr = self.evaluate(target)?;
                let idx = self.evaluate(index)?;
                match (&arr, &idx) {
                    (Value::Values(items), Value::Integer(i)) => {
                        let i_usize = *i as usize;
                        if i_usize >= items.len() {
                            return Err(self
                                .err(
                                    format!("index {} out of bounds (len {})", i, items.len()),
                                    expression.span,
                                )
                                .with_label(
                                    target.span,
                                    format!("this array has length {}", items.len()),
                                ));
                        }
                        items[i_usize].clone()
                    }
                    _ => {
                        return Err(self
                            .err("invalid index operation", expression.span)
                            .with_label(target.span, format!("this is {}", arr.type_name()))
                            .with_label(index.span, format!("this is {}", idx.type_name())));
                    }
                }
            }
            ExpressionKind::ArrayLiteral(items) => {
                let mut values = Vec::with_capacity(items.len());
                for e in items {
                    values.push(self.evaluate(e)?);
                }
                Value::Values(values)
            }
            ExpressionKind::IndexAssign {
                target,
                index,
                value,
            } => self.index_assign(target, index, value, expression.span)?,
            ExpressionKind::Grouping(inner) => self.evaluate(inner)?,
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.match_binary_operator(
                    left_val,
                    left.span,
                    right_val,
                    right.span,
                    operator,
                    expression.span,
                )?
            }
            ExpressionKind::Unary { operator, operand } => {
                let operand_val = self.evaluate(operand)?;
                self.match_unary_operator(operand_val, operand.span, operator, expression.span)?
            }
            ExpressionKind::Identifier(name) => self.get_value(name, expression.span)?,
            ExpressionKind::Assign { name, value } => {
                let val = self.evaluate(value)?;
                self.insert_value(name.clone(), val.clone(), expression.span)?;
                val
            }
            ExpressionKind::Call { name, args } => {
                let mut evaluated_args = Vec::with_capacity(args.len());
                for arg in args {
                    evaluated_args.push(self.evaluate(arg)?);
                }
                self.call_function(name, evaluated_args, expression.span)?
            }
        };
        Ok(value)
    }

    pub fn get_value(&self, name: &str, span: Span) -> Result<Value, Error> {
        match self.environment.get(name) {
            Some((val, _)) => Ok(val.clone()),
            None => {
                let mut err = self.err(format!("undefined variable {}", name), span);
                if let Some(suggestion) =
                    closest_match(name, self.environment.keys().map(|s| s.as_str()))
                {
                    err = err.with_help(format!("did you mean `{}`?", suggestion));
                }
                Err(err)
            }
        }
    }

    pub fn insert_value(&mut self, name: String, value: Value, span: Span) -> Result<(), Error> {
        if let Some((_, true)) = self.environment.get(&name) {
            return Err(self.err(format!("cannot assign to constant '{}'", name), span));
        }
        self.environment.insert(name, (value, false));
        Ok(())
    }

    pub fn insert_const(&mut self, name: String, value: Value, span: Span) -> Result<(), Error> {
        if self.environment.contains_key(&name) {
            return Err(self.err(format!("'{}' is already declared", name), span));
        }
        self.environment.insert(name, (value, true));
        Ok(())
    }

    pub fn call_function(
        &mut self,
        name: &str,
        args: Vec<Value>,
        span: Span,
    ) -> Result<Value, Error> {
        if stdlib::display::is_in_display(name) {
            Ok(stdlib::display::match_std_display(name, args))
        } else if stdlib::math::is_in_math(name) {
            Ok(stdlib::math::match_std_math(name, args))
        } else if stdlib::io::is_in_io(name) {
            Ok(stdlib::io::match_std_io(name, args))
        } else {
            let mut err = self.err(format!("undefined function {}", name), span);
            let candidates = stdlib::display::KEYWORDS
                .iter()
                .chain(stdlib::math::KEYWORDS)
                .chain(stdlib::io::KEYWORDS)
                .copied();
            if let Some(suggestion) = closest_match(name, candidates) {
                err = err.with_help(format!("did you mean `{}`?", suggestion));
            }
            Err(err)
        }
    }
}
