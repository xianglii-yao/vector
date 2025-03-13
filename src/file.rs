use std::io::{read_to_string, Error};

struct Document {
    rows: Vec<String>,
    file_name: String
}

pub fn read_file(location: &str) -> Result<Vec<String>, Error> {
    let file = read_to_string(location);
    match file {
        Ok(doc) => doc,
        _=()
    }
}
