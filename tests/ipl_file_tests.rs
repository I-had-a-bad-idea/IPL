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
    assert!(output.contains("3.14159265358979")); // Cant do more, because they are cut of
}

#[test]
fn test_variable_assignment() {
    let output = run_ipl_file("tests/ipl_files/variables.ipl");
    assert!(output.contains("15"));
}

#[test]
fn test_if_elif_else() {
    let output = run_ipl_file("tests/ipl_files/if_elif_else.ipl");
    assert!(output.contains("5"));
    assert!(output.contains("20"));
    assert!(output.contains("10"));
}

#[test]
fn test_import() {
    let output = run_ipl_file("tests/ipl_files/import.ipl");
    assert!(output.contains("Woof"));
    assert!(output.contains("Some sound"));
}
// #[test]
// fn test_libraries() {
//     let output = run_ipl_file("tests/ipl_files/libraries.ipl");
//     assert!(output.contains("3"));
//     assert!(output.contains("13"));
//     assert!(output.contains("16"));
//     assert!(output.contains("10"));
//     assert!(output.contains("Hello World"));
// }