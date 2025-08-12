pub fn parse_line(line: &str) -> Option<&str> {
    let line = line.trim();
    if line.starts_with('[') && line.ends_with(']') && line.len() >= 2 {
        return Some(&line[1..line.len() - 1]);
    }
    if let Some((key, value)) = line.split_once('=') {
        println!("Key: '{}' value: '{}'", key.trim(), value.trim());
    }
    None
}