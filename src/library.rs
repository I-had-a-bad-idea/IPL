use std::path::PathBuf;
use std::path::Path;
use std::fs;

use crate::debug::EvaluatioError;

const ILI_PATH : &str = "C:\\ProgramData\\ILI";

#[derive(Debug)]
struct Library {
    name: String,
    version: String,
    entry: String,
    dependencies: Vec<String>,
}

fn load_library_json(path: &Path) -> Option<Library> {
    let file = path.join("Library.json");
    let raw = fs::read_to_string(&file).ok()?;

    let mut name = String::new();
    let mut version = String::new();
    let mut entry = String::new();
    let mut dependencies = Vec::new();

    for line in raw.lines() {
        let l = line.trim();

        if l.starts_with("\"name\"") {
            name = extract_string(l)?;
        } else if l.starts_with("\"version\"") {
            version = extract_string(l)?;
        } else if l.starts_with("\"entry\"") {
            entry = extract_string(l)?;
        } else if l.starts_with("\"dependencies\"") {
            dependencies = extract_array(l, &raw)?;
        }
    }

    Some(Library { name, version, entry, dependencies })
}

fn extract_string(line: &str) -> Option<String> {
    let start = line.find('"')?; // Where it starts
    let rest = &line[start+1..];
    let mid = rest.find('"')?; // End of key
    let rest = &rest[mid+1..];
    let value_start = rest.find('"')?; // Start of value
    let rest = &rest[value_start+1..];
    let value_end = rest.find('"')?; // End of value
    Some(rest[..value_end].to_string()) // Extracted value
}

fn extract_array(_line: &str, full: &str) -> Option<Vec<String>> {
    let start = full.find('[')?; // Start of array
    let end = full.find(']')?; // End of array
    let inside = &full[start+1..end]; // Inside the brackets
    let mut out = Vec::new();
    for part in inside.split(',') { // Split by commas
        let t = part.trim();
        if t.starts_with('"') && t.ends_with('"') { // Is a string
            out.push(t[1..t.len()-1].to_string()); // Remove quotes
        }
    }
    Some(out)
}

fn get_library_entry_path(libary_name: &str) -> PathBuf {
    let libs_dir: PathBuf = PathBuf::from(ILI_PATH).join("libs");
    let path: PathBuf = libs_dir.join(libary_name);
    if !path.exists(){
        EvaluatioError::new("Library doesnt exist".to_string()).raise();
    }

    let path_to_config: PathBuf = path.join("Library.json");
    if !path_to_config.exists(){
        EvaluatioError::new("Library is missing Library.json file".to_string()).raise();
    }

    let lib_json: Option<Library> = load_library_json(&path_to_config);

    if !lib_json.is_some(){
        EvaluatioError::new("Failed to load Library.json file".to_string()).raise();
    }

    let lib: Library = lib_json.unwrap();
    let entry_path: PathBuf = PathBuf::from(path).join(lib.entry);
    if !entry_path.exists(){
        EvaluatioError::new("Library entry file doesnt exist".to_string()).raise();
    }

    return entry_path;

}