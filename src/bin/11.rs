use std::collections::VecDeque;

struct Monkey {
    id: usize,
    items: VecDeque<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> usize>,
}

const k: u128 = 2 * 7 * 3 * 11 * 17 * 5 * 13 * 19;

fn get_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            id: 0,
            items: vec![54, 53].into(),
            operation: Box::new(|n| n * 3),
            test: Box::new(|n| if n % 2 == 0 { 2 } else { 6 }),
        },
        Monkey {
            id: 1,
            items: vec![95, 88, 75, 81, 91, 67, 65, 84].into(),
            operation: Box::new(|n| n * 11),
            test: Box::new(|n| if n % 7 == 0 { 3 } else { 4 }),
        },
        Monkey {
            id: 2,
            items: vec![76, 81, 50, 93, 96, 81, 83].into(),
            operation: Box::new(|n| n + 6),
            test: Box::new(|n| if n % 3 == 0 { 5 } else { 1 }),
        },
        Monkey {
            id: 3,
            items: vec![83, 85, 85, 63].into(),
            operation: Box::new(|n| n + 4),
            test: Box::new(|n| if n % 11 == 0 { 7 } else { 4 }),
        },
        Monkey {
            id: 4,
            items: vec![85, 52, 64].into(),
            operation: Box::new(|n| n + 8),
            test: Box::new(|n| if n % 17 == 0 { 0 } else { 7 }),
        },
        Monkey {
            id: 5,
            items: vec![57].into(),
            operation: Box::new(|n| n + 2),
            test: Box::new(|n| if n % 5 == 0 { 1 } else { 3 }),
        },
        Monkey {
            id: 6,
            items: vec![60, 95, 76, 66, 91].into(),
            operation: Box::new(|n| n * n),
            test: Box::new(|n| if n % 13 == 0 { 2 } else { 5 }),
        },
        Monkey {
            id: 7,
            items: vec![65, 84, 76, 72, 79, 65].into(),
            operation: Box::new(|n| n + 5),
            test: Box::new(|n| if n % 19 == 0 { 6 } else { 0 }),
        },
    ]
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = get_monkeys();
    let mut activity: Vec<u64> = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().unwrap();
                item = (monkeys[i].operation)(item) / 3;
                item %= k;
                let dest = (monkeys[i].test)(item);
                monkeys[dest].items.push_back(item);

                activity[i] += 1;
            }
        }
    }

    activity.sort();
    Some(activity.iter().rev().take(2).product())
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut monkeys = get_monkeys();
    let mut activity: Vec<u128> = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop_front().unwrap();
                item = (monkeys[i].operation)(item);
                item %= k;
                let dest = (monkeys[i].test)(item);
                monkeys[dest].items.push_back(item);

                activity[i] += 1;
            }
        }
    }

    activity.sort();
    Some(activity.iter().rev().take(2).product())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
