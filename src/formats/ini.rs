use std::collections::HashMap;
use std::fs;

pub fn parse_file(filename: &str) -> Option<HashMap<String, HashMap<String, String>>> {
    let mut map = HashMap::<String, HashMap<String, String>>::new();

    let mut segment = String::new();
    fs::read_to_string(filename).unwrap().lines().map(|l| l.trim()).for_each(|line| {
        if line.starts_with('[') && line.ends_with(']') && line.len() >= 2 {
            segment = (&line[1..line.len() - 1]).to_string();
            map.insert(segment.to_string(), HashMap::<String, String>::new());
        }
        if let Some((key, value)) = line.split_once('=') {
            if map.contains_key(&segment) {
                map.get_mut(&segment).unwrap().insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    });

    if map.is_empty() {
        return None
    }
    Some(map)
}