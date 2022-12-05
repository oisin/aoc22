use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("December 05 / 1");

    // First tranche of inputs is the layout of the stacks
    // Read until a blank line is reached

    let mut stack_data: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./test_input_part1.txt") {
        for line in lines {
            if let Ok(stack) = line {
                if stack.is_empty() {
                    break;
                }
                stack_data.push(stack);
            }
        }
        println!(" stack looks like {:?}", stack_data);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
