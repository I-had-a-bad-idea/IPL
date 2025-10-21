mod common;
use common::run_ipl_file;

#[test]
fn test_while_loop_with_continue() {
    let output = run_ipl_file("tests/ipl_files/loops.ipl");
    assert!(output.contains("7"));
}

#[test]
fn test_function_return() {
    let output = run_ipl_file("tests/ipl_files/functions.ipl");
    assert!(output.contains("10"));
}

#[test]
fn test_class_inheritance() {
    let output = run_ipl_file("tests/ipl_files/classes.ipl");
    assert!(output.contains("Woof!"));
}

#[test]
fn test_math_output() {
    let output = run_ipl_file("tests/ipl_files/math.ipl");
    // Check for expected math operation results
    assert!(output.contains("15"));
    assert!(output.contains("5"));
    assert!(output.contains("50"));
    assert!(output.contains("10"));
}

#[test]
fn test_variable_assignment() {
    let output = run_ipl_file("tests/ipl_files/variables.ipl");
    assert!(output.contains("15"));
}
