struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(s: &str) -> Self {
        let start_end: Vec<_> = s.split('-').map(|n| n.parse::<i32>().unwrap()).collect();
        assert_eq!(start_end.len(), 2);
        Self {
            start: start_end[0],
            end: start_end[1],
        }
    }

    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        if self.start <= other.start {
            other.start <= self.end
        } else {
            other.end >= self.start
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut count = 0;
    for s in input.split("\n") {
        let ranges: Vec<_> = s.split(",").map(Range::new).collect();
        assert_eq!(ranges.len(), 2);
        if ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for s in input.split("\n") {
        let ranges: Vec<_> = s.split(",").map(Range::new).collect();
        assert_eq!(ranges.len(), 2);
        if ranges[0].overlaps(&ranges[1]) {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
