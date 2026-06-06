use crate::{
    interpreter::evaluator::Evaluator,
    interpreter::values::Value,
    lexer::tokentypes::TokenType,
    utils::{errors::Error, span::Span},
};

impl Evaluator {
    pub fn match_unary_operator(
        &mut self,
        operand: Value,
        operator: &TokenType,
        span: Span,
    ) -> Result<Value, Error> {
        let v = match operator {
            TokenType::Bang => match operand {
                Value::Bool(b) => Value::Bool(!b),
                _ => return Err(self.err("type mismatch on !", span)),
            },
            TokenType::Minus => match operand {
                Value::Integer(i) => Value::Integer(-i),
                Value::Float(f) => Value::Float(-f),
                _ => return Err(self.err("type mismatch on -", span)),
            },
            _ => return Err(self.err("unknown unary operator", span)),
        };
        Ok(v)
    }
}
