use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::ops::Index;
use std::ops::Add;

use crate::error::EvaluatioError;
use crate::tokenizer::Tokenizer;

#[derive(Debug, Clone)]
pub enum Value{
    Number(f64),
    List(Vec<Value>),
    Bool(bool),
    Str(String),
    None,
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, idx: usize) -> &Self::Output {
        match self {
            Value::List(vec) => &vec[idx],
            _ => panic!("Indexing only supported on Value::List"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
            (Value::Str(a), Value::Number(b)) => Value::Str(a + &b.to_string()),
            (Value::Number(a), Value::Str(b)) => Value::Str(a.to_string() + &b),
            _ => Value::None,
        }
    }
}

impl Value {
    fn as_f64(&self) -> f64 {
        match self {
            Value::Number(n) => n.clone(),
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            _ => 0.0,
        }
    }
    fn as_usize(&self) -> usize {
        match self {
            Value::Number(n) => *n as usize,
            Value::Bool(b) => if *b { 1 } else { 0 }
            _ => 0,
        }
    }
    fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::None => false,
            _ => true,
        }
    }
    fn to_string_value(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::None => "None".into(),
            _ => "".to_string(),
            }
        }
    fn length(&self) -> usize {
        match self {
            Value::List(v) => v.len(),
            Value::Str(s) => s.len(),
            _ => 0,
        }
    }
    fn iter(&self) -> Box<dyn Iterator<Item = &Value> + '_> {
        match self {
            Value::List(v) => Box::new(v.iter()),
            _ => Box::new(std::iter::empty()),
        }
    }
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
    fn get_indentation(line: &str) -> usize {
        line.chars().take_while(|c| c.is_whitespace()).count()
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
        let tokens = Tokenizer::new().tokenize(expr, self.variables.clone(), self.functions.clone());

        let stack = vec![];
        for token in tokens{
            if token.
        }
        // Placeholder for expression evaluation logic
        println!("Evaluating expression: {}", expr);
        return Value::None;
    }
    fn ev_func(&mut self, function_name: &str, args: &[Value]) -> Value {
        let file = &self.functions[function_name]["file"];
        if file.to_string_value() != self.path.to_str().unwrap() {
            if let Some(ev) = self.evaluators.get_mut(&file.to_string_value()) {
                return ev.ev_func(function_name, args);
            }   else {
                EvaluatioError::new("Evaluator for file not found".to_string(), None, None).raise();
                }
        }

        let function_arguments = &self.functions[function_name]["arguments"];
        let function_lines = &self.functions[function_name]["function_body"];

        println!("Executing function {} with lines: {:?}", function_name, function_lines);
        println!("Function lines content:");
        for i in function_lines.iter() {
            println!("  {:?}: '{}'", i, self.lines[i.as_usize()]);
        }
        if args.len() != function_arguments.length() {
            EvaluatioError::new("Wrong amount of arguments".to_string(), None, None).raise();
        }
        let global_variables = self.variables.clone();

        for (name, value) in function_arguments.iter().zip(args.iter()) {
            self.variables.insert(name.to_string_value(), format!("{:?}", value));
        }
        self.indentation_stack.push(("function".to_string(), Self::get_indentation(&self.lines[function_lines[0].as_usize()])));

        let result = self.execute_lines(function_lines[0].as_usize(), (function_lines[function_lines.length() - 1].clone() + Value::Number(1.0)).as_usize());
        
        self.variables = global_variables;
        self.indentation_stack.pop();

        return result;

        // Placeholder for function evaluation logic

    }


//         for name, value in zip(function_arguments, arguments):
//             self.variables[name] = value

//         self.indentation_stack.append(("function", get_indentation(self.lines[function_lines[0]])))

//         result = self.execute_lines(function_lines[0], function_lines[-1] + 1)

//         self.variables = global_variables

//         self.indentation_stack.pop()

//         return result

}
