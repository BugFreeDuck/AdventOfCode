use core::panic;
use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let mut input_lines = get_input_lines("test_input.txt");

    let directions_str = input_lines.next().unwrap();
    let _ = input_lines.next(); // Skip empty line
    let instructions_map =parse_instructions_map(input_lines);
   
    let result = solve_part_1(directions_str, instructions_map);
    println!("Result: {}", result);
}

fn solve_part_2(directions_str: String, instructions_map: HashMap<String, (String, String)>) -> u64 {
    let mut steps_taken:u64 = 0 ;
    let mut current = "AAA";
    let end = "ZZZ";
    
    let mut dir_idx = 0;
    let direction_chars = directions_str.chars().collect::<Vec<char>>();
    while !current.eq(end){
        let (left, right) = instructions_map.get(current).unwrap();
        current = match direction_chars[dir_idx]{
            'L' => left,
            'R' => right,
            _ => panic!()
        };

        steps_taken += 1;
        dir_idx += 1;

        if dir_idx == direction_chars.len(){
            dir_idx = 0;
        }
    }

    return steps_taken;
}


fn solve_part_1(directions_str: String, instructions_map: HashMap<String, (String, String)>) -> u64 {
    let mut steps_taken:u64 = 0 ;
    let mut current = "AAA";
    let end = "ZZZ";
    
    let mut dir_idx = 0;
    let direction_chars = directions_str.chars().collect::<Vec<char>>();
    while !current.eq(end){
        let (left, right) = instructions_map.get(current).unwrap();
        current = match direction_chars[dir_idx]{
            'L' => left,
            'R' => right,
            _ => panic!()
        };

        steps_taken += 1;
        dir_idx += 1;

        if dir_idx == direction_chars.len(){
            dir_idx = 0;
        }
    }

    return steps_taken;
}

fn parse_instructions_map(input_lines: impl Iterator<Item = String>) -> HashMap<String, (String, String)>{
    return input_lines.map(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        let node:String = chars[..3].iter().collect();
        let left:String = chars[7..10].iter().collect();
        let right:String = chars[12..15].iter().collect();
        return (node, (left, right));
    }).collect::<HashMap<String, (String, String)>>();
}

fn get_input_lines(path: &str) -> impl Iterator<Item = String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join(path)).expect("input file missing");
    let buf = BufReader::new(file);
    
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"));
}
