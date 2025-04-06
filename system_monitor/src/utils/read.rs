
use std::fs;

pub fn read_file(path: &str) -> Option<String> {
    match fs::read_to_string(path){
        Ok(content) => Some(content),
        Err(_) => None,
    }
}
