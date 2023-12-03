#![allow(dead_code)]
use std::{env, fs::File, io::{BufReader, BufRead}, time::Instant};

#[derive(Debug)]
struct Numeral{
    x_pos: u8,
    y_pos: u8,
    length: u8,
    value: u32,
}

#[derive(Debug)]
struct Symbol {
    x_pos: u8,
    y_pos: u8,
}

fn main() {
    let now = Instant::now();

    let matrix = read_input();
    let (numerals, symbols) = prase_input(&matrix);
    let result = solve_part_2(&numerals, &symbols);

    let elapsed = now.elapsed();

    println!("{}", result);
    println!("{:?}", elapsed);
}

// ~ 15ms
fn solve_part_2(numerals: &Vec<Numeral>, symbols: &Vec<Symbol>) -> u32{
    let mut result: u32 = 0;
    for sym in symbols{
        let adjecent_numerals = numerals.iter().filter(|numeral|
            sym.y_pos.abs_diff(numeral.y_pos) <= 1 &&
            (numeral.x_pos <= sym.x_pos + 1 && sym.x_pos - 1 <= numeral.x_pos + numeral.length - 1)
        ).collect::<Vec<&Numeral>>();
    
        if adjecent_numerals.len() > 1{
            result += adjecent_numerals
                .iter()
                .map(|x| x.value)
                .fold(1, |acc, x| acc * x);
        }
    }

    return result;
}

// ~ 10ms
fn solve_part_1(numerals: &Vec<Numeral>, symbols: &Vec<Symbol>) -> u32{
    let mut result: u32 = 0;
    for numeral in numerals{
        let adjacent_symbol = symbols.iter().find(|sym|
            // -1 < y < +1
            sym.y_pos.abs_diff(numeral.y_pos) <= 1 &&
            // x1 <= y2 && y1 <= x2
            (numeral.x_pos <= sym.x_pos + 1 && sym.x_pos - 1 <= numeral.x_pos + (numeral.length - 1))
        );
    
        if adjacent_symbol.is_some() {
            result += numeral.value;
        }
    }

    return result;
}

fn prase_input(matrix: &Vec<Vec<char>>) -> (Vec<Numeral>, Vec<Symbol>){
    let mut numerals = Vec::<Numeral>::new();
    let mut symbols = Vec::<Symbol>::new();

    let mut buf_x: Option<u8> = None;
    let mut buf_y: Option<u8> = None;
    let mut buf_value: Option<u32> = None;
    let mut buf_length: Option<u8> = None;

    for (row_idx , row) in matrix.iter().enumerate(){
        for (col_idx, value) in row.iter().enumerate(){
            if value.is_numeric() && buf_x.is_none() {
                buf_y = Some(row_idx as u8);
                buf_x = Some(col_idx as u8);
                buf_length = Some(1 as u8);
                buf_value = Some(value.to_digit(10).unwrap() as u32);
                continue;
            }

            if buf_x.is_some() && value.is_numeric() {
                let length = buf_length.take().unwrap() + 1;
                buf_length = Some(length);

                let value = buf_value.take().unwrap() * 10 + 
                                    value.to_digit(10).unwrap() as u32;
 
                buf_value = Some(value);
                continue;
            }

            if buf_x.is_some() && !value.is_numeric() {
                numerals.push(Numeral { 
                    x_pos: buf_x.unwrap(),
                    y_pos: buf_y.unwrap(),
                    length: buf_length.unwrap(),
                    value: buf_value.unwrap() 
                });

                buf_x = None; buf_y = None; buf_value = None; buf_length = None;
            }

            if value != &'.' && value.is_ascii_punctuation() {
                symbols.push(Symbol {
                    x_pos: col_idx as u8,
                    y_pos: row_idx as u8,
                });
            }
        }
    }

    return (numerals, symbols);
}

fn read_input() -> Vec<Vec<char>> {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join("input.txt")).expect("no such file");
    let buf = BufReader::new(file);

    return buf.lines()
        .map(|l| 
            l.unwrap()
                .chars()
                .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
}