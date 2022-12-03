use std::{path::Path, fs::{File}, collections::HashSet, vec};
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
    
    let mut vec_buf = Vec::<String>::new();
    let line_buf = &mut String::new();

    while let Ok(count) = reader.read_line(line_buf){
        if count == 0 { break; }
        
        vec_buf.push(line_buf.trim().to_string());
        if vec_buf.len() == 3{
            
            let a: HashSet<char> = vec_buf[0].chars().collect(); 
            let b: HashSet<char> = vec_buf[1].chars().collect();
            let c: HashSet<char> = vec_buf[2].chars().collect();
    
            let duplicates: Vec<&char> = a.iter()
                .filter(|set| b.contains(set))
                .filter(|set| c.contains(set)).collect();

            sum += get_char_value(&char_value_map, duplicates.first().unwrap());

            vec_buf.clear();
        }

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