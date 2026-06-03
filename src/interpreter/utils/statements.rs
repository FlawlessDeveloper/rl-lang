use crate::{
    ast::statements::Statement,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

impl Evaluator {
    pub fn evaluate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDeclaration {
                name,
                type_annotation,
                value,
            } => {
                let val = self.evaluate(value);
                // should add type check here but for now assume the user input correctly
                self.insert_value(name.clone(), val);
            }
            Statement::Expression(expr) => {
                self.evaluate(expr);
            }
            Statement::While { condition, body } => loop {
                match self.evaluate(condition) {
                    Value::Bool(true) => {}
                    Value::Bool(false) => break,
                    _ => {
                        panic!();
                    }
                }
                self.evaluate_block(body);
            },
            Statement::ConditionalBranch { condition, body } => match condition {
                Some(condition) => {
                    match self.evaluate(condition) {
                        Value::Bool(true) => {}
                        Value::Bool(false) => {
                            return;
                        }
                        _ => {
                            panic!();
                        }
                    }
                    self.evaluate_block(body);
                }
                _ => {
                    self.evaluate_block(body);
                }
            },
            Statement::Conditional {
                if_branch,
                elseif_branch,
                else_branch,
            } => {
                self.evaluate_statement(if_branch);

                if let Some(branches) = elseif_branch {
                    for branch in branches {
                        self.evaluate_statement(branch);
                    }
                }

                if let Some(branch) = else_branch {
                    self.evaluate_statement(branch);
                }
            }
        }
    }

    pub fn evaluate_block(&mut self, statements: &[Statement]) {
        for statement in statements {
            self.evaluate_statement(statement);
        }
    }
}
