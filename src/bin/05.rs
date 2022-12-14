fn starting_piles() -> Vec<Vec<u8>> {
    // Yeah I'm not parsing that
    vec![
        "FCJPHTW".into(),
        "GRVFZJBH".into(),
        "HPTR".into(),
        "ZSNPHT".into(),
        "NVFZHJCD".into(),
        "PMGFWDZ".into(),
        "MVZWSJDP".into(),
        "NDS".into(),
        "DZSFM".into(),
    ]
}

fn move_to(pile: &mut Vec<Vec<u8>>, n_items: usize, src: usize, dst: usize) {
    assert!(pile[src - 1].len() >= n_items);

    for _ in 0..n_items {
        let n = pile[src - 1].pop().unwrap();
        pile[dst - 1].push(n);
    }
}

fn move_chunk(pile: &mut Vec<Vec<u8>>, n_items: usize, src: usize, dst: usize) {
    assert!(pile[src - 1].len() >= n_items);
    let end_idx = pile[src - 1].len() - n_items;
    let mut moved: Vec<_> = pile[src - 1].drain(end_idx..).collect();
    pile[dst - 1].append(&mut moved);
}

pub fn part_one(input: &str) -> Option<String> {
    let mut sp = starting_piles();
    for s in input.split("\n") {
        let ns: Vec<_> = s
            .split_whitespace()
            .map(|d| d.parse::<usize>().unwrap())
            .collect();
        assert_eq!(ns.len(), 3);
        move_to(&mut sp, ns[0], ns[1], ns[2]);
    }

    let mut tops = String::new();
    for stack in sp {
        tops.push(*stack.last().unwrap() as char);
    }
    Some(tops)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut sp = starting_piles();
    for s in input.split("\n") {
        let ns: Vec<_> = s
            .split_whitespace()
            .map(|d| d.parse::<usize>().unwrap())
            .collect();
        assert_eq!(ns.len(), 3);
        move_chunk(&mut sp, ns[0], ns[1], ns[2]);
    }

    let mut tops = String::new();
    for stack in sp {
        tops.push(*stack.last().unwrap() as char);
    }
    Some(tops)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
