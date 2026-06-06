use crate::{
    interpreter::evaluator::Evaluator,
    interpreter::values::Value,
    lexer::tokentypes::TokenType,
    utils::{errors::Error, span::Span},
};

impl Evaluator {
    pub fn match_binary_operator(
        &mut self,
        left: Value,
        right: Value,
        operator: &TokenType,
        span: Span,
    ) -> Result<Value, Error> {
        let v = match operator {
            TokenType::Plus => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                _ => return Err(self.err("type mismatch on +", span)),
            },
            TokenType::Minus => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                _ => return Err(self.err("type mismatch on -", span)),
            },
            TokenType::Star => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                _ => return Err(self.err("type mismatch on *", span)),
            },
            TokenType::Slash => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => return Err(self.err("type mismatch on /", span)),
            },
            TokenType::Less => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a < b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                _ => return Err(self.err("type mismatch on <", span)),
            },
            TokenType::Greater => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a > b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                _ => return Err(self.err("type mismatch on >", span)),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a <= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
                _ => return Err(self.err("type mismatch on <=", span)),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a >= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
                _ => return Err(self.err("type mismatch on >=", span)),
            },
            TokenType::BangEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a != b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a != b),
                (Value::String(a), Value::String(b)) => Value::Bool(a != b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a != b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
                _ => return Err(self.err("type mismatch on !=", span)),
            },
            TokenType::Compare => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a == b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a == b),
                (Value::String(a), Value::String(b)) => Value::Bool(a == b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a == b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                _ => return Err(self.err("type mismatch on ==", span)),
            },
            _ => return Err(self.err("unknown binary operator", span)),
        };
        Ok(v)
    }
}
