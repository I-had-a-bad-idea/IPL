
pub struct EvaluatioError{
    message: String,
    line_number: Option<u32>,
    line_content: Option<String>,
}

impl EvaluatioError{
    // Create a new EvaluatioError, with message and optional line number and line content
    pub fn new(message: String, line_number: Option<u32>, line_content: Option<String>) -> Self{
        Self {
            message,
            line_number,
            line_content,
        }
    }   

    pub fn raise(&self) {
        panic!("Error: {} on line {:?}: {:?}", self.message, self.line_number, self.line_content);
    }
}