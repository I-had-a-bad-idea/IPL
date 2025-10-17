mod common;
use common::run_ipl_file;

#[test]
fn test_function_return() {
    let output = run_ipl_file("tests/ipl_files/functions.ipl");
    assert!(output.contains("10"));
}