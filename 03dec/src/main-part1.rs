use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  println!("December 03 / 1");

  let mut total_score = 0;
  if let Ok(lines) = read_lines("./input.txt") { // 
    for line in lines {
      if let Ok(strat_str) = line {
        // Split string in half
        // find the common character
        let len = strat_str.chars().count();  
        let (lcompart, rcompart) = strat_str.split_at(len/2);

        println!("{} => {}", lcompart, rcompart);

        // Find the common parts in the lcompart / rcompart
        // Iterate thru lcompart char by char and see if rcompart has it
        // This code will find multiple instances. 
        let shared_chars: Vec<char> = lcompart.chars()
          .filter(|&ch| { rcompart.contains(ch) })
          .collect();

        static LCASE: &str = "abcdefghijklmnopqrstuvwxyz";
        static UCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let priority;
        if shared_chars[0].is_uppercase() {
          priority = UCASE.chars().position(|c| c == shared_chars[0]).unwrap() + 27;
        } else {
          priority = LCASE.chars().position(|c| c == shared_chars[0]).unwrap() + 1;
        }

        total_score += priority;
      }
    }
    println!("Total priority is {}", total_score);
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;  // io::Result<File>
    Ok(io::BufReader::new(file).lines()) 
}