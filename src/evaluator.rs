use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::ops::Index;
use std::ops::Add;
use std::result;

use crate::built_in_functions::call_built_in_function;
use crate::built_in_functions::BUILT_IN_FUNCTIONS;
use crate::debug::EvaluatioError;
use crate::tokenizer::Tokenizer;


#[derive(Debug, Clone)]
pub enum Value{
    Number(f64),
    List(Vec<Value>),
    Bool(bool),
    Str(String),
    Path(PathBuf),
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
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, HashMap<String, Value>>,
    evaluators: HashMap<String, Evaluator>,
    indentation_stack : Vec<(String, usize)>,
    
    folder: String,
    path: PathBuf,
}

fn get_indentation(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
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
        self.folder += "//";
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

    fn ev_func(&mut self, function_name: &str, args: Vec<Value>) -> Value {
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
            self.variables.insert(name.to_string_value(), value.clone());
        }
        self.indentation_stack.push(("function".to_string(), get_indentation(&self.lines[function_lines[0].as_usize()])));

        let result = self.execute_lines(function_lines[0].as_usize(), (function_lines[function_lines.length() - 1].clone() + Value::Number(1.0)).as_usize());
        
        self.variables = global_variables;
        self.indentation_stack.pop();

        return result;

        // Placeholder for function evaluation logic

    }

    fn execute_lines(&mut self, start: usize, end: usize) -> Value {
        let mut programm_counter: usize = start;

        println!("execute_lines called with start {} and end {}", start, end);

        while programm_counter < end{
            println!("At line {}", programm_counter);

            let mut line = self.lines[programm_counter].clone();
            line = line.split("#").collect::<Vec<_>>()[0].to_string();

            let indentation = get_indentation(&line);

            println!("Indentation_stack: {:?}", self.indentation_stack);
            if indentation <= self.indentation_stack[self.indentation_stack.len() - 1].1{
                if self.indentation_stack[self.indentation_stack.len() - 1].0 == "while"{
                    while self.lines[programm_counter].split(" ").collect::<Vec<_>>()[0] != "while"{
                        programm_counter -= 1;
                    }
                    self.indentation_stack.pop();
                    continue
                }
            } else if self.indentation_stack[self.indentation_stack.len() - 1].0 == "if"{
                self.indentation_stack.pop();
            } else if self.indentation_stack[self.indentation_stack.len() - 1].0 == "else"{
                self.indentation_stack.pop();
            }
            let first_word = line
                .splitn(2, ' ')
                .next();
            
            let first_word = match first_word {
                Some(word) if !word.is_empty() => word,
                _ => {
                    programm_counter += 1;
                    continue;}
            };
            match line.split(" ").collect::<Vec<_>>()[0]{
                "import" => {
                    let file = self.folder.clone() + line.split(" ").collect::<Vec<_>>()[1];
                    println!("File path to be impprted: {}", file);
                    self.evaluators.insert(file.clone(), Evaluator::new());
                    if let Some(evaluator) = self.evaluators.get_mut(&file) {
                        evaluator.ev_file(&file);
                    }
                    self.functions.extend(self.evaluators[&file].functions.clone());
                    self.variables.extend(self.evaluators[&file].variables.clone());

                    programm_counter += 1;
                    }
                "while" => {
                    let mut result = false;
                    if let Some((_first, rest)) = line.split_once(' ') {
                        result = self.ev_expr(rest).as_bool(); // or whatever you want to do with `rest`
                    }
                    if result == true{
                        programm_counter += 1;
                        self.indentation_stack.push(("while".to_string(), indentation));
                    } else{
                        programm_counter += 1;
                        while get_indentation(&self.lines[programm_counter].clone()) > self.indentation_stack[self.indentation_stack.len() - 1].1{
                            programm_counter += 1
                        }
                    }
                }
                "if" => {
                    let mut result = false;
                    if let Some((_first, rest)) = line.split_once(' ') {
                        result = self.ev_expr(rest).as_bool(); // or whatever you want to do with `rest`
                    }
                    if result == true{
                        programm_counter += 1;
                        self.indentation_stack.push(("if".to_string(), indentation));
                    }
                    else{
                        loop{
                            programm_counter += 1;
                            while get_indentation(&self.lines[programm_counter].clone()) > self.indentation_stack[self.indentation_stack.len() - 1].1{
                                programm_counter += 1;
                            }
                            if self.lines[programm_counter].split(" ").collect::<Vec<_>>()[0] == "else" && get_indentation(&self.lines[programm_counter].clone()) == indentation{
                                programm_counter += 1;
                                self.indentation_stack.push(("else".to_string(), indentation));
                                break
                            }
                            else if self.lines[programm_counter].split(" ").collect::<Vec<_>>()[0] == "elif" && get_indentation(&self.lines[programm_counter].clone()) == indentation{                                
                                let mut result = false;
                                if let Some((_first, rest)) = line.split_once(' ') {
                                    result = self.ev_expr(rest).as_bool(); // or whatever you want to do with `rest`
                                }
                                if result == true{
                                    programm_counter += 1;
                                    self.indentation_stack.push(("if".to_string(), indentation));
                                    break;
                                }
                                else{continue;}
                            }
                            else{continue;}
                        }
                    }
                }
                "else" => {
                    programm_counter += 1;
                    while get_indentation(&self.lines[programm_counter].clone()) > self.indentation_stack[self.indentation_stack.len() - 1].1{
                        programm_counter += 1;
                    }
                }
                "elif" => {
                    programm_counter += 1;
                    while get_indentation(&self.lines[programm_counter].clone()) > self.indentation_stack[self.indentation_stack.len() - 1].1{
                        programm_counter += 1;
                    }
                }
                "return" => {
                    let expr = line.split("return").collect::<Vec<_>>()[1];
                    println!("Returning result of {}", expr);
                    return self.ev_expr(expr);
                }
                "def" => {
                    let function_decleration = line.split(" ").collect::<Vec<_>>()[1];
                    let function_name = function_decleration.split("(").collect::<Vec<_>>()[0];
                    let args = function_decleration
                                        .split_once('(') // returns Option<(&str, &str)>
                                        .and_then(|(_, rest)| rest.split_once(')')) // safely get the inside of the parentheses
                                        .map(|(args_str, _)| {
                                            args_str
                                                .split(',')
                                                .map(str::trim)
                                                .filter(|s| !s.is_empty())
                                                .collect::<Vec<_>>()
                                        })
                                        .unwrap_or_else(|| Vec::new());
                    let function_arguments = args.iter().map(|n| Value::Str(n.to_string())).collect::<Vec<Value>>();
                    programm_counter += 1;
                    let start_line = programm_counter;
                    while get_indentation(&self.lines[programm_counter]) > indentation{
                        programm_counter += 1;
                    }
                    let function_lines = (start_line..programm_counter)
                        .map(|n| Value::Number(n as f64))
                        .collect::<Vec<Value>>();
                    let mut function_hash_map: HashMap<String, Value> = HashMap::new();
                    function_hash_map.insert("file".to_string(), Value::Path(self.path.clone()));
                    function_hash_map.insert("arguments".to_string(), Value::List(function_arguments));
                    function_hash_map.insert("function_body".to_string(), Value::List(function_lines));
                    self.functions.insert(function_name.to_string(), function_hash_map);

                    println!("functions: {:?}", self.functions);
                }
                _ => {
                    if line == "End of file"{
                        break;
                    }
                    if line.contains("="){
                        println!("Is assignment");
                        if let Some((mut variable_name, expr)) = line.split_once("="){
                            variable_name = variable_name.trim();
                            println!("Variable name: {}, expr: {}", variable_name, expr);
                            let result = self.ev_expr(expr);
                            self.variables.insert(variable_name.to_string(), result);
                        }
                    }
                    else{
                        self.ev_expr(&line);
                    }
                    programm_counter += 1;
                }
            }
        }
        return Value::None;
    }

    fn ev_expr(&mut self, expr: &str) -> Value {
        let tokens = Tokenizer::new().tokenize(expr, self.variables.clone(), self.functions.clone());

        println!("tokens: {:?}", tokens);

        let mut stack = vec![];
        let mut i = 0;
        while i < tokens.len(){
            let token = tokens.get(i).expect("Empty token").to_string();
            println!("token: {}", token);
            if token.trim_matches('.').parse::<f64>().is_ok(){
                stack.push(token);
            }
            else if (token.starts_with('"') && token.ends_with('"')) || (token.starts_with("'") && token.ends_with("'")){
                stack.push((token[1..token.len()-1]).to_string());
            }
            else if self.variables.contains_key(&token) {
                stack.push(self.variables[&token].clone().to_string_value())
            }
            else if self.functions.contains_key(&token)|| BUILT_IN_FUNCTIONS.contains_key(&token as &str) {
                println!("Found function");
                let function_name = &token;
                let mut args: Vec<Value> = vec![];
                let mut func_i = i + 1;
                let required_args = BUILT_IN_FUNCTIONS[&function_name as &str].clone();
                loop{
                    if args.len() >= required_args.len(){
                        i += func_i - i - 1;
                        break;
                    }
                    let func_token = tokens.get(func_i).expect("Empty token").to_string();
                    args.push(self.ev_expr(&func_token));
                    if func_i + 1 >= tokens.len(){
                        i += func_i - i - 1;
                        break;
                    }
                    if tokens.get(func_i + 1).expect("Empty token").to_string() != ","{
                        i += func_i - i - 1;
                        break
                    }
                    func_i += 2;
                }
                if BUILT_IN_FUNCTIONS.contains_key(function_name as &str){
                    stack.push(call_built_in_function(function_name, args).to_string_value());
                }
                else if self.functions.contains_key(function_name) {
                    stack.push(self.ev_func(function_name, args).to_string_value());
                }
                i += func_i - i - 1;
            }
            else{
                let rhs = Value::Str(stack.pop().unwrap()).as_f64();
                let lhs = Value::Str(stack.pop().unwrap()).as_f64();

                if token == "+"{
                    stack.push((lhs + rhs).to_string());
                }
                else if token == "-" {
                    stack.push((lhs - rhs).to_string());
                }
                else if token == "*" {
                    stack.push((lhs * rhs).to_string());
                }
                else if token == "/" {
                    stack.push((lhs / rhs).to_string());
                }

                else if token == "==" {
                    stack.push((lhs == rhs).to_string());
                }
                else if token == "!=" {
                    stack.push((lhs != rhs).to_string());
                }
                else if token == "<" {
                    stack.push((lhs < rhs).to_string());
                }
                else if token == "<=" {
                    stack.push((lhs <= rhs).to_string());
                }
                else if token == ">" {
                    stack.push((lhs > rhs).to_string());
                }
                else if token == ">=" {
                    stack.push((lhs >= rhs).to_string());
                }

                else if token == "and" {
                    stack.push((Value::Number(lhs).as_bool() && Value::Number(rhs).as_bool()).to_string());    
                }
                else if token == "or" {
                    stack.push((Value::Number(lhs).as_bool() || Value::Number(rhs).as_bool()).to_string());
                }
            }
            i += 1;
        }
    return Value::Str(stack[0].clone());
    }

}
