use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("December 04 / 1");
  
    let mut contained_ranges = 0;
    if let Ok(lines) = read_lines("./input.txt") {  
      for line in lines {
        if let Ok(assignments_str) = line {

            let r: Vec<usize> =
                assignments_str
                .split(',')
                .map(|s| {s.split('-').collect::<Vec<&str>>() })
                .flatten()
                .map(|s| { s.parse().unwrap() })
                .collect();

            if is_contained(r[0], r[1], r[2], r[3]) {
                contained_ranges += 1;
            }
        }
      }
    }
    println!("Total contained_ranges = {}", contained_ranges);
  }

fn is_contained(x1: usize, x2: usize, y1: usize, y2: usize) -> bool {
    return (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2);
} 

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?; 
    Ok(io::BufReader::new(file).lines()) 
}
