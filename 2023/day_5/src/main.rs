#![allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead}, time::Instant};

fn main() {
    let start_time = Instant::now();

    let mut lines = get_input_line_iterator("input.txt");
    let result = solve_part_2(&mut lines);
    
    println!("Finished in: {:?}", start_time.elapsed());
    println!("Result: {}", result);
}

// ~200s
fn solve_part_2(lines: &mut impl Iterator<Item = String>) -> u64 { 
    let min_locations = lines.next().map(|seeds_str| {
        let colon_idx = seeds_str.find(':').unwrap();
        let (_, numbers_str) = seeds_str.split_at(colon_idx+2);
        let seed_numbers = parse_numbers(numbers_str);
        
        let lines_materialized = lines.collect::<Vec<String>>();
        let mut mins = Vec::<u64>::new();
        for idx in (1..seed_numbers.len()).step_by(2) {            
            let mut seeds = (seed_numbers[idx-1]..seed_numbers[idx-1] + seed_numbers[idx]).collect::<Vec<u64>>();            
            mins.push(apply_almanach(&mut seeds, &mut lines_materialized.clone().into_iter()));
        }
        
        return mins;
    }).unwrap();

    return min_locations.iter().min().unwrap().clone();
}

// ~ 1ms
fn solve_part_1(lines: &mut impl Iterator<Item = String>) -> u64 {   
    let mut seeds = lines.next().map(|seeds_str|{
        let colon_idx = seeds_str.find(':').unwrap();
        let (_, numbers_str) = seeds_str.split_at(colon_idx+2);
        return parse_numbers(numbers_str);
    }).unwrap();
    
    return apply_almanach(&mut seeds, lines);   
}

fn apply_almanach(seeds: &mut Vec<u64>, lines: &mut impl Iterator<Item = String>) -> u64 {
    lines.next();

    let mut map = Vec::<(u64, u64, u64)>::new();
    while let Some(line) = lines.next()  {
        if line.is_empty(){
            apply_map(seeds, &map);
            map.clear();
            continue;
        }
        
        if line.contains(':'){
            continue;
        } else{
            let map_numbers = parse_numbers(&line);
            let map_line = (map_numbers[0], map_numbers[1], map_numbers[2]);
            
            map.push(map_line);
        }
    }

    apply_map(seeds, &map);
    return *seeds.iter().min().unwrap() 
}

fn apply_map(input: &mut Vec<u64>, map: &Vec<(u64, u64, u64)>) -> Vec<u64>{
    input.iter_mut().for_each(| value | {
         let corresponding_map = map.iter()
            .find(| (_, source, count) |  *value >= *source && *value < *source + count );

        if let Some((destination, source, _,)) = corresponding_map{
            let offset = *value - source;
            *value = *destination + offset;
        }
    });
    
    Vec::<u64>::new()
}

fn parse_numbers(number_str: &str) -> Vec<u64>{
    return number_str
        .split_ascii_whitespace()
        .map(|num_str| u64::from_str_radix(num_str, 10).unwrap())
        .collect::<Vec<u64>>();
}

fn get_input_line_iterator(path: &str) -> impl Iterator<Item = String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join(path)).expect("input file missing");
    let buf = BufReader::new(file);
    
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"));
}
