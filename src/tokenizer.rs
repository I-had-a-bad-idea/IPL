use regex::Regex;
use std::{collections::HashMap};

use crate::{debug::EvaluatioError, evaluator::Value, built_in_functions::BUILT_IN_FUNCTIONS};

pub struct Tokenizer{
}

impl Tokenizer{
    pub fn new() -> Self{
        Self {
        }
    }
    pub fn tokenize(&self, input: &str, variables: HashMap<String, Value>, functions: HashMap<String, HashMap<String, Value>>) -> Vec<Value> {
        // Placeholder for tokenization logic
        let tokens = self.split(input);
        //println!("tokens after splitting: {:?}", tokens);
        let output = self.shunting_yard(tokens, variables, functions);
        return output;
    }

    fn split(&self, input: &str) -> Vec<String> {
        let token_pattern = r#""[^"]*"|'[^']*'|==|!=|<=|>=|,|[+\-*/=()<>]|\band\b|\bor\b|\bnot\b|[a-zA-Z_]\w*|\d+(\.\d+)?"#;

        let re = Regex::new(token_pattern).unwrap();
        let tokens: Vec<String> = re.find_iter(input)
            .map(|mat| mat.as_str().to_string())
            .collect();
        return tokens;
    }
        fn shunting_yard(&self, tokens: Vec<String>, variables: HashMap<String, Value>, functions: HashMap<String, HashMap<String, Value>>) -> Vec<Value> {
        let prec = HashMap::from([
            ("or", 1), ("and", 2),
            ("==", 3), ("!=", 3), ("<", 3), ("<=", 3), (">", 3), (">=", 3),
            ("+", 4), ("-", 4),
            ("*", 5), ("/", 5)
        ]);
        let mut output: Vec<Value> = vec![];
        let mut stack: Vec<Value> = vec![];

        let mut i = 0;
        while i < tokens.len(){
            let token = &tokens[i];
            if token.starts_with('"') && token.ends_with('"') || token.starts_with("'") && token.ends_with("'"){
                output.push(Value::Str(token.clone()));
            }
            else if token.trim_matches('.').parse::<f64>().is_ok() || variables.contains_key(token){
                output.push(Value::Str(token.clone()));
            }
            else if functions.contains_key(token) || BUILT_IN_FUNCTIONS.contains_key(&token as &str){
                if tokens.get(i+1) != Some(&"(".to_string()){
                    EvaluatioError::new(format!("Function {} must be followed by (", token), None, None).raise();
                }
                output.push(Value::Str(token.clone()));
                i += 1; // Skip the next token which is '('
                let mut function_arguments = vec![];
                let mut argument: String = String::new();
                while let Some(next_token) = tokens.get(i+1) {
                    if next_token == ")" {
                        i += 1;
                        break;
                    }
                    else if next_token == "," {
                        if argument.is_empty() {
                            i += 1;
                            continue;
                        }
                        println!("Pushing argument: {}", argument);
                        function_arguments.push(Value::Str(argument.clone()));
                        i += 2;
                    }
                    else {
                        argument += next_token;
                        i += 1;
                    }
                }
                function_arguments.push(Value::Str(argument.clone()));
                output.push(Value::List(function_arguments));
            }
            else if prec.contains_key(token.as_str()) {
                while let Some(last) = stack.last() {
                    if prec.contains_key(last.to_string_value().as_str()) && prec[last.to_string_value().as_str()] >= prec[token.as_str()]{
                        output.push(stack.pop().unwrap());
                    }
                    else{
                        break;
                    }
                }
                stack.push(Value::Str(token.clone()));
            }
            else if token == "("{
                stack.push(Value::Str(token.clone()));
            }
            else if token == "," {
                // Pop operators until we reach a left parenthesis
                while let Some(last) = stack.last() {
                    if last.to_string_value() != "(" {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                // Add the comma to the output
                output.push(Value::Str(token.clone()));
            }
            else if token == ")"{
                while let Some(last) = stack.last(){
                    if last.to_string_value() != "("{
                        output.push(stack.pop().unwrap());
                    }
                    else{
                        break;
                    }
                } 
                if stack.is_empty(){
                    EvaluatioError::new("Mismatched parentheses".to_string(), None, None).raise();
                }
                stack.pop(); // remove "("
            }
            else{
                EvaluatioError::new(format!("Unknown token {}", token), None, None).raise();
            }
            i += 1;
        }

        // println!("Stack after processing tokens: {:?}", stack);
        // println!("Output after processing tokens: {:?}", output);
        while let Some(last) = stack.last(){
            if last.to_string_value() == "(" || last.to_string_value() == ")"{
                EvaluatioError::new("Mismatched parentheses".to_string(), None, None).raise();
            }
            output.push(stack.pop().unwrap());
        }

        return output;
    }
}

