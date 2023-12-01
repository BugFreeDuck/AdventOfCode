use std::{
    env,
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

fn main() {
    let input_lines = read_input();

    let mut curr_dir = "/".to_owned();
    let mut dir_statistics = HashMap::<String, i32>::new();
    for line in input_lines {
        let split_vec = line.split(' ')
            .map(|str| str.trim())
            .collect::<Vec<&str>>();

        // Reading an operation
        if split_vec.len() == 3 {
            if split_vec[1] == "cd"{
                let param_val = split_vec[2];
                curr_dir = cd(&curr_dir, param_val);
            }
            if split_vec[1] == "ls"{
                continue;
            }
        }

        // Reading ls output
        if split_vec.len() == 2{
            if split_vec[1] == "ls"{
                continue;
            }

            if split_vec[0] == "dir"{
                let new_dir = curr_dir.clone() + split_vec[1];               
                dir_statistics.insert(new_dir, 0);
            }
            else{
                let key = curr_dir.as_str();  
                let file_size = split_vec[0].parse::<i32>().unwrap();
                if dir_statistics.contains_key(key){
                    *dir_statistics.get_mut(key).unwrap() += file_size;
                };
            }
        }
    }
}


//current_dir = "/"
//cd_value = "a"
fn cd (current_dir: &str, cd_value: &str) -> String{
    let mut new_dir = current_dir.clone().to_owned();
    if cd_value == ".."{
        
    }
    else if cd_value == "/"{
        return cd_value.to_string()
    }
    else {
        let append_val = "/".to_owned() + cd_value;
        new_dir.push_str(append_val.as_str()) ;
    }

    return new_dir;
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