use regex::Regex;
use std::collections::HashMap;

use crate::{error::EvaluatioError, evaluator::Value, built_in_functions::BUILT_IN_FUNCTIONS};

pub struct Tokenizer{
}

impl Tokenizer{
    pub fn new() -> Self{
        Self {
        }
    }
    pub fn tokenize(&self, input: &str, variables: HashMap<String, Value>, functions: HashMap<String, HashMap<String, Value>>) -> Vec<String> {
        // Placeholder for tokenization logic
        let tokens = self.split(input);
        println!("tokens after splitting: {:?}", tokens);
        let output = self.shunting_yard(tokens, variables, functions);
        return output;
    }

    fn split(&self, input: &str) -> Vec<String> {
        let token_pattern = r#""[^"]*"|'[^']*'|==|!=|<=|>=|[+\-*/=()<>]|\band\b|\bor\b|\bnot\b|[a-zA-Z_]\w*|\d+(\.\d+)?"#;

        let re = Regex::new(token_pattern).unwrap();
        let tokens: Vec<String> = re.find_iter(input)
            .map(|mat| mat.as_str().to_string())
            .collect();
        return tokens;
    }
    fn shunting_yard(&self, tokens: Vec<String>, variables: HashMap<String, Value>, functions: HashMap<String, HashMap<String, Value>>) -> Vec<String> {
        let prec = HashMap::from([
            ("or", 1), ("and", 2),
            ("==", 3), ("!=", 3), ("<", 3), ("<=", 3), (">", 3), (">=", 3),
            ("+", 4), ("-", 4),
            ("*", 5), ("/", 5)
        ]);
        let mut output: Vec<String> = vec![];
        let mut stack: Vec<String> = vec![];

        for token in &tokens{
            println!("At token {}, output: {:?}", token, output);
            if token.starts_with('"') && token.ends_with('"') || token.starts_with("'") && token.ends_with("'"){
                output.push(token.clone());
            }
            else if token.trim_matches('.').parse::<f64>().is_ok() || variables.contains_key(token) || functions.contains_key(token) || BUILT_IN_FUNCTIONS.contains_key(&token as &str){
                output.push(token.clone());
            }
            else if prec.contains_key(token.as_str()) {
                while let Some(last) = stack.last() {
                    if prec.contains_key(last.as_str()) && prec[last.as_str()] >= prec[token.as_str()]{
                        output.push(stack.pop().unwrap());
                    }
                    else{
                        break;
                    }
                }
                stack.push(token.clone()); //push the operator itself
            }
            else if token == "("{
                stack.push(token.clone());
                
            }
            else if token == ")"{
                while let Some(last) = stack.last(){
                    if last != "("{
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
        }
        println!("Remaining stack: {:?}", stack);
        while let Some(last) = stack.last(){
            // println!("Last: {}", last);
            // println!("Remaining stack: {:?}", stack);
            if last == "(" || last == ")"{
                EvaluatioError::new("Mismatched parentheses".to_string(), None, None).raise();
            }
            output.push(stack.pop().unwrap());
        }

        return output;
    }
}

