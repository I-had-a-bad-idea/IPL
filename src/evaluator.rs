use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

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
    pub variables: HashMap<String, String>,
    pub functions: HashMap<String, HashMap<String, Value>>,
    evaluators: HashMap<String, Evaluator>,
    indentation_stack : Vec<(String, usize)>,
    
    folder: String,
    path: PathBuf,
}

impl Evaluator{
    pub fn new() -> Self{
        Self{
            lines: vec![],
            variables: HashMap::new(),
            functions: HashMap::new(),
            evaluators: HashMap::new(),
            indentation_stack: vec![],
            
            folder: String::new(),
            path: PathBuf::new(),
        }
    }
    pub fn ev_file(&mut self, file: &str) {
        let path = PathBuf::from(file);
        self.path = path.clone();
        self.folder = path
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string();
        let contents = fs::read_to_string(file).expect("Should have been able to read the file");

        self.lines = contents
            .lines()
            .map(|line| line.to_string())
            .filter(|line| !line.trim().is_empty())
            .collect();

        self.lines.push("End of file".to_string());

        self.indentation_stack = vec![("normal".to_string(), 0)];
        
        self.execute_lines(0, self.lines.len());
        println!("variables {:#?}", self.variables);
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
