use std::collections::HashSet;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn new(s: &str) -> Self {
        match s {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            s => panic!("Unknown command: {}", s),
        }
    }
}

struct Rope {
    knots: Vec<(i64, i64)>,
    tail_visited: HashSet<(i64, i64)>,
}

impl Rope {
    fn new(n: usize) -> Self {
        let mut r = Rope {
            knots: vec![(0, 0); n],
            tail_visited: HashSet::new(),
        };
        r.tail_visited.insert((0, 0));
        r
    }

    fn move_n(&mut self, dir: Dir, n: u32) {
        for _ in 0..n {
            self.move1(&dir);
        }
    }

    fn move1(&mut self, dir: &Dir) {
        match dir {
            Dir::Up => self.knots[0].0 += 1,
            Dir::Down => self.knots[0].0 -= 1,
            Dir::Left => self.knots[0].1 -= 1,
            Dir::Right => self.knots[0].1 += 1,
        };
        self.update_tail();
    }

    fn update_tail(&mut self) {
        for i in 0..self.knots.len() - 1 {
            let (hi, hj) = self.knots[i];
            let (ti, tj) = &mut self.knots[i + 1];
            let should_move = {
                let xd = hi.abs_diff(*ti);
                let yd = hj.abs_diff(*tj);
                (xd + yd > 2) || (xd > 1) || (yd > 1)
            };

            if should_move {
                if hi > *ti {
                    *ti += 1;
                } else if hi < *ti {
                    *ti -= 1;
                }

                if hj > *tj {
                    *tj += 1;
                } else if hj < *tj {
                    *tj -= 1;
                }
            }
        }
        self.tail_visited.insert(*self.knots.last().unwrap());
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut r = Rope::new(2);
    for cmd in input.split("\n") {
        let l = cmd.split_whitespace().collect::<Vec<_>>();
        let d = Dir::new(l[0]);
        let n = l.last().unwrap().parse::<u32>().unwrap();
        r.move_n(d, n);
    }
    Some(r.tail_visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut r = Rope::new(10);
    for cmd in input.split("\n") {
        let l = cmd.split_whitespace().collect::<Vec<_>>();
        let d = Dir::new(l[0]);
        let n = l.last().unwrap().parse::<u32>().unwrap();
        r.move_n(d, n);
    }
    Some(r.tail_visited.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
