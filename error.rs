
pub struct EvaluatioError{
    message: String,
    line_number: Option<u32>,
    line_content: Option<String>,
}

impl EvaluatioError{
    pub fn new(message: String, line_number: Option<u32>, line_content: Option<String>) -> Self{
        Self {
            message,
            line_number,
            line_content,
        }
    }   

    pub fn raise(&self) {
        println!("Error: {} on line {:?}: {:?}", self.message, self.line_number, self.line_content);
    }
}