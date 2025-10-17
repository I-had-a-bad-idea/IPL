mod common;
use common::run_ipl_file;

#[test]
fn test_sum_output() {
    let output = run_ipl_file("tests/ipl_files/sum.ipl");
    assert!(output.contains("15"));
}