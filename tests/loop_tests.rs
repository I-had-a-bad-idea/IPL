mod common;
use common::run_ipl_file;

#[test]
fn test_while_loop_with_continue() {
    let output = run_ipl_file("tests/ipl_files/loops.ipl");
    assert!(output.contains("7"));
}