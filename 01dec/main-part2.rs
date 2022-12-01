use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("December 01 / 2");
    let mut most_calories: [i32; 3] = [0; 3];

    if let Ok(lines) = read_lines("./sampledata.txt") {
        let mut calories_count = 0;
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    pick_top_three (&mut most_calories,  calories_count);
                    calories_count = 0;
                } else {
                  calories_count = calories_count + calories.parse::<i32>().unwrap();
                }
            }
        }
    }

    let sum: i32 = most_calories.iter().sum();
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn pick_top_three(three: &mut [i32; 3], other: i32) {
    if other > three[0] {
        three[2] = three[1];
        three[1] = three[0];
        three[0] = other;
    } else if other > three[1] {
        three[2] = three[1];
        three[1] = other;
    } else if other > three[2] {
        three[2] = other;
    } 
}