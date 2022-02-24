use std::fs::File;
use std::io::{Read, Result};

pub fn read_input_from_file_into_string(file_name: String) -> Result<String> {
    let mut content = String::new();
    let mut file = File::open(file_name)?;
    file.read_to_string(&mut content);

    Ok(content)
}
