use std::env;
mod error;
mod evaluator;
mod tokenizer;
mod built_in_functions;

use error::EvaluatioError;
use evaluator::Evaluator;

fn main(){
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        EvaluatioError::new("Please provide a file to evaluate".to_string(), None, None).raise();
        return;
    }
    let file = &args[1];
    if !file.ends_with(".ipl"){
        EvaluatioError::new("File must be a IPL file".to_string(), None, None).raise();
        return;
    }
    let mut evaluator = Evaluator::new();
    evaluator.ev_file(file);
    let tokenizer = tokenizer::Tokenizer::new();
    let tokens = tokenizer.tokenize("h = variable + 3 * 3", evaluator.variables, evaluator.functions);
    println!("Tokens: {:?}", tokens);
}
