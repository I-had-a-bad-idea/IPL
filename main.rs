use std::env;
mod error;

use error::EvaluatioError;

fn main(){
    let error = EvaluatioError::new("Test error".to_string(), None, None).raise();
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Please provide a file to evaluate");
        return;
    }
    let file = args[1];
    if !file.ends_with(".ipl"){
        println!("File must be a IPL file");
        return;
    }
    //let evaluator = Evaluator::new();
    //evaluator.ev_file(file)
}
