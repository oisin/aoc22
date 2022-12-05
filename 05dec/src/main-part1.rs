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

        //println!("indexes are {:?}", boop);

        // boop is a vector containing the columns we need to check to pull
        // out the crate labels from the right place in the loaded strings 
        // to create the vectors.
        //
        // In the test data, the columns are [1,5,9]. Three stacks then, in
        // a vector themselves, in order. 

        println!("{} columns: {:?}", boop.len(), boop);

        let mut stacks: Vec<Vec<char>> = Vec::new();

        for _ in 1..=boop.len() {
            stacks.push(Vec::new());
        }

        println!("stacks vec: {:?}", stacks);

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
        // now the moves can begin - read the moves from the input file
        //    move 1 from 2 to 1
        // split on space -> [move, 1, from, 2, to, 1]
        // relevant instructions are on inx [1,3,5]
        // convert to usize
        // : number to pop 
        // : index of the source vector + 1
        // : index of the target vector + 1

        

        println!("hash loks like: {:?}", stacks)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
