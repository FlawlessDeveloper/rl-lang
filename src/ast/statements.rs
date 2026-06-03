use crate::{ast::nodes::Expression, lexer::tokentypes::TokenType};

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        type_annotation: TokenType,
        value: Expression,
    },
    Expression(Expression),
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    ConditionalBranch {
        condition: Option<Expression>,
        body: Vec<Statement>,
    },
    Conditional {
        if_branch: Box<Statement>,
        elseif_branch: Option<Vec<Statement>>,
        else_branch: Option<Box<Statement>>,
    },
}
