use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_path: &str) -> Map<String, Value> {
    let mut file: File = File::open(file_path.to_string()).unwrap();

    let mut data: String = String::new();
    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();

    // Se não o clonar, estara apenas retornando uma referência.
    let state: Map<String, Value> = json.as_object().unwrap().clone();

    state
}

pub fn write_to_file(file_path: &str, state: &mut Map<String, Value>) {
    let new_data: Value = json!(state);
    fs::write(file_path, new_data.to_string()).expect("Unable to write file");
}
