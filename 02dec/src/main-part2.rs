use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  println!("December 02 / 1");

  let mut total_score = 0;
  if let Ok(lines) = read_lines("./input.txt") { 
    for line in lines {
      if let Ok(strat_str) = line {
        let mut round_score = 0;
        let opponent_play = strat_str.chars().next().unwrap();
        let outcome = strat_str.chars().last().unwrap();
        let my_play;
        match outcome {
          'X' => { my_play = lose_play(opponent_play)  },
          'Y' => { round_score += 3; my_play = opponent_play }, 
          'Z' => { round_score += 6; my_play = win_play(opponent_play) } , 
          _ => my_play = ' ',
        }

        match my_play {
          'A' => round_score += 1,
          'B' => round_score += 2,
          'C' => round_score += 3,
          _ => (),
        }
        
        total_score += round_score
      }
    }
  }

  println!("Total score = {}", total_score);
}

fn lose_play(play: char) -> char {
  return match play {
    'A' => 'C',
    'B' => 'A',
    'C' => 'B',
    _ => ' ',
  }
}

fn win_play(play: char) -> char {
  return match play {
    'A' => 'B',
    'B' => 'C',
    'C' => 'A',
    _ => ' ',
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;  // io::Result<File>
    Ok(io::BufReader::new(file).lines()) 
}