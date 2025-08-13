use std::collections::HashMap;
use std::path::Path;

use crate::formats::ini;

pub fn read_lines(filename: &str) {
    let result: Option<HashMap<String, HashMap<String, String>>>;
    match parse_file_type(filename).as_str() {
        "ini" => {
            result = ini::parse_file(filename);
            println!("{:?}", result);
        },
        "json" => {},
        "toml" => {},
        _ => {}
    }
}

fn parse_file_type(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| panic!("Unable to parse file type '{}'", filename))
}