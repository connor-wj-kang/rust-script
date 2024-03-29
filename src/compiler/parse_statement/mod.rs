use super::Compiler;
use crate::scanner::token::TokenKind;

mod parse_block_statement;
mod parse_break_statement;
mod parse_continue_statement;
mod parse_expression_statement;
mod parse_for_statement;
mod parse_if_statement;
mod parse_loop_statement;
mod parse_print_statement;
mod parse_return_statement;
mod parse_while_statement;

impl Compiler {
    pub fn parse_statement(&mut self) {
        use TokenKind::*;

        if self.matches(Print) {
            self.parse_print_statement();
        } else if self.matches(For) {
            self.parse_for_statement();
        } else if self.matches(If) {
            self.parse_if_statement();
        } else if self.matches(Return) {
            self.parse_return_statement();
        } else if self.matches(While) {
            self.parse_while_statement();
        } else if self.matches(LeftBrace) {
            self.begin_scope();
            self.parse_block_statement();
            self.end_scope();
        } else if self.matches(Break) {
            self.parse_break_statement();
        } else if self.matches(Loop) {
            self.parse_loop_statement();
        } else if self.matches(Continue) {
            self.parse_continue_statement();
        } else {
            self.parse_expression_statement();
        }
    }
}
