use core::panic;
#[allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
struct Position{
    x: usize,
    y: usize
}

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Direction{
    North,
    East,
    South,
    West
}

fn main() {
    let input_iterator = get_input_lines("input.txt");

    let matrix = input_iterator
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_pos = find_start(&matrix).unwrap();
    let mut current_pos = start_pos;
    let mut next_direction = get_first_direction(&current_pos, &matrix);
    
    move_in_direction(&mut current_pos, &next_direction);
    let mut steps_taken = 1;    

    loop{
        let last_dir = next_direction;
        next_direction = get_next_direction(&current_pos, &last_dir, &matrix);
        move_in_direction(&mut current_pos, &next_direction);

        steps_taken += 1;
        if start_pos == current_pos { break;}
    }

    println!("Result: {}", steps_taken/2);
}

fn move_in_direction(pos: &mut Position, dir: &Direction){
    match dir {
        Direction::North => pos.y = pos.y - 1,
        Direction::East =>  pos.x = pos.x + 1,
        Direction::South => pos.y = pos.y + 1,
        Direction::West =>  pos.x = pos.x - 1
    };
}

fn get_next_direction(pos: &Position, last_dir: &Direction, matrix: &Vec<Vec<char>>) -> Direction{
    let current_char = matrix[pos.y][pos.x];

    return match last_dir {
        Direction::North => match current_char { 'F' => Direction::East,  '7' => Direction::West,  '|' => Direction::North, _ => panic!() },
        Direction::South => match current_char { 'L' => Direction::East,  'J' => Direction::West,  '|' => Direction::South, _ => panic!() },
        Direction::West =>  match current_char { 'F' => Direction::South, 'L' => Direction::North, '-' => Direction::West,  _ => panic!() },
        Direction::East =>  match current_char { '7' => Direction::South, 'J' => Direction::North, '-' => Direction::East,  _ => panic!() },
    };
}

fn get_first_direction(pos: &Position, matrix: &Vec<Vec<char>>) -> Direction
{
    let i_x = pos.x as isize;
    let i_y = pos.y as isize;
    let neighbors = [
        (Direction::North , matrix_get(i_x, i_y - 1, matrix)),
        (Direction::East  , matrix_get(i_x + 1, i_y, matrix)),
        (Direction::South , matrix_get(i_x, i_y + 1, matrix)),
        (Direction::West  , matrix_get(i_x - 1, i_y, matrix))
    ];

    let pipe_part = neighbors.iter()
        .find(|(direction, value)| {
            if value.is_none() { return false; }
            else{
                let val = value.unwrap();
                return direction_char_compatible(direction, val);
            }
        })
        .unwrap();

    return pipe_part.0;
}

fn direction_char_compatible(dir: &Direction, char: &char) -> bool{
    match dir {
        Direction::North => ['|', 'F', '7'].contains(char),
        Direction::East  => ['-', 'J', '7'].contains(char),
        Direction::South => ['|', 'L', 'J'].contains(char),
        Direction::West  => ['-', 'L', 'F'].contains(char),
    }
}

fn matrix_get(x: isize, y: isize, matrix: &Vec<Vec<char>>) -> Option<&char>{
    return match matrix.get(y as usize){
        Some(row) => row.get(x as usize),
        None => None,
    }
}

fn find_start(matrix: &Vec<Vec<char>>) -> Option<Position>{
    for (y, row) in matrix.iter().enumerate(){
        for (x, char) in row.iter().enumerate(){
            if char.eq(&'S') {
                return Some(Position { x, y });
            }
        }
    }

    return None;
}

fn get_input_lines(path: &str) -> impl Iterator<Item = String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join(path)).expect("input file missing");
    let buf = BufReader::new(file);

    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"));
}
