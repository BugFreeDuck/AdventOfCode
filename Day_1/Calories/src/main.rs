use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let current_dir = match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_owned(),
        Err(error) => panic!("{error}"),
    };

    let file_path = current_dir + "\\input.txt";
    // let max = max_calories::<String>(file_path);
    let top = top_calories::<String>(file_path);
    let total = top.0 + top.1 + top.2;


    print!("{:?}", total);
}

fn top_calories<P>(file_path: String) -> (i32, i32, i32)
where P: AsRef<Path>,
{
    let lines = read_lines(file_path).expect("Could not read lines");

    let mut calories_sums: Vec<i32> = Vec::new();

    let mut elf_calories: i32 = 0;
    for line in lines {
        let line_val = line.expect("No line");
        if line_val.is_empty() {
            calories_sums.push(elf_calories);
            elf_calories = 0;
            continue;
        }

        let int_val = line_val.parse::<i32>().unwrap();
        elf_calories += int_val;        
    }

    calories_sums.push(elf_calories);
    calories_sums.sort_by(|a, b| a.cmp(b));

    let results = calories_sums.as_slice()[calories_sums.len()-3..].to_vec();

    return (results[0], results[1], results[2]);
}



fn max_calories<P>(file_path: String) -> i32
where P: AsRef<Path>,
{
    let lines = read_lines(file_path).expect("Could not read lines");

    let mut max_calories: i32 = 0;
    let mut elf_calories: i32 = 0;
    for line in lines {
        let line_val = line.expect("No line");
        if line_val.is_empty() {
            if elf_calories > max_calories{
                max_calories = elf_calories;
            }

            elf_calories = 0;
            continue;
        }

        let int_val = line_val.parse::<i32>().unwrap();
        elf_calories += int_val;        
    }

    if elf_calories > max_calories{
        max_calories = elf_calories;
    }

    return max_calories;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
