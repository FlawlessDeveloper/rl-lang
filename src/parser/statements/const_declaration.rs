use crate::{
    ast::statements::{Statement, StatementKind, TypeAnnotation},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::span::Span,
};

impl Parser {
    pub fn parse_const_declartion(&mut self, start: Span) -> Statement {
        log::debug!("{:?}", self.peek());
        log::debug!("parsing type");
        if self.match_type(&[TokenType::Array]) && self.peek() == TokenType::LeftBracket {
            self.advance();
            let annoation_type = match self.peek() {
                TokenType::Int => {
                    self.advance();
                    TypeAnnotation::Int
                }
                TokenType::Float => {
                    self.advance();
                    TypeAnnotation::Float
                }
                TokenType::Bool => {
                    self.advance();
                    TypeAnnotation::Bool
                }
                TokenType::String => {
                    self.advance();
                    TypeAnnotation::String
                }
                TokenType::Char => {
                    self.advance();
                    TypeAnnotation::Char
                }
                TokenType::Array => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(false);
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::Array(Box::new(inner))
                }
                _ => {
                    crate::utils::errors::Error::init(
                        "expected type after dec".to_string(),
                        None,
                        Some(crate::utils::errors::ErrorReason::init(
                            crate::utils::errors::Reason::Parse,
                            None,
                        )),
                    )
                    .print_error();
                    unreachable!()
                }
            };
            if !self.match_type(&[TokenType::RightBracket]) {
                crate::utils::errors::Error::init(
                    "expected ']' after type".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
            }

            let name = match self.peek() {
                TokenType::Identifier(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => {
                    crate::utils::errors::Error::init(
                        "expected name after array type".to_string(),
                        None,
                        Some(crate::utils::errors::ErrorReason::init(
                            crate::utils::errors::Reason::Parse,
                            None,
                        )),
                    )
                    .print_error();
                    unreachable!()
                }
            };

            if !self.match_type(&[TokenType::Assign]) {
                crate::utils::errors::Error::init(
                    "expected '=' after name".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
            }

            if !self.match_type(&[TokenType::LeftBracket]) {
                crate::utils::errors::Error::init(
                    "expected '[' after type".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
            }
            let mut items = Vec::new();

            while self.peek() != TokenType::RightBracket {
                let value = self.parse_expression();
                items.push(value);
                if self.peek() == TokenType::RightBracket {
                    break;
                }
                if !self.match_type(&[TokenType::Comma]) {
                    crate::utils::errors::Error::init(
                        "expected ',' between list items".to_string(),
                        None,
                        Some(crate::utils::errors::ErrorReason::init(
                            crate::utils::errors::Reason::Parse,
                            None,
                        )),
                    )
                    .print_error();
                }
            }
            self.match_type(&[TokenType::RightBracket]);
            let span = start.join(self.previous_span());
            return Statement::new(
                StatementKind::ConstantArray {
                    name,
                    type_annotation: annoation_type,
                    value: items,
                },
                span,
            );
        }

        let const_type = self.parse_type(false);
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => {
                crate::utils::errors::Error::init(
                    "expected name after type".to_string(),
                    None,
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Parse,
                        None,
                    )),
                )
                .print_error();
                unreachable!()
            }
        };

        if !self.match_type(&[TokenType::Assign]) {
            crate::utils::errors::Error::init(
                "expected '=' after name".to_string(),
                None,
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Parse,
                    None,
                )),
            )
            .print_error();
        }

        let value = self.parse_expression();
        let span = start.join(value.span);

        Statement::new(
            StatementKind::ConstantDeclaration {
                name,
                type_annotation: const_type,
                value,
            },
            span,
        )
    }
}
