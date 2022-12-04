use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  println!("December 02 / 1");

  let mut total_score = 0;
  if let Ok(lines) = read_lines("./input.txt") { // 
    for line in lines {
      if let Ok(strat_str) = line {
        let mut round_score = 0;
        match strat_str.as_str() {
          "A X" | "B Y" | "C Z" => round_score += 3,
          "A Y" | "B Z" | "C X" => round_score += 6,
          _ => (),
        }
        match strat_str.chars().last().unwrap() {
          'X' => round_score += 1,
          'Y' => round_score += 2,
          'Z' => round_score += 3,
          _ => (),
        }
        total_score += round_score
      }
    }
  }

  println!("Total score = {}", total_score);
}

/* change this to take an iterator as a parameter so can use it with lines or string */


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;  // io::Result<File>
    Ok(io::BufReader::new(file).lines()) 
}