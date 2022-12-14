use std::cmp;

fn read(input: &str) -> Vec<Vec<u32>> {
    let mut v: Vec<Vec<_>> = vec![];
    for line in input.split("\n") {
        v.push(
            line.as_bytes()
                .iter()
                .map(|d| (*d as char).to_digit(10).unwrap())
                .collect(),
        );
    }
    v
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees = read(input);
    let mut seen: Vec<Vec<bool>> = vec![vec![false; trees[0].len()]; trees.len()];
    for i in 0..trees.len() {
        seen[i][0] = true;
        *seen[i].last_mut().unwrap() = true;

        let mut max = 0;
        for j in 0..trees[0].len() {
            if trees[i][j] > max {
                max = trees[i][j];
                seen[i][j] = true;
            }
        }

        max = 0;
        for j in (0..trees[0].len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                seen[i][j] = true;
            }
        }
    }

    for j in 0..trees[0].len() {
        seen[0][j] = true;
        seen.last_mut().unwrap()[j] = true;

        let mut max = 0;
        for i in 0..trees.len() {
            if trees[i][j] > max {
                max = trees[i][j];
                seen[i][j] = true;
            }
        }

        max = 0;
        for i in (0..trees.len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                seen[i][j] = true;
            }
        }
    }
    Some(
        seen.iter()
            .map(|r| r.iter().filter(|e| **e).count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees = read(input);
    let rows = trees.len() as i32;
    let cols = trees[0].len() as i32;

    let tree_score = |mut i: i32, mut j: i32| -> u32 {
        let in_bounds = |i: i32, j: i32| i >= 0 && j >= 0 && i < rows && j < cols;

        let mut total_score = 1;
        let i0 = i;
        let j0 = j;

        enum Dir {
            Up,
            Down,
            Left,
            Right,
        };
        for d in &[Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            j = j0;
            i = i0;

            let mut score = 0;
            loop {
                match d {
                    Dir::Up => j += 1,
                    Dir::Down => j -= 1,
                    Dir::Left => i -= 1,
                    Dir::Right => i += 1,
                };
                if in_bounds(i, j) {
                    score += 1;
                    if trees[i as usize][j as usize] >= trees[i0 as usize][j0 as usize] {
                        break;
                    }
                } else {
                    break;
                }
            }
            total_score *= score;
        }
        total_score
    };

    let mut max_score = 0;
    for i in 0..rows {
        for j in 0..cols {
            max_score = cmp::max(max_score, tree_score(i, j));
        }
    }
    Some(max_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
