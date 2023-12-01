#![allow(dead_code)]

use std::{fs::File, env, io::{BufReader, BufRead}};

static LEGAL_WORDS: &'static [&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

/*
    Quite a lot of clutter
    Getting back to rust ways of doing things, 
    This time just went with general "Feeling" without focusing too much on correctness/cleanliness of the code
    To get answers for different parts, replace .map on line 20 with:
        first_digit()          for first part answer
        first_digit_or_word()  for second part answer
*/

fn main() {
    let input = read_input();
    let result: u32 = input.iter()
        .map(|x| 
            (first_digit_or_word(x.chars()) * 10) +
             first_digit_or_word(x.chars().rev()))
        .sum();

    println!("{}", result);
}

fn first_digit(chars: impl Iterator<Item = char>) -> u32 {
    for char in chars{
        if char.is_ascii_digit() {
            return char.to_digit(10).unwrap();
        }
    }

    return 0;    
}

fn first_digit_or_word(chars: impl Iterator<Item = char>) -> u32 {    
    let mut word_buff = Vec::new();
    for char in chars{
        if char.is_ascii_digit() {
            return char.to_digit(10).unwrap();
        }

        word_buff.push(char);

        let word: String = word_buff.iter().collect();
        match parse_legal_word(&word) {
            Some(number) => return number,
            None => (),
        };
    }

    return 0;    
}

fn parse_legal_word(test_word: &str) -> Option<u32>{
    for (value, word) in LEGAL_WORDS.iter().enumerate(){
        if test_word.contains(word) {
            return Some((value + 1) as u32)
        }

        let rev_test_word = word.chars().rev().collect::<String>();
        if test_word.contains(&rev_test_word) {
            return Some((value + 1) as u32) 
        }
    }

    None
}

fn read_input() -> Vec<String> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join("input.txt")).expect("no such file");
    let buf = BufReader::new(file);

    return buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}