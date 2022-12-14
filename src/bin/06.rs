use std::collections::BTreeSet;

pub fn detect_unique_window(input: &str, window_size: usize) -> Option<usize> {
    for (i, w) in input.as_bytes().windows(window_size).enumerate() {
        let mut set = BTreeSet::new();
        for c in w {
            set.insert(*c);
        }
        if set.len() == window_size {
            return Some(i + window_size);
        }
    }
    None
}
pub fn part_one(input: &str) -> Option<usize> {
    detect_unique_window(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    detect_unique_window(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: String = advent_of_code::read_file("examples", 6);
        for s in input.split("\n") {
            let k: Vec<_> = s.split_whitespace().collect();
            assert_eq!(k.len(), 2);
            let i = k[1].parse::<usize>().unwrap();
            assert_eq!(part_one(k[0]), Some(i));
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
