use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value{
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
            lines: vec![],
            variables: HashMap::new(),
            functions: HashMap::new(),
            evaluators: HashMap::new(),
            indentation_stack: vec![],
        }
    }
    pub fn ev_file(&self, file: &str) {
        // Placeholder for file evaluation logic
        println!("Evaluating file: {}", file);
    }
    fn execute_lines(&self, start: usize, end: usize) -> Value {
        // Placeholder for line execution logic
        println!("Executing lines from {} to {}", start, end);
        return Value::None;
    }

    fn ev_expr(&self, expr: &str) -> Value {
        // Placeholder for expression evaluation logic
        println!("Evaluating expression: {}", expr);
        return Value::None;
    }
    fn ev_func(&self, name: &str, args: &[Value]) -> Value {
        // Placeholder for function evaluation logic
        println!("Evaluating function: {} with args {:?}", name, args);
        return Value::None;
    }
}
