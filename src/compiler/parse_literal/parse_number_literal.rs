use super::Compiler;
use crate::value::Value;

impl Compiler {
    pub fn parse_number_literal(&mut self, _can_assign: bool) {
        let number = self.parser().previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(number));
    }
}
