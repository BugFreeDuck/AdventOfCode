#[allow(dead_code)]
use std::vec;
use std::{env, fs::File, io::{BufReader, BufRead}};
use colored::Colorize;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Position{
    x: usize,
    y: usize
}

#[derive(Debug)]
enum Direction{
    North,
    East,
    South,
    West
}

fn main() {
    let input_iterator = get_input_lines("test_input.txt");
    
    let matrix = input_iterator
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();


    let start_pos = find_start(&matrix).unwrap();    
    // let path = folow_path(&matrix);

    // dbg!(path);
    print_matrix(matrix, &start_pos);
    // println!("Current pos: {:?}", current_pos); 

}

fn folow_path(matrix: &Vec<Vec<char>>) -> Vec<Position> {
    let start_pos = find_start(&matrix).unwrap();
    let mut current_pos = start_pos.clone();
    let mut prev_pos: Option<&Position> = None;

    dbg!(&current_pos);

    let mut path = Vec::<Position>::new();
    path.push(current_pos.clone());

    let mut steps_taken: u64 = 0;
    loop {
        let new_pos = get_valid_neigbor(&current_pos, prev_pos, matrix);    
        path.push(new_pos.clone());
        
        prev_pos = Some(&current_pos);
        // current_pos = new_pos.clone();
        steps_taken += 1;


        if steps_taken == 10 { break; }
        if current_pos.eq(&start_pos) { break; }
    }

    return path;
}

fn get_valid_neigbor(pos: &Position, prev_pos: Option<&Position>, matrix: &Vec<Vec<char>>) -> Position{
    let neighbors = [
        (Direction::North , matrix_get(pos.x, pos.y - 1, matrix), Position { x: pos.x, y: pos.y - 1}),
        (Direction::East  , matrix_get(pos.x + 1, pos.y, matrix), Position { x: pos.x + 1, y: pos.y}),
        (Direction::South , matrix_get(pos.x, pos.y + 1, matrix), Position { x: pos.x, y: pos.y + 1}),
        (Direction::West  , matrix_get(pos.x - 1, pos.y, matrix), Position { x: pos.x - 1, y: pos.y})
    ];
   
    let pipe_part = neighbors.iter()
        .find(|(direction, value, next_pos)| {
            if value.is_none() { return false; }
            else{
                let val = value.unwrap();
                return !val.eq(&'.') &&
                    direction_char_compatible(direction, val) &&
                    (prev_pos.is_some() && prev_pos.unwrap().eq(next_pos))
            }
        })
        .unwrap();

    return pipe_part.2.to_owned();
}

fn direction_char_compatible(dir: &Direction, char: &char) -> bool{
    match dir {
        Direction::North => ['|', 'F', '7'].contains(char),
        Direction::East  => ['-', 'J', '7'].contains(char),
        Direction::South => ['|', 'L', 'J'].contains(char),
        Direction::West  => ['-', 'L', 'F'].contains(char),
    }
}

fn matrix_get(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> Option<&char>{
    return match matrix.get(y){
        Some(row) => row.get(x),
        None => None,
    }
}

fn find_start(matrix: &Vec<Vec<char>>) -> Option<Position>{
    for (y, row) in matrix.iter().enumerate(){
        for (x, char) in row.iter().enumerate(){
            if char.eq(&'S') {
                return Some(Position { x, y});
            }
        }
    }

    return None;
}

fn print_matrix(matrix: Vec<Vec<char>>, current_pos: &Position) {
    println!("Print matrix:");
    for (y, row) in matrix.iter().enumerate(){ 
        for (x, char) in row.iter().enumerate()
        {
            if current_pos.x == x && current_pos.y == y {
                print!("{}", char.to_string().as_str().green());
            }
            else {
                print!("{}", char);
            }
        }

        println!("");
    }
}

fn get_input_lines(path: &str) -> impl Iterator<Item = String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join(path)).expect("input file missing");
    let buf = BufReader::new(file);
    
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"));
}
