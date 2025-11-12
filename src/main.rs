use std::env;
mod built_in_functions;
mod debug;
mod evaluator;
mod state;
mod tokenizer;
pub mod value;

use debug::EvaluatioError;
use evaluator::Evaluator;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>(); // Collect command line arguments
    if args.len() < 2 {
        EvaluatioError::new("Please provide a file to evaluate".to_string()).raise();
        return;
    }
    let file = &args[1]; // Get the file name from arguments
    if !file.ends_with(".ipl") {
        EvaluatioError::new("File must be a IPL file".to_string()).raise();
        return;
    }
    let mut evaluator: Evaluator = Evaluator::new(); // Create a new evaluator for the file
    evaluator.ev_file(file); // Evaluate the file
}
