use std::collections::HashSet;

fn letter_score(s: char) -> i32 {
    if !s.is_alphabetic() {
        panic!("Invalid letter: {}", s);
    }
    if s.is_lowercase() {
        (s as i32) - ('a' as i32) + 1
    } else if s.is_uppercase() {
        (s as i32) - ('A' as i32) + 27
    } else {
        panic!("Invalid contents: {}", s);
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .split("\n")
            .map(|s| {
                let mid_idx = (s.len() - 1) / 2;
                let first_half = s[..=mid_idx].as_bytes();
                let second_half = s[mid_idx + 1..].as_bytes();

                let pouch1_contents: HashSet<&u8> = HashSet::from_iter(first_half.iter());
                for c in second_half {
                    if pouch1_contents.contains(c) {
                        return letter_score(*c as char);
                    }
                }
                panic!("Malformed rucksack")
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut score = 0;
    let sacks: Vec<&str> = input.split("\n").collect();
    for i in (0..sacks.len()).step_by(3) {
        let c1: HashSet<&u8> = HashSet::from_iter(sacks[i].as_bytes().iter());
        let c2: HashSet<&u8> = HashSet::from_iter(sacks[i + 1].as_bytes().iter());
        for c in sacks[i + 2].as_bytes() {
            if c1.contains(c) && c2.contains(c) {
                score += letter_score(*c as char);
                break;
            }
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
