use supply_stacks::read_input;

const RADIX: u32 = 10;

fn main(){

    let input_str = read_input();
    let input_lines = input_str.lines().collect::<Vec<&str>>();
    let empty_line_idx = input_lines.iter().position(|line| line.is_empty()).unwrap();

    let stack_count = count_stacks(input_lines[empty_line_idx-1]);
    let stack_lines = input_lines.clone()[0..(empty_line_idx - 1)].to_owned();
    let mut crate_stacks = parse_stacks(stack_lines, stack_count);

    let operation_lines = input_lines.clone()[(empty_line_idx+1)..input_lines.len()].to_owned();
    let operations = parse_operations(operation_lines);
    let mut temp_vec = Vec::<char>::new();
    for (move_count, from_idx, to_idx) in operations{
        temp_vec.clear();
        for _ in 0..move_count{
            let buf = crate_stacks[from_idx].pop().unwrap();
            temp_vec.push(buf);
        }

        for _ in 0..move_count{
            let buf = temp_vec.pop().unwrap();
            crate_stacks[to_idx].push(buf);
        }         
    }

    for stack in &crate_stacks{
        print!("{}", stack.last().unwrap())
    }
}

fn count_stacks(number_line: &str) -> u8{
    
    let stack_count = number_line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|num| num.to_digit(RADIX).unwrap() as u8 )
        .max()
        .unwrap();

    return stack_count;
}

fn parse_stacks(stack_lines: Vec<&str>, stack_count: u8) -> Vec<Vec<char>>{
    let mut stack_vec: Vec<Vec<char>> = std::iter::repeat_with(|| Vec::<char>::new())
        .take(usize::from(stack_count))
        .collect();

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

    stack_vec
}

fn parse_operations(operation_lines: Vec<&str>) -> Vec<(u8, usize, usize)>{
    operation_lines
        .iter()
        .map(|line| {
            line.split(' ')
                .filter(|word| word.bytes().all(|b| b.is_ascii_digit()))
                .map(|word| word.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .map(|nums| {
            (nums[0], (nums[1] - 1) as usize, (nums[2] - 1) as usize)
        })
        .collect::<Vec<(u8, usize, usize)>>()
}