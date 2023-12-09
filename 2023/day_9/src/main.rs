#[allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead}};

fn main() {
    let input = get_input_lines("input.txt");

    let mut result: i64 = 0;
    for line in input{
        let initial_numbers = parse_numbers(line);
        let matrix = extrapolate_matrix(initial_numbers);

        result += count_part_2(matrix);
    }

    println!("Result: {}", result);
}

fn count_part_2(matrix: Vec<Vec<i64>>) -> i64 {
    matrix.iter().rev()
        .map(|numbers| *numbers.first().unwrap())
        .fold(0, |acc, num| num - acc)
}

fn count_part_1(matrix: Vec<Vec<i64>>) -> i64 {
    matrix.iter().rev()
        .map(|numbers| *numbers.last().unwrap())
        .sum()
}

fn extrapolate_matrix(initial_numbers: Vec<i64>) -> Vec<Vec<i64>>{
    let mut matrix = Vec::<Vec<i64>>::new();
    matrix.push(initial_numbers);

    loop{
        let new_line = matrix.last().unwrap()
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i64>>();

        matrix.push(new_line);

        if matrix.last().unwrap().iter().all(|x| x == &0) { break; }            
    }

    matrix
}

fn parse_numbers(line: String) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|str| str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn get_input_lines(path: &str) -> impl Iterator<Item = String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join(path)).expect("input file missing");
    let buf = BufReader::new(file);
    
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"));
}
