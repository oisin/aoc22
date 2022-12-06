use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("December 05 / 2");

    // First tranche of inputs is the layout of the stacks
    // Read until a blank line is reached

    let mut stack_data: Vec<String> = Vec::new();
    let mut move_data: Vec<Vec<usize>> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        let mut stack_collecting = true;

        for line in lines {
            if let Ok(instruction) = line {
                if stack_collecting {
                    if instruction.is_empty() {
                        // Switch mode from stack description to move instructions processing
                        stack_collecting = false;
                    } else {
                        stack_data.push(instruction);
                    }
                } else {
                    // now the moves can begin - read the moves from the input file
                    //    move 1 from 2 to 1
                    // split on space -> [move, 1, from, 2, to, 1]
                    // relevant instructions are on inx [1,3,5]
                    // convert to usize
                    // : number to pop
                    // : index of the source vector + 1
                    // : index of the target vector + 1
                    let splitvec: Vec<usize> = instruction
                        .split(' ')
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    move_data.push(splitvec);
                }
            }
        }

        // The number at the base of the stack pinpoints the column where the
        // crate identifier is. Follow the columns to create vectors for each
        // stack of crates.

        let indices = stack_data.last().unwrap();
        let boop: Vec<usize> = indices
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if !c.is_whitespace() {
                    return i;
                }
                return 0;
            })
            .filter(|&i| i != 0)
            .collect();

        // boop is a vector containing the columns we need to check to pull
        // out the crate labels from the right place in the loaded strings
        // to create the vectors.
        //
        // In the test data, the columns are [1,5,9]. Three stacks then, in
        // a vector themselves, in order.

        for _ in 1..=boop.len() {
            stacks.push(Vec::new());
        }

        let mut extract_keys = true;
        for stackline in stack_data.iter().rev() {
            if extract_keys {
                extract_keys = false;
                continue;
            }
            for (inx, pos) in boop.iter().enumerate() {
                let target_char = stackline.chars().nth(*pos).unwrap();
                if !target_char.is_whitespace() {
                    stacks[inx].push(target_char);
                }
            }
        }

        // from the test data, the above will make:  [['Z', 'N'], ['M', 'C', 'D'], ['P']]
    }

    // At last we can now do the processing sigh
    for mov in move_data {
        let final_len = stacks[mov[1] - 1].len() - mov[0];
        let mut els = stacks[mov[1] - 1].split_off(final_len);

        stacks[mov[2] - 1].append(&mut els);
    }

    let result: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    println!("result: {:?}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
