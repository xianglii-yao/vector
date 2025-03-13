use std::io::{Error, read_to_string};

struct Document {
    rows: Vec<String>,
    file_name: String,
}

pub fn read_file(location: &str) -> Result<Vec<String>, Error> {}
