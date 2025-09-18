use std::collections::HashMap;
mod error;
use error::EvaluatioError;

let built_in_functions = HashMap::from([
    ("out", vec!["output"]),
    ("value", vec!["number"]),
    ("in", vec!["message"]),
    ("random", vec!["start", "end"]),
    ("min", vec![]),
    ("max", vec![]),
    ("round", vec!["number"]),
    ("pow", vec!["base", "exp"]),
]);

fn call_built_in_function(name: &str, args: Vec<&str>) -> None {
    match name {
        "out" => {
            if let Some(output) = args.get(0) {
                println!("{}", output);
            } else {
                EvaluatioError::new("Error: 'out' function requires 1 argument".to_string(), None, None).raise();
            }
        }
        "value" => {
            if let Some(number) = args.get(0) {
                if let Ok(num) = number.parse::<f64>() {
                    return Some(num);
                } else {
                    EvaluatioError::new("Error: 'value' function requires a numeric argument".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'value' function requires 1 argument".to_string(), None, None).raise();
            }
        }
        "in" => {
            use std::io::{self, Write};
            if let Some(message) = args.get(0) {
                print!("{}", message);
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                return Some(input.trim().to_string());
            } else {
                EvaluatioError::new("Error: 'in' function requires 1 argument".to_string(), None, None).raise();
            }
        }
        "random" => {
            use rand::Rng;
            if args.len() == 2 {
                if let (Ok(start), Ok(end)) = (args[0].parse::<i32>(), args[1].parse::<i32>()) {
                    let mut rng = rand::thread_rng();
                    return Some(rng.gen_range(start..=end) as f64);
                } else {
                    EvaluatioError::new("Error: 'random' function requires two numeric arguments".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'random' function requires 2 arguments".to_string(), None, None).raise();
            }
        }
        "min" => {
            // Placeholder for min implementation
            println!("'min' function called");
        }
        "max" => {
            // Placeholder for max implementation
            println!("'max' function called");
        }
        "round" => {
            if let Some(number) = args.get(0) {
                if let Ok(num) = number.parse::<f64>() {
                    return Some(num.round());
                } else {
                    EvaluatioError::new("Error: 'round' function requires a numeric argument".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'round' function requires 1 argument".to_string(), None, None).raise();
            }
        }
        "pow" => {
            if args.len() == 2 {
                if let (Ok(base), Ok(exp)) = (args[0].parse::<f64>(), args[1].parse::<f64>()) {
                    return Some(base.powf(exp));
                } else {
                    EvaluatioError::new("Error: 'pow' function requires two numeric arguments".to_string(), None, None).raise();
                }
            } else {
                EvaluatioError::new("Error: 'pow' function requires 2 arguments".to_string(), None, None).raise();
            }
        }

