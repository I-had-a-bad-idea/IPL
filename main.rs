use std::env;
mod error;
mod evaluator;

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
    let evaluator = Evaluator::new();
    evaluator.ev_file(file)
}
