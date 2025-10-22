use std::sync::RwLock;

/// Holds the current line number and content
pub struct LineData {
    pub line_number: usize, 
    pub line_content: String,
}

// Global, thread-safe state
static PROGRAMM_STATE: RwLock<LineData> = RwLock::new(LineData {
    line_number: 0,
    line_content: String::new(),
});

/// Public getter for line number
pub fn get_line_number() -> usize {
    let state = PROGRAMM_STATE.read().unwrap();
    state.line_number
}

/// Public getter for line content
pub fn get_line_content() -> String {
    let state = PROGRAMM_STATE.read().unwrap();
    state.line_content.clone()
}

/// Crate-private setter: only accessible inside this crate (e.g., from `evaluator.rs`)
pub fn set_programm_state(line_number: usize, line_content: &str) {
    let mut state = PROGRAMM_STATE.write().unwrap();
    state.line_number = line_number;
    state.line_content = line_content.to_string();
}
