use crate::scanner::token::{Token, TokenKind};
use std::cell::Cell;

pub mod parse_rule;

pub struct Parser {
    pub current: Token,
    pub previous: Token,
    pub had_error: Cell<bool>,
    pub is_panic_mode: Cell<bool>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token {
                kind: TokenKind::EOF,
                lexeme: "".to_string(),
                line_number: 0,
            },
            previous: Token {
                kind: TokenKind::EOF,
                lexeme: "".to_string(),
                line_number: 0,
            },
            had_error: Cell::new(false),
            is_panic_mode: Cell::new(false),
        }
    }

    pub fn error(&self, message: &str) {
        self.error_at(&self.previous, message);
    }

    pub fn error_at_current(&self, message: &str) {
        self.error_at(&self.current, message);
    }

    pub fn error_at(&self, token: &Token, message: &str) {
        use TokenKind::*;

        if self.is_panic_mode.get() {
            return;
        }

        self.is_panic_mode.set(true);

        eprint!("[line {}] Error", token.line_number);

        if token.kind == EOF {
            eprint!(" at end");
        } else if token.kind == Error {
        } else {
            eprint!(" at '{}'", token.lexeme);
        }

        eprintln!(": {message}");
        self.had_error.set(true);
    }
}
