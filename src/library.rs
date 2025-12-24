use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::debug::EvaluatioError;

fn get_ili_path() -> PathBuf {
    match std::env::var("ILI_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => get_hardcoded_ili_path(),
    }
}

fn get_hardcoded_ili_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from(std::env::var("PROGRAMDATA").unwrap()).join("ILI")
    }

    #[cfg(target_os = "linux")]
    {
        PathBuf::from("/usr/local/share/ILI")
    }

    #[cfg(target_os = "macos")]
    {
        PathBuf::from("/Library/Application Support/ILI")
    }
}

fn load_library_json(path: &Path) -> String {
    let raw = fs::read_to_string(&path).ok();

    if raw.is_none() {
        EvaluatioError::new("Failed to read Library.json file".to_string()).raise();
    }

    for line in raw.unwrap().lines() {
        let l = line.trim();

        if l.starts_with("\"entry\"") {
            let entry = extract_string(l);
            if entry.is_some() {
                return entry.unwrap();
            }
        }
    }
    "".to_string()
}

fn extract_string(line: &str) -> Option<String> {
    let start = line.find('"')?; // Where it starts
    let rest = &line[start + 1..];
    let mid = rest.find('"')?; // End of key
    let rest = &rest[mid + 1..];
    let value_start = rest.find('"')?; // Start of value
    let rest = &rest[value_start + 1..];
    let value_end = rest.find('"')?; // End of value
    Some(rest[..value_end].to_string()) // Extracted value
}

pub fn get_library_entry_path(libary_name: &str) -> PathBuf {
    let libs_dir: PathBuf = PathBuf::from(get_ili_path()).join("libs");
    let path: PathBuf = libs_dir.join(libary_name);

    if !path.exists() {
        EvaluatioError::new("Library doesnt exist".to_string()).raise();
    }

    let path_to_config: PathBuf = path.join("Library.json");
    if !path_to_config.exists() {
        EvaluatioError::new("Library is missing Library.json file".to_string()).raise();
    }

    let lib_entry: String = load_library_json(&path_to_config);

    let entry_path: PathBuf = PathBuf::from(path).join(lib_entry);
    if !entry_path.exists() {
        EvaluatioError::new("Library entry file doesnt exist".to_string()).raise();
    }

    return entry_path;
}
