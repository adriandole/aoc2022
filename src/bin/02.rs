#[derive(Debug, PartialEq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn to_rps(input: &str) -> RPS {
    match input {
        "A" | "X" => RPS::Rock,
        "B" | "Y" => RPS::Paper,
        "C" | "Z" => RPS::Scissors,
        s => panic!("Invalid input: {}", s),
    }
}

fn game_score(you: &RPS, other: &RPS) -> u32 {
    let mut score = match you {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    enum Outcome {
        Win,
        Draw,
        Loss,
    }

    let game_outcome = {
        if you == other {
            Outcome::Draw
        } else {
            match (you, other) {
                (RPS::Rock, RPS::Scissors) => Outcome::Win,
                (RPS::Paper, RPS::Rock) => Outcome::Win,
                (RPS::Scissors, RPS::Paper) => Outcome::Win,
                _ => Outcome::Loss,
            }
        }
    };

    score += match game_outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Loss => 0,
    };
    score
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|s| {
                let plays: Vec<RPS> = s.split_ascii_whitespace().map(to_rps).collect();
                assert_eq!(plays.len(), 2);
                game_score(&plays[1], &plays[0])
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|s| {
                let plays: Vec<&str> = s.split_ascii_whitespace().collect();
                let them = to_rps(plays[0]);
                let you = match plays[1] {
                    "X" => match &them {
                        RPS::Rock => RPS::Scissors,
                        RPS::Paper => RPS::Rock,
                        RPS::Scissors => RPS::Paper,
                    },
                    "Y" => them.clone(),
                    "Z" => match &them {
                        RPS::Rock => RPS::Paper,
                        RPS::Paper => RPS::Scissors,
                        RPS::Scissors => RPS::Rock,
                    },
                    s => panic!("Invalid move: {}", s),
                };
                game_score(&you, &them)
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
