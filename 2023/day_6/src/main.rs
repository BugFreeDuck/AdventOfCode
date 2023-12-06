use std::{env, fs::File, io::{BufReader, BufRead}};

fn main() {
    let input = get_input_lines("input.txt");

    let (time_line, distance_line) = (input.first().unwrap(), input.last().unwrap());
    let (times, distances) = (
        parse_numbers(&time_line[10..time_line.len()]),
        parse_numbers(&distance_line[10..distance_line.len()]),
    ); 

    let rounds = times.into_iter().zip(distances.into_iter()).collect::<Vec<(u64, u64)>>();

    let result = solve_part_1(rounds);
    println!("Result:  {}", result);
}

fn solve_part_1(rounds: Vec<(u64, u64)>) -> u64{
    let mut round_results = Vec::<u64>::new();
    for (time, required_distance) in rounds{
        let mut acc = 0;
        
        for time_held in 0..time+1 {
            let distance = time_held * (time - time_held);            
            if distance > required_distance {
                acc += 1;
            }
        }

        round_results.push(acc);
    }

    return *round_results.first().unwrap();
}

fn parse_numbers(number_str: &str) -> Vec<u64>{
    return number_str
        .split_ascii_whitespace()
        .map(|num_str| u64::from_str_radix(num_str, 10).unwrap())
        .collect::<Vec<u64>>();
}

fn get_input_lines(path: &str) -> Vec<String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join(path)).expect("input file missing");
    let buf = BufReader::new(file);
    
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
}
