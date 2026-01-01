mod common;
use std::vec;

use common::{run_ipl_file, assert_lines};

#[test]
fn test_while_and_for_loop_with_continue() {
    let output = run_ipl_file("tests/ipl_files/loops.ipl");
    let expected = vec!["7", "3", "1", "3", "5", "1", "2"];
    assert_lines(&output, expected);
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
    let expected = vec!["15", "5", "50", "10", "3.14159265358979"]; // Cant do more, because they are cut of
    assert_lines(&output, expected);
}

#[test]
fn test_variable_assignment() {
    let output = run_ipl_file("tests/ipl_files/variables.ipl");
    assert!(output.contains("15"));
}

#[test]
fn test_if_elif_else() {
    let output = run_ipl_file("tests/ipl_files/if_elif_else.ipl");
    let expected = vec!["5", "20", "10"];
    assert_lines(&output, expected);
}

#[test]
fn test_import() {
    let output = run_ipl_file("tests/ipl_files/import.ipl");
    assert!(output.contains("Woof"));
    assert!(output.contains("Some sound"));
}
#[test]
fn test_libraries() {
    let output = run_ipl_file("tests/ipl_files/libraries.ipl");
    let expected = vec!["3", "13", "16", "10", "Hello World"];
    assert_lines(&output, expected);
}
#[test]
fn test_indexing() {
    let output = run_ipl_file("tests/ipl_files/indexing.ipl");
    assert!(output.contains("1"));
    assert!(output.contains("[2, 3, 4, 5]"));
}
