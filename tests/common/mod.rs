use std::process::Command;

pub fn run_ipl_file(file: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", file])
        .output()
        .expect("failed to run IPL file");

    println!("Output: {:#?}", output);
    String::from_utf8_lossy(&output.stdout).to_string()
}


pub fn assert_lines(output: &str, expected: Vec<&str>) {
    let lines : Vec<&str> = output.lines().collect();
    assert_eq!(lines, expected);
}