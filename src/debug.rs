use crate::state;

pub struct EvaluatioError{
    message: String,
    line_number: usize,
    line_content: String,
}

impl EvaluatioError{
    // Create a new EvaluatioError, with message and optional line number and line content
    pub fn new(message: String) -> Self{
        Self {
            message: message,
            line_number: state::get_line_number(),
            line_content: state::get_line_content(),
        }
    }   

    pub fn raise(&self) {
        panic!("Error: {} on line {:?}: {:?}", self.message, self.line_number, self.line_content);
    }
}