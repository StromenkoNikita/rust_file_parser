use std::fs;
use std::path::Path;

use crate::formats::ini;

pub fn read_lines(filename: &str){
    let file_type;
    match parse_file_type(filename).as_str() {
        "ini" => file_type = "ini".to_string(),
        "json" => file_type = "json".to_string(),
        "toml" => file_type = "toml".to_string(),
        _ => file_type = "unknown".to_string()
    }
    println!("{}", file_type);
    fs::read_to_string(filename).unwrap().lines().for_each(|line| {
        let res = ini::parse_line(line);
        match res {
            Some(_) => println!("{}", res.unwrap()),
            None => ()
        }
    });
}

fn parse_file_type(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| panic!("Unable to parse file type '{}'", filename))
}