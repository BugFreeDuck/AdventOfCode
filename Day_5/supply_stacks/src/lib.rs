use std::{fs::{self}, env};

pub fn read_input() -> String
{
    let directory = env::current_dir().unwrap();

    let path = directory.join("input.txt");
    return fs::read_to_string(path).unwrap();
}