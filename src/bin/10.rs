#[derive(Debug, Clone, Copy)]
enum Instr {
    NOP,
    Addx(i64),
}

#[derive(Debug)]
struct CPU {
    x: i64,
    record: Vec<i64>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            x: 1,
            record: vec![1, 1],
        }
    }

    fn run(&mut self, i: Instr) {
        match i {
            Instr::NOP => {
                self.record.push(self.x);
            }
            Instr::Addx(x) => {
                self.record.push(self.x);
                self.x += x;
                self.record.push(self.x);
            }
        }
    }
}

fn display(r: &Vec<i64>) {
    let mut s: String = "".into();
    for i in 0..r.len() - 1 {
        if r[i + 1].abs_diff((i % 40) as i64) <= 1 {
            s.push('#');
        } else {
            s.push('.');
        }
        if i % 40 == 0 {
            s.push('\n');
        }
    }
    println!("{}", s);
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut cpu = CPU::new();
    for line in input.split("\n") {
        let instr = {
            if line.starts_with("addx") {
                let r = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                Instr::Addx(r)
            } else {
                Instr::NOP
            }
        };
        cpu.run(instr);
    }
    let mut total = 0;
    for s in [20, 60, 100, 140, 180, 220] {
        total += s as i64 * cpu.record[s];
    }
    display(&cpu.record);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
