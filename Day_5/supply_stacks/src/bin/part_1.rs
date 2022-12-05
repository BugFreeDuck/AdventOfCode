use supply_stacks::read_input;

fn main(){

    let input_str = read_input();
    let input_lines = input_str.lines().collect::<Vec<&str>>();
    let empty_line_idx = input_lines.iter().position(|line| line.is_empty()).unwrap();

    const RADIX: u32 = 10;
    let stack_count = input_lines[empty_line_idx-1]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|num| num.to_digit(RADIX).unwrap() as u8 )
        .max()
        .unwrap();

    let mut stack_vec: Vec<Vec<char>> = std::iter::repeat_with(|| Vec::<char>::new())
        .take(usize::from(stack_count))
        .collect();

    let stack_lines = input_lines.clone()[0..(empty_line_idx - 1)].to_owned();
    for line in stack_lines.into_iter().rev(){
        let crates = line
            .chars()
            .enumerate()
            .filter(|(_idx, c)| c.is_ascii_alphabetic())
            .map(|(idx, c)| ((idx + 2) / 4, c))
            .collect::<Vec<(usize,char)>>();

        for (idx, c) in crates{
            stack_vec[idx].push(c)
        }
    }

    // for line in &input_lines[0..(empty_line_idx - 1)]{
    //     println!("{}",line);
    // }

    // println!("{}", empty_line_idx);

    
    // let mut operation_vec = Vec::<&str>::new();

    // for line in input_lines[0..empty_line_idx].iter(){
        
    // }
    
}
