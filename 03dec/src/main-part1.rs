use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  println!("December 03 / 1");

  let mut total_score = 0;
  if let Ok(lines) = read_lines("./test_input_part1.txt") { // 
    for line in lines {
      if let Ok(strat_str) = line {
        // Split string in half
        // find the common character
        let len = strat_str.chars().count();  
        let (lcompart, rcompart) = strat_str.split_at(len/2);

        let shared_char = find_shared_char(lcompart, rcompart);
        total_score += priority_for_char(shared_char);
      }
    }
    println!("Total priority is {}", total_score);
  }
}

fn find_shared_char(s1: String, s2: String) -> char {
  println!("{} => {}", s1, s2);

  // Find the common parts in the lcompart / rcompart
  // Iterate thru lcompart char by char and see if rcompart has it
  // This code will find multiple instances. 
  return s1
    .chars()
    .filter(|&ch| { s2.contains(ch) })
    .collect()
    .first();
}

fn priority_for_char(ch: char) -> usize {
  static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
  return ALPHABET.chars().position(|c| c == ch).unwrap() + 1;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;  // io::Result<File>
    Ok(io::BufReader::new(file).lines()) 
}