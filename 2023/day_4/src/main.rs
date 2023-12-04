use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap};

fn main() {
    let input_lines = read_input("test_input.txt");
    let games = parse_input(input_lines);
    
    let result = solve_part_2(games);
    
    dbg!(result);
}

fn solve_part_2(games: Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    let winning_counts = count_card_matches(&games);
    let cards_won_by_card = winning_counts.iter().enumerate()
        .map(|(idx, count)| (idx..idx+(*count as usize)).collect())
        .collect::<Vec<Vec<usize>>>();

    let mut memo = HashMap::<usize, u32>::new();
    let mut result:u32 = winning_counts.len() as u32;
    for (card_idx, win_count) in winning_counts.iter().enumerate() {
      result += count_total_winnings(memo, card_idx, &winning_counts);
    }    

    

    return 1;
}

fn count_total_winnings(mut memo: HashMap<usize, u32>, idx: usize, winning_counts: &Vec<u8>) -> u32{
        
    let cached_total = memo.get(&idx);
    if cached_total.is_some(){
        return *cached_total.unwrap();
    } else {
        
        let winings = count_total_winnings(, idx, winning_counts)

        memo.insert(1, 1);
        return 1;
    }
}


fn solve_part_2_dp(games: Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    let winning_counts = count_card_matches(&games);

    let count_won_by_card = winning_counts.iter().enumerate()
        .map(|(idx, count)| (idx, *count))
        .collect::<Vec<(usize, u8)>>();

    let cards_won_by_card = winning_counts.iter().enumerate()
        .map(|(idx, count)| (idx..idx+(*count as usize)).collect())
        .collect::<Vec<Vec<usize>>>();


    let mut total_won_by_card = HashMap::<usize, u32>::new();

    let mut cards_from_lowest = count_won_by_card.clone();
    cards_from_lowest.sort_by(|a, b| a.1.cmp(&b.1));


    dbg!(&cards_from_lowest);

    let mut acc: u32 = winning_counts.len() as u32;
    for (card_idx, win_count) in cards_from_lowest.iter() {
        println!("Current card {}, won {}", card_idx, win_count);
        dbg!(&total_won_by_card);

        let cached_total = total_won_by_card.get(&card_idx);
        if(cached_total.is_some()){
            acc += cached_total.unwrap();
            continue;
        }

        let cards_won = &cards_won_by_card[*card_idx];
        if cards_won.len() == 0 {
            total_won_by_card.insert(*card_idx, 0);
            continue;
        }

        let mut local_acc = 0;
        for card in cards_won {
            local_acc += total_won_by_card.get(card).unwrap();
        }

        total_won_by_card.insert(*card_idx, local_acc);    
    }    

    

    return 1;
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