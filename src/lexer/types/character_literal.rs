use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// scans a single quoted literal
    ///
    /// only single character are allowed
    /// returns TokenType::CharacterLiteral
    pub fn character_literal(&mut self) {
        self.advance();

        if self.is_at_end() {
            crate::utils::errors::Error::init(
                "unterminated character literal".to_string(),
                Some(self.line),
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Lexer,
                    None,
                )),
            )
            .print_error();
            return;
        }

        let character = self.source[self.current - 1];
        let value: char = if character == '\\' {
            // escape sequence
            if self.is_at_end() {
                crate::utils::errors::Error::init(
                    "unterminated character literal".to_string(),
                    Some(self.line),
                    Some(crate::utils::errors::ErrorReason::init(
                        crate::utils::errors::Reason::Lexer,
                        None,
                    )),
                )
                .print_error();
                return;
            }
            let escaped = self.source[self.current];
            self.advance();
            match escaped {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '0' => '\0',
                _ => {
                    crate::utils::errors::Error::init(
                        format!("unknown escape sequence '\\{}'", escaped),
                        Some(self.line),
                        Some(crate::utils::errors::ErrorReason::init(
                            crate::utils::errors::Reason::Lexer,
                            None,
                        )),
                    )
                    .print_error();
                    return;
                }
            }
        } else {
            character
        };

        if self.peek() != '\'' {
            crate::utils::errors::Error::init(
                "unterminated character literal".to_string(),
                Some(self.line),
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Lexer,
                    None,
                )),
            )
            .print_error();
            return;
        }

        self.advance();

        self.add_token(TokenType::CharacterLiteral(value));
    }
}
