#[allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let mut input_lines = get_input_lines("input.txt");

    let directions_str = input_lines.next().unwrap();
    let _ = input_lines.next(); // Skip empty line
    let instructions_map =parse_instructions_map(input_lines);
   
   
    let result = solve_part_2(directions_str, instructions_map);
    println!("Result: {}", result);
}

fn solve_part_2(directions_str: String, instructions_map: HashMap<String, (String, String)>) -> u128 {
    let direction_chars = directions_str.chars().collect::<Vec<char>>();
    let mut ghosts = instructions_map.iter()
        .filter(|(ghost, (_, _))| ghost.ends_with('A'))
        .map(|(ghost, (_, _))| ghost)
        .collect::<Vec<&String>>();

    let mut steps_taken:u128 = 0 ;  
    let mut z_cycle_steps = HashMap::<usize, u128>::new();
    let mut z_assignments = 0;
    for dir in direction_chars.iter().cycle(){
        steps_taken += 1;
        for (idx, ghost) in ghosts.iter_mut().enumerate(){
            if z_cycle_steps.contains_key(&idx){
                continue;
            }

            let (left, right) = instructions_map.get(*ghost).unwrap();
            *ghost = match dir{
                'L' => left,
                'R' => right,
                _ => panic!()
            };

            if ghost.ends_with('Z'){
                z_cycle_steps.insert(idx, steps_taken);
                z_assignments += 1;
            }
        }

        if z_cycle_steps.len() == ghosts.len() { break; }
    }
    
    let steps_taken = z_cycle_steps.iter().map(|(_, steps)| *steps).collect::<Vec<u128>>();
    return find_lcm(steps_taken);
}

fn find_lcm(steps_vec: Vec<u128>) -> u128 {
    let mut result = lcm(steps_vec[0], steps_vec[1]);
    for num in steps_vec.iter().skip(2) {
        result = lcm(result, *num);
    }

    return result;
}

fn lcm(a:u128, b:u128) -> u128 {
    let mut x: u128 = 0;
    let mut y: u128 = 0;
    
    if a > b {
        x = a;
        y = b;
    }
    else {
        x = b;
        y = a;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    return a * b / y;
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
