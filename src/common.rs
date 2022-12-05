use std::fs;

pub fn read_file_to_lines(filename: &str) -> Vec<String> {
    let filedata = fs::read_to_string(filename).unwrap();
    filedata.split('\n').map(|e| e.to_string()).collect()
}
