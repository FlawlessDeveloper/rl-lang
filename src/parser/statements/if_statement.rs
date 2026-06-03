use crate::{ast::statements::Statement, lexer::tokentypes::TokenType, parser::parser::Parser};

impl Parser {
    pub fn parse_if(&mut self) -> Statement {
        let if_condition = self.parse_expression();
        let if_body = self.parse_block();
        let if_branch = Statement::ConditionalBranch {
            condition: Some(if_condition),
            body: if_body,
        };
        let mut elseif_branch = Vec::new();
        let else_body = Vec::new();
        while self.peek() == TokenType::Else {
            self.advance();
            if self.peek() == TokenType::If {
                let branch_condition = self.parse_expression();
                let branch_body = self.parse_block();
                let branch = Statement::ConditionalBranch {
                    condition: Some(branch_condition),
                    body: branch_body,
                };
                elseif_branch.push(branch);
            } else {
                let else_body = self.parse_block();
            }
        }
        Statement::Conditional {
            if_branch: Box::new(if_branch),
            elseif_branch: Some(elseif_branch),
            else_branch: Some(Box::new(Statement::ConditionalBranch {
                condition: None,
                body: else_body,
            })),
        }
    }
}
