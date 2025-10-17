use std::process::Command;

pub fn run_ipl_file(file: &str) -> String {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", file])
        .output()
        .expect("failed to run IPL file");

    String::from_utf8_lossy(&output.stdout).to_string()
}
