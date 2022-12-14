pub fn input_to_sums(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| s.split("\n").map(|s| s.parse::<u32>().unwrap_or(0)).sum())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_calories = input_to_sums(input);
    elf_calories.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_calories = input_to_sums(input);
    elf_calories.sort_unstable();
    match elf_calories.len() {
        0 => None,
        n => Some(elf_calories[n - 3..].iter().sum()),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        println!("{:?}", part_one(&input));
        assert_eq!(part_one(&input), Some(100));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
