use std::collections::HashMap;

enum Value{
    Number(f64),
    List(Vec<Value>),
    Bool(bool),
    Str(String),
    None,
}

pub struct Evaluator{
    lines: Vec<String>,
    variables: HashMap<String, String>,
    functions: HashMap<String, HashMap<String, Value>>,
    evaluators: HashMap<String, Evaluator>,
    indentation_stack : Vec<usize>,
}

impl Evaluator{
    pub fn new() -> Self{
        Self{

        }
    }
    pub fn ev_file(&self, file: &str) {
        // Placeholder for file evaluation logic
        println!("Evaluating file: {}", file);
    }
    fn execute_lines(&self, start: usize, end: usize) {
        // Placeholder for line execution logic
        println!("Executing lines from {} to {}", start, end);
    }
    fn ev_expr(&self, expr: &str) -> Value {
        // Placeholder for expression evaluation logic
        println!("Evaluating expression: {}", expr);
        Value::None
    }
    fn ev_func(&self, name: &str, args: Vec<Value>) -> Value {
        // Placeholder for function evaluation logic
        println!("Evaluating function: {} with args {:?}", name, args);
        Value::None
    }
}
