use std::fs::{File};
use std::env;
use std::io::{self, BufRead, BufReader, Lines};

fn main() {
    let lines = read_input_lines().unwrap();

    let mut result: i32 = 0;
    for line_result in lines{
        let line = line_result.unwrap(); 
        let range_str = line
            .trim()
            .split(',')
            .collect::<Vec<&str>>();

            let range_tuples = range_str.iter()
            .map(|item|{
                let mut split = item.splitn(2, '-');
                let low_edge = split.next().unwrap().parse::<i32>().unwrap();
                let high_edge = split.next().unwrap().parse::<i32>().unwrap();

                (low_edge, high_edge)
            })
            .collect::<Vec<(i32,i32)>>();

            let (start1, end1) = range_tuples.first().unwrap();
            let (start2, end2) = range_tuples.last().unwrap();
            
            if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
                result += 1;
            }            
    }

    println!("Result: {}", result);
}

fn read_input_lines() -> io::Result<Lines<BufReader<File>>>
{
    let directory = env::current_dir().unwrap();

    let path = directory.join("input.txt");
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}