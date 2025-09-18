use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

use crate::error::EvaluatioError;

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
        // Placeholder for expression evaluation logic
        println!("Evaluating expression: {}", expr);
        return Value::None;
    }
    fn ev_func(&self, function_name: &str, args: &[Value]) -> Value {
        let file = self.functions[function_name]["file"];
        if file != self.path.to_str().unwrap() {
            return self.evaluators[file].ev_func(function_name, args);
        }

        let function_arguments = self.functions[function_name]["arguments"];
        let function_lines = self.functions[function_name]["function_body"];

        println!("Executing function {} with lines: {:?}", function_name, function_lines);
        println!("Function lines content:");
        for i in function_lines {
            println!("  {}: '{}'", i, self.lines[i]);
        }
        if args.len() != function_arguments.len() {
            EvaluatioError::new("Wrong amount of arguments".to_string(), None, None).raise();
        }
        let global_variables = self.variables.clone();

        for name, value in function_arguments.iter().zip(args.iter()) {
            self.variables.insert(name.clone(), format!("{:?}", value));
        }
        self.indentation_stack.push(("function".to_string(), Self::get_indentation(&self.lines[function_lines[0]])));

        let result = self.execute_lines(function_lines[0], function_lines[function_lines.len() - 1] + 1);
        
        self.variables = global_variables;
        self.indentation_stack.pop();

        return result;

        // Placeholder for function evaluation logic
        println!("Evaluating function: {} with args {:?}", function_name, args);
        return Value::None;

    }


//         for name, value in zip(function_arguments, arguments):
//             self.variables[name] = value

//         self.indentation_stack.append(("function", get_indentation(self.lines[function_lines[0]])))

//         result = self.execute_lines(function_lines[0], function_lines[-1] + 1)

//         self.variables = global_variables

//         self.indentation_stack.pop()

//         return result

}
