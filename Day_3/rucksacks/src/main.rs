

use std::{path::Path, fs::{File}, collections::HashSet};
use std::io::{BufReader, BufRead};
use std::env;

fn main() {
    let char_value_map = vec![
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z', 'A', 'B', 'C', 'D', 
        'E', 'F', 'G', 'H', 'I', 
        'J', 'K', 'L', 'M', 'N', 
        'O', 'P', 'Q', 'R', 'S', 
        'T', 'U', 'V', 'W', 'X', 
        'Y', 'Z' 
    ];


    let active_dir = env::current_dir().unwrap();
    let file_path = active_dir.join("input.txt").to_path_buf();
    
    let mut sum = 0;
    let mut reader = initialize_input_reader(&file_path);
    let line_buf = &mut String::new();
    while let Ok(count) = reader.read_line(line_buf){
        if count == 0 { break; }
        let mid_idx = line_buf.trim().len() / 2;
        let rucksack_a: HashSet<char> = line_buf[0..mid_idx].chars().collect();
        let rucksack_b: HashSet<char> = line_buf[mid_idx..].chars().collect();        

        let intersection = rucksack_a.intersection(&rucksack_b);
        let dup_vec: Vec<&char> = intersection.collect();
        let char = dup_vec.first().unwrap().to_owned();
        
        sum += get_char_value(&char_value_map, char);

        line_buf.clear();
    }    

    println!("Result: {}", sum);
}

fn get_char_value(map: &Vec<char>, char: &char) -> i32{
    let idx = map
        .iter()
        .position(|x| x == char)
        .unwrap();

    return idx as i32 + 1;
}

fn initialize_input_reader(path: &Path) -> BufReader<File>{
    let file_result = File::open(path);
    let file = match file_result {
        Ok(file) => file,
        Err(_) => panic!("Could not read file"),
    };

    return BufReader::new(file);
}