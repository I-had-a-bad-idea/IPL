mod common;
use common::run_ipl_file;

#[test]
fn test_variable_assignment() {
    let output = run_ipl_file("tests/ipl_files/variables.ipl");
    assert!(output.contains("15"));
}