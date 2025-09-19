use regex::Regex;
use std::collections::HashMap;

use crate::{error::EvaluatioError, evaluator::{Value}};

pub struct Tokenizer{
}

impl Tokenizer{
    pub fn new() -> Self{
        Self {
        }
    }
    pub fn tokenize(&self, input: &str, variables: HashMap<String, String>, functions: HashMap<String, HashMap<String, Value>>) -> Vec<String> {
        // Placeholder for tokenization logic
        let tokens = self.split(input);
        let output = self.shunting_yard(tokens, variables, functions);
        return output;
    }

    fn split(&self, input: &str) -> Vec<String> {
        let token_pattern = r#""[^"]*"|'[^']*'|\S+"#;
        let re = Regex::new(token_pattern).unwrap();
        let tokens: Vec<String> = re.find_iter(input)
            .map(|mat| mat.as_str().to_string())
            .collect();
        return tokens;
    }
    fn shunting_yard(&self, tokens: Vec<String>, variables: HashMap<String, String>, functions: HashMap<String, HashMap<String, Value>>) -> Vec<String> {
        let prec = HashMap::from([
            ("or", 1), ("and", 2),
            ("==", 3), ("!=", 3), ("<", 3), ("<=", 3), (">", 3), (">=", 3),
            ("+", 4), ("-", 4),
            ("*", 5), ("/", 5)
        ]);
        let mut output: Vec<String> = vec![];
        let mut stack: Vec<String> = vec![];

        for token in &tokens{
            if token.starts_with('"') && token.ends_with('"') || token.starts_with("'") && token.ends_with("'"){
                output.push(token.clone());
            }
            else if token.trim_matches('.').parse::<f64>().is_ok() || variables.contains_key(token) || functions.contains_key(token){
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
        while let Some(last) = stack.last(){
            if last == "(" || last == ")"{
                EvaluatioError::new("Mismatched parentheses".to_string(), None, None).raise();
            output.push(stack.pop().unwrap());
            }
        }

        // Placeholder for shunting yard algorithm
        return tokens;
    }
}


    // def shunting_yard(self, tokens):
    //     prec = {
    //         "or": 1, "and": 2,
    //         "==": 3, "!=": 3, "<": 3, "<=": 3, ">": 3, ">=": 3,
    //         "+": 4, "-": 4,
    //         "*": 5, "/": 5
    //     }
    //     output = []
    //     stack = []

    //     for token in tokens:
    //         if (token.startswith('"') and token.endswith('"')) or (token.startswith("'") and token.endswith("'")): #If it is a string
    //             output.append(token)
    //         elif token.strip(".").isdigit() or token in self.variables or token.split("(")[0] in self.functions or token.split("(")[0] in built_in_functions:
    //             output.append(token)
    //         elif token in prec:
    //             while stack and stack[-1] in prec and prec[stack[-1]] >= prec[token]:
    //                 output.append(stack.pop())
    //             stack.append(token)
    //         elif token == "(":
    //             stack.append(token)
    //         elif token == ")":
    //             while stack and stack[-1] != "(":
    //                 output.append(stack.pop())
    //             if not stack:
    //                 raise EvaluationError("Mismatched parentheses")
    //             stack.pop()  # remove "("
    //         else:
    //             raise EvaluationError(f"Unknown token {token}")

    //     while stack:
    //         if stack[-1] in ("(", ")"):
    //             raise EvaluationError("Mismatched parentheses")
    //         output.append(stack.pop())

    //     return output   