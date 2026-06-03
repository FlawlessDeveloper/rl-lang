use crate::{ast::statements::Statement, parser::parser::Parser};

impl Parser {
    pub fn parse_while(&mut self) -> Statement {
        let condition = self.parse_expression();
        let body = self.parse_block();
        Statement::While { condition, body }
    }
}
