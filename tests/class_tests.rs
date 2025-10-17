mod common;
use common::run_ipl_file;

#[test]
fn test_class_inheritance() {
    let output = run_ipl_file("tests/ipl_files/classes.ipl");
    assert!(output.contains("Woof!"));
}
