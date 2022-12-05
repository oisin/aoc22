use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  println!("December 03 / 2");

  let mut total_score = 0;
  let mut group: [String; 3] = [String::new(), String::new(), String::new()] ;

  if let Ok(lines) = read_lines("./input.txt") { 
    let mut counter = 0;
    for line in lines {
      group[counter] = line.expect("cannot read line from file");
      counter += 1;
      if counter == 3 {
        counter = 0;
        let shared = find_shared(&group[0], &group[1]);
        let all_shared = find_shared(&shared, &group[2]);
        total_score += priority_for_char(all_shared.chars().next().unwrap());
      }
    }
  }
  println!("Total priority is {}", total_score);
}

fn find_shared(s1: &str, s2: &str) -> String {
  return s1
    .chars()
    .filter(|&ch| { s2.contains(ch) })
    .collect::<Vec<_>>()
    .into_iter()
    .collect();
}

fn priority_for_char(ch: char) -> usize {
  static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
  return ALPHABET.chars().position(|c| c == ch).unwrap() + 1;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;  // io::Result<File>
    Ok(io::BufReader::new(file).lines()) 
}