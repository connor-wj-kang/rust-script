use super::Compiler;

impl Compiler {
    pub fn parse_self(&mut self, _can_assign: bool) {
        if self.current_class.borrow().is_none() {
            self.parser().error("Can't use 'self' outside of a class.");
            return;
        }
        self.parse_variable_expression(false);
    }
}
