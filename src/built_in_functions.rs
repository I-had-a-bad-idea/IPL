use std::collections::HashMap;
use crate::debug::EvaluatioError;
use crate::evaluator::Value;

pub static BUILT_IN_FUNCTIONS: std::sync::LazyLock<HashMap<&str, Vec<&str>>> = std::sync::LazyLock::new(|| HashMap::from([
    ("out", vec!["output"]),
    ("value", vec!["number"]),
    ("in", vec!["message"]),
    ("random", vec!["start", "end"]),
    ("min", vec![]),
    ("max", vec![]),
    ("round", vec!["number"]),
    ("pow", vec!["base", "exp"]),
    ("test", vec![]),
]));

pub fn call_built_in_function(name: &str, args: Vec<Value>) -> Value {
    println!("Called built in function {} with arguments {:?}", name, args);
    match name {
        "test" => {
            return Value::Number(24.0);
        }
        "out" => {
            if let Some(output) = args.get(0) {
                println!("{:?}", output);
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
                print!("{:?}", message);
                io::stdout().flush().unwrap();
                let mut input = String::new();
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
                    let mut rng = rand::rng();
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
            // Placeholder for min implementation
            println!("'min' function called");
            return Value::None;
        }
        "max" => {
            // Placeholder for max implementation
            println!("'max' function called");
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

