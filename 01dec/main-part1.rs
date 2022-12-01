use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("December 01 / 1");
    let mut most_calories = 0;
    if let Ok(lines) = read_lines("./sampledata.txt") {
    let mut calories_count = 0;
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    if calories_count > most_calories {
                        most_calories = calories_count;
                    }
                calories_count = 0;
                } else {
                  calories_count = calories_count + calories.parse::<i32>().unwrap();
                }
            }
        }
        println!("{}", most_calories);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}