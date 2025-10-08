use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::ops::Index;
use std::ops::Add;

use crate::built_in_functions::call_built_in_function;
use crate::built_in_functions::BUILT_IN_FUNCTIONS;
use crate::debug::EvaluatioError;
use crate::tokenizer::Tokenizer;

#[derive(Debug, Clone)]
pub struct Class {
    file: PathBuf,
    body: Value,
    functions: HashMap<String, HashMap<String, Value>>,
    variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Instance {
    class: Class,
    variables: HashMap<String, Value>,
}

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
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Number(n) => n.clone(),
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            Value::Str(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    pub fn as_usize(&self) -> usize {
        match self {
            Value::Number(n) => *n as usize,
            Value::Bool(b) => if *b { 1 } else { 0 }
            _ => 0,
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::None => false,
            _ => true,
        }
    }
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
            Value::Path(p) => p.to_str().unwrap_or("").to_string(),
            Value::None => "None".into(),
            _ => "".to_string(),
            }
        }
    pub fn length(&self) -> usize {
        match self {
            Value::List(v) => v.len(),
            Value::Str(s) => s.len(),
            _ => 0,
        }
    }
    pub fn iter(&self) -> Box<dyn Iterator<Item = &Value> + '_> {
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
    pub classes: HashMap<String, Class>,
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
            variables: HashMap::from([
                ("True".to_string(), Value::Bool(true)),
                ("False".to_string(), Value::Bool(false)),
                ("None".to_string(), Value::None),
            ]),
            functions: HashMap::new(),
            evaluators: HashMap::new(),
            classes: HashMap::new(),
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
        
        self.execute_lines(0, self.lines.len(), "".to_string());
        println!("variables {:#?}", self.variables);
        println!("classes {:#?}", self.classes);
        println!("functions {:#?}", self.functions);
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

        // println!("Executing function {} with lines: {:?}", function_name, function_lines);
        // println!("Function lines content:");
        // for i in function_lines.iter() {
        //     println!("  {:?}: '{}'", i, self.lines[i.as_usize()]);
        //}
        if args.len() != function_arguments.length() {
            EvaluatioError::new("Wrong amount of arguments".to_string(), None, None).raise();
        }
        let global_variables = self.variables.clone();

        for (name, value) in function_arguments.iter().zip(args.iter()) {
            self.variables.insert(name.to_string_value(), value.clone());
        }
        self.indentation_stack.push(("function".to_string(), get_indentation(&self.lines[function_lines[0].as_usize()])));

        let result = self.execute_lines(function_lines[0].as_usize(), (function_lines[function_lines.length() - 1].clone() + Value::Number(1.0)).as_usize(), "".to_string());
        
        self.variables = global_variables;
        self.indentation_stack.pop();

        return result;

        // Placeholder for function evaluation logic

    }

    fn execute_lines(&mut self, start: usize, end: usize, self_value: String) -> Value {
        let mut programm_counter: usize = start;

        println!("execute_lines called with start {} and end {}", start, end);

        while programm_counter < end{
            // println!("At line {}", programm_counter);

            let mut line = self.lines[programm_counter].clone();
            line = line.split("#").collect::<Vec<_>>()[0].to_string();

            // println!("Current line: '{}'", line);

            let indentation = get_indentation(&line);

            line = line.trim().to_string();

            // println!("Indentation_stack: {:?}", self.indentation_stack);
            if indentation <= self.indentation_stack.last().unwrap_or(&("".to_string(), 0)).1{
                if self.indentation_stack.last().is_none(){
                    return Value::None;
                }
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

            match line.split(" ").collect::<Vec<_>>()[0]{
                "import" => {
                    let file = self.folder.clone() + line.split(" ").collect::<Vec<_>>()[1];
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
                        result = self.ev_expr(rest).as_bool();
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
                "for" =>{
                    let variable_name = line.split(" ").collect::<Vec<_>>()[1];
                    let iterable_expr = line.split("in").collect::<Vec<_>>()[1].trim();
                    // println!("For loop variable: {}, iterable expression: {}", variable_name, iterable_expr);
                    let iterable = self.ev_expr(iterable_expr); 
                    // println!("Iterable evaluated to: {:?}", iterable);
                    let start_line = programm_counter + 1;
                    let mut end_line = start_line;
                    while get_indentation(&self.lines[end_line]) > indentation{
                        end_line += 1;
                    }

                    self.indentation_stack.push(("for".to_string(), indentation));
                    for value in iterable.iter(){
                        self.variables.insert(variable_name.to_string(), value.clone());
                        self.execute_lines(start_line, end_line, "".to_string());
                    }
                    self.indentation_stack.pop();
                    programm_counter = end_line;
                }
                "if" => {
                    let mut result = false;
                    if let Some((_first, rest)) = line.split_once(' ') {
                        result = self.ev_expr(rest).as_bool();
                    }
                    if result == true{
                        programm_counter += 1;
                        self.indentation_stack.push(("if".to_string(), indentation));
                    }
                    else{
                        programm_counter += 1;
                        while programm_counter < end{
                            let current_line = self.lines[programm_counter].clone();
                            let current_indent = get_indentation(&current_line);
                            let first_word = current_line.split_whitespace().next().unwrap_or("");
                            
                            if current_indent > indentation{
                                programm_counter += 1;
                                continue;
                            }
                            if first_word == "elif" {
                                if let Some((_first, rest)) = current_line.split_once(' ') {
                                    if self.ev_expr(rest).as_bool() {
                                        programm_counter += 1;
                                        self.indentation_stack.push(("if".to_string(), indentation));
                                        break;
                                    } else {
                                        // Skip block
                                        programm_counter += 1;
                                        while programm_counter < end && get_indentation(&self.lines[programm_counter]) > indentation {
                                            programm_counter += 1;
                                        }
                                    }
                                }
                            } else if first_word == "else" {
                                programm_counter += 1;
                                self.indentation_stack.push(("else".to_string(), indentation));
                                break;
                            } else if current_indent <= indentation{
                                break;
                            } else {
                                programm_counter += 1;
                                break;
                            } 
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
                "break" => {
                    while let Some(x) = self.indentation_stack.pop(){
                        if x.0 == "while"{
                            while get_indentation(&self.lines[programm_counter]) > x.1{
                                programm_counter += 1;
                            }
                            break;
                       }
                        else if x.0 == "normal"{
                            EvaluatioError::new("Error: 'break' outside loop".to_string(), None, None).raise();
                        }
                    }
                } 
                "continue" => {
                    while let Some(x) = self.indentation_stack.pop(){
                        if x.0 == "while"{
                            while self.lines[programm_counter].split(" ").collect::<Vec<_>>()[0] != "while"{
                                programm_counter -= 1;
                                continue;
                            }
                        }
                        else if x.0 == "for" {
                            return Value::None; // Stop execution of current iteration
                        }
                        else if x.0 == "normal"{
                            EvaluatioError::new("Error: 'continue' outside loop".to_string(), None, None).raise();
                        }
                    }
                } 
                "return" => {
                    let expr = line.split("return").collect::<Vec<_>>()[1];
                    return self.ev_expr(expr);
                }
                "class" => {
                    let class_name = line.split(" ").collect::<Vec<_>>()[1];
                    let start_line = programm_counter + 1;
                    let mut end_line = start_line;
                    while get_indentation(&self.lines[end_line]) > indentation{
                        end_line += 1;
                    }
                    
                    self.indentation_stack.push(("class".to_string(), indentation + 1));
                    let vars = self.variables.clone();
                    let funcs = self.functions.clone();
                    // self.variables.clear();
                    // self.functions.clear();

                    let function_lines = (start_line..end_line)
                        .map(|n| Value::Number(n as f64))
                        .collect::<Vec<Value>>();
                    self.classes.insert(class_name.to_string(), Class {
                        functions: HashMap::new(),
                        variables: HashMap::new(),
                        file: self.path.clone(),
                        body: Value::List(function_lines),
                    });

                    self.execute_lines(start_line, end_line, class_name.to_string());

                    // self.classes.get_mut(class_name).unwrap().functions = self.functions.clone();
                    // self.classes.get_mut(class_name).unwrap().variables = self.variables.clone();

                    // self.variables = vars;
                    // self.functions = funcs;

                    self.indentation_stack.pop();
                    programm_counter = end_line;
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

                }
                _ => {
                    if line == "End of file"{
                        break;
                    }
                    if line.contains("="){
                        if let Some((mut variable_name, expr)) = line.split_once("="){
                            let result = self.ev_expr(expr);

                            variable_name = variable_name.trim();
                            if variable_name.contains("self"){
                                if self_value == "".to_string(){
                                    EvaluatioError::new("self used outside class".to_string(), None, None).raise();
                                }
                                else{
                                    let var_name = variable_name.split(".").collect::<Vec<_>>()[1];
                                    self.classes.get_mut(&self_value).unwrap().variables.insert(var_name.to_string(), result);
                                }
                                
                            }
                            else{
                                self.variables.insert(variable_name.to_string(), result);
                            }
                            // println!("Variable name: {}, expr: {}", variable_name, expr);
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

        // println!("tokens: {:?}", tokens);

        let mut stack: Vec<Value> = vec![];
        let mut i = 0;
        while i < tokens.len(){
            let token = tokens.get(i).expect("Empty token");
            let token_str = token.to_string_value();
            // println!("token: {:?} , stack: {:?}", token, stack);
            if token_str.trim_matches('.').parse::<f64>().is_ok(){
                stack.push(Value::Number(token_str.parse::<f64>().unwrap()));
            }
            else if (token_str.starts_with('"') && token_str.ends_with('"')) || (token_str.starts_with("'") && token_str.ends_with("'")){
                stack.push(Value::Str(token_str[1..token_str.len()-1].to_string()));
            }
            else if matches!(token, Value::List(_)) {
                stack.push(token.clone());
            }
            else if self.variables.contains_key(&token_str) {
                stack.push(self.variables[&token_str].clone());
            }
            else if self.functions.contains_key(&token_str)|| BUILT_IN_FUNCTIONS.contains_key(&token_str as &str) {
                let function_name = &token_str;
                let mut args: Vec<Value> = vec![];
                // println!("Function call detected: {}", function_name);
                // println!("Functions: {:?}", self.functions);
                let function_args = tokens.get(i + 1);
                for arg in function_args.unwrap_or(&Value::None).iter(){
                    if let Value::Str(s) = arg {
                        let evaluated_arg = self.ev_expr(s);
                        args.push(evaluated_arg);
                    } else {
                        args.push(arg.clone());
                    }
                }
                if args.len() == 1 && args[0].to_string_value() == Value::None.to_string_value(){
                    args = vec![];
                }
                // println!("Function {} called with arguments: {:?}", function_name, args);
                let result = if BUILT_IN_FUNCTIONS.contains_key(function_name as &str) {
                    call_built_in_function(function_name, args)
                } else if self.functions.contains_key(function_name) {
                    self.ev_func(function_name, args)
                } else {
                    Value::None
                };
                stack.push(result);
                i += 1; // Skip the next token which is the argument list
            }
            else{ 

                let rhs = stack.pop().expect("Not enough values on stack");
                let lhs = stack.pop().expect("Not enough values on stack");
                

                if token_str == "+"{
                    stack.push(Value::Number(lhs.as_f64() + rhs.as_f64()));
                }
                else if token_str == "-" {
                    stack.push(Value::Number(lhs.as_f64() - rhs.as_f64()));
                }
                else if token_str == "*" {
                    stack.push(Value::Number(lhs.as_f64() * rhs.as_f64()));
                }
                else if token_str == "/" {
                    stack.push(Value::Number(lhs.as_f64() / rhs.as_f64()));
                }

                else if token_str == "==" {
                    stack.push(Value::Bool(lhs.as_f64() == rhs.as_f64()));
                }
                else if token_str == "!=" {
                    stack.push(Value::Bool(lhs.as_f64() != rhs.as_f64()));
                }
                else if token_str == "<" {
                    stack.push(Value::Bool(lhs.as_f64() < rhs.as_f64()));
                }
                else if token_str == "<=" {
                    stack.push(Value::Bool(lhs.as_f64() <= rhs.as_f64()));
                }
                else if token_str == ">" {
                    stack.push(Value::Bool(lhs.as_f64() > rhs.as_f64()));
                }
                else if token_str == ">=" {
                    stack.push(Value::Bool(lhs.as_f64() >= rhs.as_f64()));
                }

                else if token_str == "and" {
                    stack.push(Value::Bool(lhs.as_bool() && rhs.as_bool()));    
                }
                else if token_str == "or" {
                    stack.push(Value::Bool(lhs.as_bool() || rhs.as_bool()));
                }
            }
            i += 1;
        }
    return stack.pop().unwrap_or(Value::None); 
    }

}
