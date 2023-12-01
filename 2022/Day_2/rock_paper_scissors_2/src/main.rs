use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead};
use std::path::Path;
use std::{io, env};

#[derive(PartialEq, Eq, Hash)]
enum Options{
    Rock,
    Paper,
    Scissors
}

enum Action{
    Win,
    Draw,
    Loose,
}

fn main() {
    let oponnent_map = HashMap::from([
        ("A", Options::Rock),
        ("B", Options::Paper),
        ("C", Options::Scissors)
    ]);

    let suggestion_map = HashMap::from([
        ("X", Action::Loose),
        ("Y", Action::Draw),
        ("Z", Action::Win)
    ]);

    let strategy = HashMap::from([
        (Options::Rock, Options::Paper),
        (Options::Paper, Options::Scissors),
        (Options::Scissors, Options::Rock),
    ]);

    let loose_strategy = HashMap::from([
        (Options::Rock, Options::Scissors),
        (Options::Paper, Options::Rock),
        (Options::Scissors, Options::Paper),
    ]);

    let directory_path = env::current_dir().unwrap();
    let path = directory_path.join("input.txt");
    let mut reader = initialize_input_reader(path.as_path());

    let line_buf = &mut String::new(); 
    let mut score: i32 = 0;
    while let Ok(count) = reader.read_line(line_buf){
        if count == 0{ break; }

       let space_idx = line_buf.find(' ').unwrap();
        let (oponnent_str, player_str) = line_buf.trim().split_at(space_idx);

        let suggested_action = suggestion_map.get(player_str.trim()).unwrap();
        let oponnent_option = oponnent_map.get(oponnent_str.trim()).unwrap();
        let winning_option = strategy.get(oponnent_option).unwrap().to_owned();
        let loosing_option = loose_strategy.get(oponnent_option).unwrap().to_owned();
        
        let player_option  = match suggested_action{
            Action::Win => winning_option,  
            Action::Draw => oponnent_option,  
            Action::Loose => loosing_option 
        };

        match player_option{
            Options::Rock => score += 1,  
            Options::Paper => score += 2,  
            Options::Scissors => score += 3
        }

        match player_option{
            o if o == winning_option => score += 6,
            o if o == oponnent_option => score += 3,
            _ => score += 0
        }

        line_buf.clear();
    }

    println!("Score: {}", score);
}

fn initialize_input_reader(path: &Path) -> io::BufReader<File>{
    let file = File::open(path).expect("File at path not found");
    return io::BufReader::new(file);
}