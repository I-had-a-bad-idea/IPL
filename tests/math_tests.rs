mod common;
use common::run_ipl_file;

#[test]
fn test_math_output() {
    let output = run_ipl_file("tests/ipl_files/math.ipl");
    // Check for expected math operation results
    assert!(output.contains("15"), "Expected output to contain 15");
    assert!(output.contains("5"), "Expected output to contain 5");
    assert!(output.contains("50"), "Expected output to contain 50");
    assert!(output.contains("10"), "Expected output to contain 10");
}