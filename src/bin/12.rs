use std::collections::{HashSet, VecDeque};

fn read(input: &str) -> (Vec<Vec<u32>>, (usize, usize), (usize, usize)) {
    let mut h: Vec<Vec<u32>> = Vec::new();
    let mut start_pos = (0, 0);
    let mut goal_pos = (0, 0);
    let offset = 'a' as u32;

    for (i, s) in input.split('\n').enumerate() {
        h.push(Vec::new());
        for (j, c) in s.as_bytes().iter().enumerate() {
            let c = match *c as char {
                'S' => {
                    start_pos = (i, j);
                    'a'
                }
                'E' => {
                    goal_pos = (i, j);
                    'z'
                }
                s => s,
            };
            h.last_mut().unwrap().push(c as u32 - offset);
        }
    }

    (h, start_pos, goal_pos)
}

fn bfs_path(h: &Vec<Vec<u32>>, start_pos: (usize, usize), goal_pos: (usize, usize)) -> Option<usize> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut bfs: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    bfs.push_back((0, start_pos));
    visited.insert(start_pos);

    let get_neighbors = |pos: (usize, usize)| {
        let (i, j) = pos;
        let mut neighbors: Vec<(usize, usize)> = vec![];
        if i > 0 {
            neighbors.push((i - 1, j));
        }
        if j > 0 {
            neighbors.push((i, j - 1));
        }
        if i < h.len() - 1 {
            neighbors.push((i + 1, j));
        }
        if j < h[0].len() - 1 {
            neighbors.push((i, j + 1));
        }
        neighbors
    };

    while !bfs.is_empty() {
        let pos = bfs.pop_front().unwrap();
        let (steps, (i, j)) = pos; 
        if (i, j) == goal_pos {
            return Some(steps);
        }

        for n in get_neighbors(pos.1) {
            if !visited.contains(&n) && h[n.0][n.1] <= h[i][j] + 1 {
                visited.insert(n);
                bfs.push_back((steps + 1, n));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let (h, start_pos, goal_pos) = read(input);
    bfs_path(&h, start_pos, goal_pos)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (h, _, goal_pos) = read(input);
    let mut paths: Vec<usize> = vec![];
    for i in 0..h.len() {
        for j in 0..h[0].len() {
            if h[i][j] == 0 {
                match bfs_path(&h, (i, j), goal_pos) {
                    Some(p) => paths.push(p),
                    None => ()
                }
            }
        }
    }
    paths.iter().min().copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
