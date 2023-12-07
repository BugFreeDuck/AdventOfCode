#![allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead}, collections::HashMap, cmp::Ordering};

/*
If not for this gentleman, I would've gave up
https://www.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/
*/


fn main() {
    let lines = get_input_lines("input.txt");
    let games = parse_games(&lines);

    let result = solve_part_2(games);
    println!("Result: {}", result);
}

fn solve_part_2(games: Vec<(&str, u64)>) -> u64{
    let char_num_map: HashMap<char, u32> = HashMap::from([
        ('J', 1),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]); 

    let card_counts = games.iter()
        .map(|(cards_str, _)| count_cards(cards_str, &char_num_map))
        .collect::<Vec<Vec<u64>>>();
    
    let hand_ratings = card_counts.iter()
        .map(|hand| rate_hand_with_jokers(hand));

    let mut game_data = games.iter().zip(hand_ratings).collect::<Vec<(&(&str, u64), u8)>>();

    game_data.sort_by(|a, b|{
        if a.1 > b.1{
            return Ordering::Greater;
        } else if b.1 > a.1 {
            return Ordering::Less;
        } else {
            for (a_char, b_char) in a.0.0.chars().zip(b.0.0.chars()){
                let a_val = parse_char_value(&a_char, &char_num_map);
                let b_val = parse_char_value(&b_char, &char_num_map);
                if a_val > b_val {
                    return Ordering::Greater
                } else if a_val < b_val{
                    return Ordering::Less
                }
            }

            panic!();
        }
    });

    return game_data.iter().enumerate()
        .map(|(idx, ((hand, score), strength))| 
        {
            let multiplier = idx as u64 + 1;
            println!("{} {} strength: {}", hand, score, strength);
            score * multiplier
        })
        .sum();
}

fn solve_part_1(games: Vec<(&str, u64)>) -> u64{
    let char_num_map: HashMap<char, u32> = HashMap::from([
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]); 

    let card_counts = games.iter()
        .map(|(cards_str, _)| count_cards(cards_str, &char_num_map))
        .collect::<Vec<Vec<u64>>>();
    
    let hand_ratings = card_counts.iter()
        .map(|hand| rate_hand(hand));

    let mut game_data = games.iter().zip(hand_ratings).collect::<Vec<(&(&str, u64), u8)>>();

    game_data.sort_by(|a, b|{
        if a.1 > b.1{
            return Ordering::Greater;
        } else if b.1 > a.1 {
            return Ordering::Less;
        } else {
            for (a_char, b_char) in a.0.0.chars().zip(b.0.0.chars()){
                let a_val = parse_char_value(&a_char, &char_num_map);
                let b_val = parse_char_value(&b_char, &char_num_map);
                if a_val > b_val {
                    return Ordering::Greater
                } else if a_val < b_val{
                    return Ordering::Less
                }
            }

            panic!();
        }
    });

    return game_data.iter().enumerate()
        .map(|(idx, ((hand, score), strength))| 
        {
            let multiplier = idx as u64 + 1;
            score * (idx as u64 + 1)
        })
        .sum();
}

fn rate_hand_with_jokers(card_counts: &Vec<u64>) -> u8 {
    let joker_count = card_counts[0];
    let highest_card_count = card_counts[1..].iter().max().unwrap();

    dbg!(joker_count);
    dbg!(highest_card_count);

    //Five of a kind
    if highest_card_count + joker_count >= 5 {
        return 6;
    } 
    
    //Four of a kind
    else if highest_card_count + joker_count >= 4 {
        return 5;
    }
    
    let contains_three = card_counts.contains(&3);
    let pair_count = card_counts[1..].iter().filter(|x| *x == &2).count() as u8;
    let contains_one = card_counts.contains(&1);
    
    //FullHouse
    if contains_three && pair_count == 1{
        return 4
    } else if contains_three && contains_one && joker_count >= 1 {
        return 4
    } else if contains_three && joker_count >= 2 {
        return 4;
    } else if pair_count == 2 && joker_count == 1 {
        return 4;
    } else if pair_count == 1 && contains_one && joker_count == 2 {
        return 4;
    }
    //Three
    else if contains_three {
        return 3;
    } else if pair_count == 1 && joker_count == 1{
        return 3;
    } else if contains_one && joker_count == 2{
        return 3;
    } else if !contains_one && joker_count == 3{
        return 3;
    }
    //Two Pairs
    else if pair_count == 2 {
        return 2;
    } else if pair_count == 1 && contains_one && joker_count == 1 {
        return 2;
    } else if !contains_one && joker_count == 2 {
        return 2;
    } 
    //Pair
    else if pair_count == 1 {
        return 1;
    }  if contains_one && joker_count == 1 {
        return 1;
    }
    //High card
    else {
        return 0;
    }
}
 
fn rate_hand(card_counts: &Vec<u64>) -> u8 {
    if card_counts.contains(&5){
        return 6;
    } else if card_counts.contains(&4) {
        return 5;
    } 
    
    let contains_three = card_counts.contains(&3);
    let pair_count = card_counts.iter().filter(|x| *x == &2).count() as u8;
    
    if contains_three && pair_count == 1{
        return 4
    } else if contains_three {
        return 3;
    } else if pair_count == 2 {
        return 2;
    } else if pair_count == 1 {
        return 1;
    } else {
        return 0;
    }
}

fn count_cards(cards_str: &str, char_num_map: &HashMap<char, u32>) -> Vec<u64> {  
    let mut card_counts = vec![0; 13];
    cards_str.chars().for_each(|char|{
        let num_val = parse_char_value(&char, &char_num_map) as usize;
        card_counts[num_val - 1] += 1;
    });

    return card_counts;
} 

fn parse_char_value(char: &char, char_num_map: &HashMap<char, u32>) -> u8{
    if char.is_numeric(){
        return char.to_digit(10).unwrap() as u8;
    } else {
        return *char_num_map.get(&char).unwrap() as u8;
    }
}
 
fn parse_games(game_stings: &Vec<String>) -> Vec<(&str, u64)>{
    return game_stings.iter().map(|game_str| {
        let space_char_idx = game_str.find(' ').unwrap();
        return (
            &game_str[..space_char_idx],
            u64::from_str_radix(&game_str[space_char_idx..game_str.len()].trim(), 10).unwrap()
        );
    }).collect::<Vec<(&str, u64)>>();
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
