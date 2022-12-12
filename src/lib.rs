use std::env;
use std::fs;

pub fn read_file(folder: &str, day: u8) -> String {
    let directory = env::current_dir().unwrap();
    let path = directory.join("src").join(folder).join(format!("{:02}.txt", day));
    fs::read_to_string(path).expect("File could not be opened.")
}