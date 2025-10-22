use regex::Regex;
use std::collections::HashMap;

use crate::{
    built_in_functions::BUILT_IN_FUNCTIONS,
    debug::EvaluatioError,
    evaluator::{Class, Value},
};

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Self {
        Self {}
    }
    // Tokenize an input string into a vector of Values (tokens), using regex and a version of the shunting yard algorithm
    pub fn tokenize(
        &self,
        input: &str,
        variables: HashMap<String, Value>,
        functions: HashMap<String, HashMap<String, Value>>,
        classes: HashMap<String, Class>,
    ) -> Vec<Value> {
        let tokens = self.split(input);
        // println!("tokens after splitting: {:?}", tokens);

        self.shunting_yard(tokens, variables, functions, classes)
    }
    // Split the input string into tokens using regex
    fn split(&self, input: &str) -> Vec<String> {
        let token_pattern = r#""[^"]*"|'[^']*'|==|!=|<=|>=|[+\-*/=()<>\[\],]|\.|\band\b|\bor\b|\bnot\b|[a-zA-Z_]\w*|\d+\.\d+|\d+"#;

        let re = Regex::new(token_pattern).unwrap();
        let tokens: Vec<String> = re
            .find_iter(input)
            .map(|mat| mat.as_str().to_string())
            .collect();
        tokens
    }

    fn str_to_datatype(&self, token: &str) -> Value {
        if token.starts_with('"') && token.ends_with('"')
            || token.starts_with("'") && token.ends_with("'")
        {
            Value::Str(token.to_string().clone())
        } else if token.trim_matches('.').parse::<f64>().is_ok() {
            Value::Number(Value::Str(token.to_string().clone()).as_f64())
        } else {
            Value::None
        }
    }

    // Convert infix tokens to postfix using the shunting yard algorithm
    fn shunting_yard(
        &self,
        tokens: Vec<String>,
        variables: HashMap<String, Value>,
        functions: HashMap<String, HashMap<String, Value>>,
        classes: HashMap<String, Class>,
    ) -> Vec<Value> {
        let prec = HashMap::from([
            ("or", 1),
            ("and", 2),
            ("==", 3),
            ("!=", 3),
            ("<", 3),
            ("<=", 3),
            (">", 3),
            (">=", 3),
            ("+", 4),
            ("-", 4),
            ("*", 5),
            ("/", 5),
            (".", 6),
        ]);
        let mut output: Vec<Value> = vec![];
        let mut stack: Vec<Value> = vec![];

        let mut i = 0;
        while i < tokens.len() {
            let token: &String = &tokens[i];
            let token_as_datatype = self.str_to_datatype(token);
            if !token_as_datatype.is_none_value() {
                output.push(token_as_datatype);
            } else if i + 1 < tokens.len()
                && ((variables.contains_key(token) || classes.contains_key(token))
                    && &tokens[i + 1] == ".")
            {
                output.push(Value::Str(token.clone()));

                if &tokens[i + 1] == "." {
                    i += 2; // Skip the '.' token
                    if let Some(attr) = tokens.get(i) {
                        output.push(Value::Str(".".to_string()));
                        if !classes
                            .values()
                            .any(|class| class.functions.contains_key(attr))
                        {
                            output.push(Value::Str(attr.clone()));
                        } else {
                            i -= 1; // Step back to reprocess this token as a function
                        }
                    } else {
                        EvaluatioError::new("Expected attribute after '.'".to_string()).raise();
                    }
                }
            } else if variables.contains_key(token) {
                output.push(Value::Str(token.clone()));
            } else if functions.contains_key(token)
                || BUILT_IN_FUNCTIONS.contains_key(token as &str)
                || classes.contains_key(token)
                || classes
                    .values()
                    .any(|class| class.functions.contains_key(token))
            {
                if tokens.get(i + 1) != Some(&"(".to_string()) {
                    EvaluatioError::new(format!("Function {} must be followed by (", token))
                        .raise();
                }
                output.push(Value::Str(token.clone()));
                i += 1; // Skip the next token which is '('
                let mut function_arguments = vec![];
                let mut argument: String = String::new();
                while let Some(next_token) = tokens.get(i + 1) {
                    if next_token == ")" {
                        i += 1;
                        break;
                    } else if next_token == "," {
                        if argument.is_empty() {
                            i += 1;
                            continue;
                        }
                        // println!("Pushing argument: {}", argument);
                        function_arguments.push(Value::Str(argument.clone()));
                        argument.clear();
                        i += 1;
                    } else {
                        argument += next_token;
                        i += 1;
                    }
                }
                function_arguments.push(Value::Str(argument.clone()));
                output.push(Value::List(function_arguments));
            } else if prec.contains_key(token.as_str()) {
                while let Some(last) = stack.last() {
                    if prec.contains_key(last.to_string_value().as_str())
                        && prec[last.to_string_value().as_str()] >= prec[token.as_str()]
                    {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(Value::Str(token.clone()));
            } else if token == "[" {
                let mut list_elements = vec![];
                let mut element: String = "".to_string();
                while let Some(next_token) = tokens.get(i + 1) {
                    if next_token == "]" {
                        i += 1;
                        break;
                    } else if next_token == "," {
                        if element.is_empty() {
                            i += 1;
                            continue;
                        }
                        // println!("Pushing element: {}", element);
                        list_elements.push(self.str_to_datatype(&element));
                        element.clear();
                        i += 1;
                    } else {
                        element += next_token;
                        i += 1;
                    }
                }
                list_elements.push(Value::Str(element.clone()));
                output.push(Value::List(list_elements));
            } else if token == "(" {
                stack.push(Value::Str(token.clone()));
            } else if token == "," {
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
            } else if token == ")" {
                while let Some(last) = stack.last() {
                    if last.to_string_value() != "(" {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                if stack.is_empty() {
                    EvaluatioError::new("Mismatched parentheses".to_string()).raise();
                }
                stack.pop(); // remove "("
            } else {
                EvaluatioError::new(format!("Unknown token {}", token)).raise();
            }
            i += 1;
        }

        //println!("Stack after processing tokens: {:?}", stack);
        //println!("Output after processing tokens: {:?}", output);
        while let Some(last) = stack.last() {
            if last.to_string_value() == "(" || last.to_string_value() == ")" {
                EvaluatioError::new("Mismatched parentheses".to_string()).raise();
            }
            output.push(stack.pop().unwrap());
        }

        output
    }
}
