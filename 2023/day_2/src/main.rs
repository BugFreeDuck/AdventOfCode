#![allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead, stdin}};

/*
    Trying to incorporate structs and tuples into workflow,
    Not the cleanliest solution with populating color counts in dynamic order :/
*/

#[derive(Debug)]
struct Game{
    id: i16,
    rounds: Vec<Handful>
}

#[derive(Debug)]
struct Handful(i32, i32, i32);

fn main() {
    let start = std::time::Instant::now();
    println!("Result: {}", solve_part_2());
    eprintln!("Solved in {:?}", start.elapsed());

    let _ = stdin().read_line(&mut String::new()).unwrap();
}

fn solve_part_2() -> i32{
    let mut acc: i32 = 0;
    
    let input_lines_iterator = read_input();
    for line in input_lines_iterator {
        let game = parse_line(line);
    
        let mut required_bag_contents = Handful(0, 0, 0);
        for round in game.rounds{
            if required_bag_contents.0 < round.0 { required_bag_contents.0 = round.0}
            if required_bag_contents.1 < round.1 { required_bag_contents.1 = round.1}
            if required_bag_contents.2 < round.2 { required_bag_contents.2 = round.2}
        }

        acc += required_bag_contents.0 * required_bag_contents.1 * required_bag_contents.2
    }

    return acc;
}

fn solve_part_1() -> i16{
    let bag_contents = Handful(12,13,14);

    let mut acc: i16 = 0;
    let input_lines_iterator = read_input();
    for line in input_lines_iterator {
        let game = parse_line(line);
        let game_valid = evaluate_legality(&game, &bag_contents);
        if game_valid {
            acc += game.id;
        }
    }

    return acc;
}

fn evaluate_legality(game: &Game, bag_contents: &Handful) -> bool{
    return !game.rounds.iter().any(| round | 
            round.0 > bag_contents.0 ||
            round.1 > bag_contents.1 ||
            round.2 > bag_contents.2);
}

fn parse_line(line: String) -> Game{
    let colon_idx = line.find(':').unwrap();
    let (game_info, rounds_info) = line.split_at(colon_idx + 1);
    return parse_game(game_info, rounds_info);
}

fn parse_game(game_info: &str, rounds_info: &str) -> Game{
    let id_str = game_info.trim_matches(':').split(' ').last().unwrap();
    let rounds_vec = parse_rounds(rounds_info);
    return Game {
        id: id_str.parse::<i16>().unwrap(),
        rounds: rounds_vec 
   }
}

fn parse_rounds(rounds_info: &str) -> Vec<Handful>{
    let mut rounds_vec = Vec::<Handful>::new();
    let rounds = rounds_info.split(';');
    for round in rounds {
        let mut round_handful = Handful(0, 0, 0);
        
        let colors_parsed = parse_colors(round);
        for (count, color_str) in colors_parsed{
            match color_str {
                "red" => round_handful.0 = count,
                "green" => round_handful.1 = count,
                "blue" => round_handful.2 = count,
                _ => panic!()
            }
        }    

        rounds_vec.push(round_handful);
    }

    return rounds_vec;
}


fn parse_colors(round: &str) -> Vec<(i32, &str)>{
    let colors_raw = round.split(',').collect::<Vec<&str>>();

    return colors_raw.iter()
        .map(|color_str| color_str.trim().split_once(' ').unwrap())
        .map(|(count_str, color_str)| (count_str.trim().parse::<i32>().unwrap(), color_str))
        .collect::<Vec<(i32, &str)>>();
}

fn read_input() -> impl Iterator<Item = String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join("input.txt")).expect("input file missing");
    let buf = BufReader::new(file);
    
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"));
}