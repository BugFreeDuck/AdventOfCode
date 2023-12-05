#![allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap, time::Instant};

fn main() {
    let start_time = Instant::now();

    let input_lines = read_input("input.txt");
    let games = parse_input(input_lines);
    let result = solve_part_2(games);
    
    println!("Calculated in: {:?}", start_time.elapsed());
    dbg!(result);
}

fn solve_part_2(games: Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    let cards_matches  = count_card_matches(&games);
    let cards_won_by_card = cards_matches.iter().enumerate()
        .map(|(idx, count)| ((idx+1)..idx+(*count as usize) + 1).collect())
        .collect::<Vec<Vec<usize>>>();

    let mut memo = HashMap::<usize, u32>::new();
    let mut result:u32 = cards_matches.len() as u32;
    for (card_idx, _) in cards_matches.iter().enumerate() {
        result += count_total_winnings(card_idx, &cards_matches, &cards_won_by_card, &mut memo);
    }

    return result;
}

fn count_total_winnings(
    idx: usize,
    cards_matches: &Vec<u8>,
    cards_won: &Vec<Vec<usize>>,
    memo: &mut HashMap<usize, u32>) -> u32
{
    let cached_total = memo.get(&idx);
    if cached_total.is_some(){
        return *cached_total.unwrap();
    } else {
        let new_cards = &cards_won[idx];
        let mut result = new_cards.len() as u32;
        let winings = new_cards.iter().map(|idx| {
           let sub_total = count_total_winnings(*idx, &cards_matches, &cards_won, memo);
           memo.insert(*idx, sub_total);
           return sub_total;
        }).sum::<u32>();

        result += winings;
        memo.insert(idx, result);
        return result;
    }
}

fn solve_part_1(games: Vec<(Vec<u8>, Vec<u8>)>) -> u32 {

    let card_matches = count_card_matches(&games);
    let result = card_matches.iter()
        .map(|winning_count|{
            if winning_count == &0 {
                return 0
            } 

            let mut acc:u32 = 1;
            for _ in 0..winning_count - 1 { acc *= 2 }
            return acc;
        }).sum();

    return result;
}

fn count_card_matches(games: &Vec<(Vec<u8>, Vec<u8>)>) -> Vec<u8>{
    return games.iter()
        .map(|(winning_numbers, ticket_numbers)| {
            return ticket_numbers.iter()
                .filter(|num| winning_numbers.contains(num))
                .count() as u8;
        }).collect::<Vec<u8>>();
}

fn parse_input(input_lines: impl Iterator<Item = String>) -> Vec<(Vec<u8>, Vec<u8>)> {
    return input_lines
        .map(|line| {
            let colon_idx = line.find(':').unwrap();
            let bar_idx = line.find('|').unwrap();

            let winning_str = line[colon_idx..bar_idx].trim_start_matches(':').trim();
            let ticket_str = line[bar_idx..].trim_start_matches('|').trim();

            return (
                parse_numbers(winning_str),
                parse_numbers(ticket_str)
            ); 
        }).collect::<Vec<(Vec<u8>, Vec<u8>)>>();
}

fn parse_numbers(number_str: &str) -> Vec<u8>{
    return number_str
        .split_ascii_whitespace()
        .map(|num_str| u8::from_str_radix(num_str, 10).unwrap())
        .collect::<Vec<u8>>();
}

fn read_input(path: &str) -> impl Iterator<Item = String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join(path)).expect("input file missing");
    let buf = BufReader::new(file);
    
    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"));
}