//Change the approach here for  this one and put more structure
// into the picture. Move to the input_str used by other people and
// away from the file reader based approach
// put in tests and run it from cargo
// have a single binary as well that will do part1 and part2 as separate functions
use std::collections::HashSet;
use std::iter::FromIterator;

static INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_n(INPUT_TXT, 4));
    println!("Part 2: {}", part_n(INPUT_TXT, 14));
}

fn part_n(input: &str, marker_size: usize) -> usize {
    let mut window_count = 0;
    let mut set: HashSet<&char>;

    for window in input.chars().collect::<Vec<char>>().windows(marker_size) {
        set = HashSet::from_iter(window.iter());
        if set.len() == marker_size {
            break;
        } else {
            window_count += 1;
            set.clear();
        }
    }
    return window_count + marker_size;
}

#[cfg(test)]
mod day_6_tests {
    use super::*;

    #[test]
    fn test_part_n_1() {
        static TEST: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_n(TEST, 4), 7);
        assert_eq!(part_n(TEST, 14), 19);
    }

    #[test]
    fn test_part_n_2() {
        static TEST: &str = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_n(TEST, 4), 6);
        assert_eq!(part_n(TEST, 14), 23);
    }

    #[test]
    fn test_part_n_3() {
        static TEST: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_n(TEST, 4), 5);
        assert_eq!(part_n(TEST, 14), 23);
    }

    #[test]
    fn test_part_n_4() {
        static TEST: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_n(TEST, 4), 10);
        assert_eq!(part_n(TEST, 14), 29);
    }

    #[test]
    fn test_part_n_5() {
        static TEST: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_n(TEST, 4), 11);
        assert_eq!(part_n(TEST, 14), 26);
    }
}
