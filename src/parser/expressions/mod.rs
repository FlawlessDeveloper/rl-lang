use std::process::exit;

use crate::{
    ast::nodes::{Expression, ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
};

impl Parser {
    pub fn parse_expression(&mut self) -> Expression {
        // offloads to term for now
        self.parse_equality()
    }

    pub fn parse_equality(&mut self) -> Expression {
        let mut left = self.parse_comparsion();
        while self.match_type(&[TokenType::BangEqual, TokenType::Compare]) {
            let operator = self.previous();
            let right = self.parse_comparsion();
            let span = left.span.join(right.span);
            left = Expression::new(
                ExpressionKind::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                span,
            );
        }
        left
    }

    pub fn parse_comparsion(&mut self) -> Expression {
        let mut left = self.parse_term();
        while self.match_type(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::PlusEqual,
            TokenType::MinusEqual,
            TokenType::StarEqual,
            TokenType::SlashEqual,
        ]) {
            let operator = self.previous();
            let right = self.parse_term();
            let span = left.span.join(right.span);

            match operator {
                TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual => {
                    if let ExpressionKind::Identifier(name) = &left.kind {
                        let name = name.clone();
                        let lhs_span = left.span;
                        let operator = match operator {
                            TokenType::PlusEqual => TokenType::Plus,
                            TokenType::MinusEqual => TokenType::Minus,
                            TokenType::StarEqual => TokenType::Star,
                            TokenType::SlashEqual => TokenType::Slash,
                            _ => unreachable!(),
                        };
                        let binary = Expression::new(
                            ExpressionKind::Binary {
                                left: Box::new(Expression::new(
                                    ExpressionKind::Identifier(name.clone()),
                                    lhs_span,
                                )),
                                operator,
                                right: Box::new(right),
                            },
                            span,
                        );
                        left = Expression::new(
                            ExpressionKind::Assign {
                                name,
                                value: Box::new(binary),
                            },
                            span,
                        );
                    } else {
                        left = Expression::new(
                            ExpressionKind::Binary {
                                left: Box::new(left),
                                operator,
                                right: Box::new(right),
                            },
                            span,
                        );
                    }
                }
                _ => {
                    left = Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        },
                        span,
                    );
                }
            }
        }
        left
    }

    pub fn parse_term(&mut self) -> Expression {
        // left operand into factor to return it if no case match for operator
        let mut left = self.parse_factor();
        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            // get the operator the match_type applied advance on
            let operator = self.previous();
            let right = self.parse_factor();
            let span = left.span.join(right.span);
            left = Expression::new(
                ExpressionKind::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                span,
            );
        }
        left
    }

    pub fn parse_factor(&mut self) -> Expression {
        let mut left = self.parse_unary();
        while self.match_type(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.parse_unary();
            let span = left.span.join(right.span);
            left = Expression::new(
                ExpressionKind::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                span,
            );
        }
        left
    }

    pub fn parse_unary(&mut self) -> Expression {
        let start = self.peek_span();
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let operand = self.parse_unary();
            let span = start.join(operand.span);
            return Expression::new(
                ExpressionKind::Unary {
                    operator,
                    operand: Box::new(operand),
                },
                span,
            );
        }
        self.parse_primary()
    }

    pub fn parse_primary(&mut self) -> Expression {
        log::debug!("current index: {:?}", self.current);
        log::debug!("current token: {:?}", self.peek());

        let start = self.peek_span();

        // is it identifier
        if self.match_type(&[TokenType::Identifier(String::new())]) {
            log::debug!("found identifier");
            let ident_span = self.previous_span();
            if let TokenType::Identifier(name) = self.previous() {
                // is it function call?
                if self.match_type(&[TokenType::LeftParen]) {
                    log::debug!("found function call");
                    let mut args = Vec::new();
                    // need to extract this as helper function that returns bool tho
                    if !(std::mem::discriminant(&self.peek())
                        == std::mem::discriminant(&TokenType::RightParen))
                    {
                        loop {
                            args.push(self.parse_expression());
                            if !self.match_type(&[TokenType::Comma]) {
                                break;
                            }
                        }
                    }
                    self.match_type(&[TokenType::RightParen]);
                    let span = start.join(self.previous_span());
                    return Expression::new(ExpressionKind::Call { name, args }, span);
                }

                // is it assignment?
                if self.match_type(&[TokenType::Assign]) {
                    log::debug!("found variable assignment");
                    let value = self.parse_expression();
                    let span = start.join(value.span);
                    return Expression::new(
                        ExpressionKind::Assign {
                            name,
                            value: Box::new(value),
                        },
                        span,
                    );
                }
                if self.match_type(&[TokenType::LeftBracket]) {
                    let index = self.parse_expression();
                    self.match_type(&[TokenType::RightBracket]);
                    let after_index_span = self.previous_span();

                    let mut expr = Expression::new(
                        ExpressionKind::Index {
                            target: Box::new(Expression::new(
                                ExpressionKind::Identifier(name.clone()),
                                ident_span,
                            )),
                            index: Box::new(index),
                        },
                        start.join(after_index_span),
                    );

                    // consume chained indices for nested arrays: arr[0][1][2]
                    while self.peek() == TokenType::LeftBracket {
                        self.advance();
                        let next_index = self.parse_expression();
                        self.match_type(&[TokenType::RightBracket]);
                        let span = start.join(self.previous_span());
                        expr = Expression::new(
                            ExpressionKind::Index {
                                target: Box::new(expr),
                                index: Box::new(next_index),
                            },
                            span,
                        );
                    }

                    // is it index-assign?
                    if self.match_type(&[TokenType::Assign]) {
                        log::debug!("found array item assignment");
                        let value = self.parse_expression();
                        let span = start.join(value.span);
                        if let ExpressionKind::Index { target, index } = expr.kind {
                            return Expression::new(
                                ExpressionKind::IndexAssign {
                                    target,
                                    index,
                                    value: Box::new(value),
                                },
                                span,
                            );
                        }
                    }

                    return expr;
                }
                return Expression::new(ExpressionKind::Identifier(name), ident_span);
            }
        }

        // is it array literal?
        if self.match_type(&[TokenType::LeftBracket]) {
            let mut items = Vec::new();
            while self.peek() != TokenType::RightBracket {
                items.push(self.parse_expression());
                if self.peek() == TokenType::RightBracket {
                    break;
                }
                if !self.match_type(&[TokenType::Comma]) {
                    crate::utils::errors::Error::init(
                        "expected ',' between array items".to_string(),
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
            return Expression::new(ExpressionKind::ArrayLiteral(items), span);
        }
        // is it integer?
        if self.match_type(&[TokenType::NumberLiteral(0)]) {
            log::debug!("found number");
            let span = self.previous_span();
            if let TokenType::NumberLiteral(n) = self.previous() {
                return Expression::new(ExpressionKind::Integer(n), span);
            }
        }

        // is it String?
        if self.match_type(&[TokenType::StringLiteral(String::new())]) {
            log::debug!("found string");
            let span = self.previous_span();
            if let TokenType::StringLiteral(s) = self.previous() {
                return Expression::new(ExpressionKind::String(s), span);
            }
        }

        // is it character?
        if matches!(
            self.tokens[self.current].token,
            TokenType::CharacterLiteral(_)
        ) {
            self.advance();
            log::debug!("found characher");
            let span = self.previous_span();
            if let TokenType::CharacterLiteral(c) = self.previous() {
                return Expression::new(ExpressionKind::Character(c), span);
            }
        }

        // is it bool?
        if self.match_type(&[TokenType::BoolLiteral(false)]) {
            // log::debug!("found bool");
            let span = self.previous_span();
            if let TokenType::BoolLiteral(b) = self.previous() {
                return Expression::new(ExpressionKind::Bool(b), span);
            }
        }

        // is it float??
        if self.match_type(&[TokenType::FloatLiteral(0.0)]) {
            log::debug!("oh no found float");
            let span = self.previous_span();
            if let TokenType::FloatLiteral(f) = self.previous() {
                return Expression::new(ExpressionKind::Float(f), span);
            }
        }

        // is it (Expression)?
        if self.match_type(&[TokenType::LeftParen]) {
            log::debug!("found group start");
            let inner = self.parse_expression();
            self.match_type(&[TokenType::RightParen]);
            let span = start.join(self.previous_span());
            return Expression::new(ExpressionKind::Grouping(Box::new(inner)), span);
        }

        // panic
        crate::utils::errors::Error::init(
            "Expected expression".to_string(),
            None,
            Some(crate::utils::errors::ErrorReason::init(
                crate::utils::errors::Reason::Parse,
                None,
            )),
        )
        .print_error();
        exit(0)
    }
}
