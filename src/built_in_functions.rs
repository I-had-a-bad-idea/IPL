use std::collections::HashMap;
use crate::debug::EvaluatioError;
use crate::evaluator::Value;

// Define built-in functions and their argument names/ amounts
pub static BUILT_IN_FUNCTIONS: std::sync::LazyLock<HashMap<&str, Vec<&str>>> = std::sync::LazyLock::new(|| HashMap::from([
    ("out", vec!["output"]),
    ("value", vec!["number"]),
    ("in", vec!["message"]),
    ("random", vec!["start", "end"]),
    ("min", vec![]),
    ("max", vec![]),
    ("round", vec!["number"]),
    ("pow", vec!["base", "exp"]),
]));

// Call a built-in function by name with given arguments
pub fn call_built_in_function(name: &str, args: Vec<Value>) -> Value {
    //println!("Called built in function {} with arguments {:?}", name, args);
    if args.len() != BUILT_IN_FUNCTIONS[name].len(){
        EvaluatioError::new(format!("Error: Function '{}' expects {} arguments, but got {}", name, BUILT_IN_FUNCTIONS[name].len(), args.len()), None, None).raise();
    }
    match name {
        "out" => {
            if let Some(output) = args.get(0) {
                println!("{}", output.to_string_value());
            } else {
                EvaluatioError::new("Error: 'out' function requires 1 argument".to_string(), None, None).raise();
            }
            return Value::None;
        }
        "value" => {
            if let Some(number) = args.get(0) {
                if let Value::Number(num) = number {
                    return Value::Number(*num);
                } else {
                    EvaluatioError::new("Error: 'value' function requires a numeric argument".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'value' function requires 1 argument".to_string(), None, None).raise();
            }
            return Value::None;
        }
        "in" => {
            use std::io::{self, Write};
            if let Some(message) = args.get(0) {
                println!("{}", message.to_string_value());
                io::stdout().flush().unwrap();
                let mut input: String = String::new();
                io::stdin().read_line(&mut input).unwrap();
                return Value::Str(input.trim().to_string());
            } else {
                EvaluatioError::new("Error: 'in' function requires 1 argument".to_string(), None, None).raise();
            }
            return Value::None;
        }
        "random" => {
            use rand::Rng;
            if args.len() == 2 {
                if let (Value::Number(start), Value::Number(end)) = (&args[0], &args[1]) {
                    let mut rng: rand::prelude::ThreadRng = rand::rng();
                    return Value::Number(rng.random_range(*start as i32..=*end as i32) as f64);
                } else {
                    EvaluatioError::new("Error: 'random' function requires two numeric arguments".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'random' function requires 2 arguments".to_string(), None, None).raise();
            }
            return Value::None;
        }
        "min" => {
            match args.get(0) {
                Some(Value::List(list)) if !list.is_empty() => {
                    let min_value = list.iter().filter_map(|v| {
                        if let Value::Number(num) = v {
                            Some(*num)
                        } else {
                            None
                        }
                    }).min_by(|a, b| a.partial_cmp(b).unwrap());
                    if let Some(min) = min_value {
                        return Value::Number(min);
                    } else {
                        EvaluatioError::new("Error: 'min' function requires a list of numeric values".to_string(), None, None).raise();
                    }
                }
                _ => {
                    EvaluatioError::new("Error: 'min' function requires 1 argument which is a non-empty list".to_string(), None, None).raise();
                }
            }
            return Value::None;
        }
        "max" =>{
            match args.get(0) {
                Some(Value::List(list)) if !list.is_empty() => {
                    let max_value = list.iter().filter_map(|v| {
                        if let Value::Number(num) = v {
                            Some(*num)
                        } else {
                            None
                        }
                    }).max_by(|a, b| a.partial_cmp(b).unwrap());
                    if let Some(max) = max_value {
                        return Value::Number(max);
                    } else {
                        EvaluatioError::new("Error: 'max' function requires a list of numeric values".to_string(), None, None).raise();
                    }
                }
                _ => {
                    EvaluatioError::new("Error: 'max' function requires 1 argument which is a non-empty list".to_string(), None, None).raise();
                }
            }
            return Value::None;
        }
        "round" => {
            if let Some(number) = args.get(0) {
                if let Value::Number(num) = number {
                    return Value::Number(num.round());
                } else {
                    EvaluatioError::new("Error: 'round' function requires a numeric argument".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'round' function requires 1 argument".to_string(), None, None).raise();
            }
            return Value::None;
        }
        "pow" => {
            if args.len() == 2 {
                if let (Value::Number(base), Value::Number(exp)) = (&args[0], &args[1]) {
                    return Value::Number(base.powf(*exp));
                } else {
                    EvaluatioError::new("Error: 'pow' function requires two numeric arguments".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'pow' function requires 2 arguments".to_string(), None, None).raise();
            }
            return Value::None;
        }
        _ => {
            EvaluatioError::new(format!("Error: Unknown built-in function '{}'", name), None, None).raise();
            return Value::None;
        }
    }
}

