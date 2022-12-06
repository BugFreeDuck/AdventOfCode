use std::{fs::File, env, io::Read};


fn main(){
    let input = read_input();
    let chars = input.as_str().chars().collect::<Vec<char>>();
    
    for (idx, _) in chars.iter().enumerate(){
        if idx < 13{
            continue;
        } 

        let slice = &input[idx-13..idx+1];
        let mut buf = slice.as_bytes().to_vec();
        
        buf.sort();
        buf.dedup();
        if buf.len() == slice.len()
        {
            println!("Answer is idx: {}", idx+1);
            break;
        }
    }    

    println!("No answer found");
}

fn read_input() -> String{
    let path = env::current_dir().unwrap();
    let mut buf = String::new(); 
    File::open(path.join("input.txt")).unwrap().read_to_string(&mut buf);
    return buf;
}