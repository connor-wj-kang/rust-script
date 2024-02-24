use crate::scanner::token::TokenKind;

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_grouping(&mut self) {
        self.parse_expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression");
    }
}