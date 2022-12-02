use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Tactic {
    Lose,
    Win,
    Draw
}

impl Move {
    fn from(name: &str) -> Move {
        return match name {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Unknown move {}", name),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}

impl Tactic {
    fn from(name: &str) -> Tactic {
        return match name {
            "X" => Tactic::Lose,
            "Y" => Tactic::Draw,
            "Z" => Tactic::Win,
            _ => panic!("Unkown tactic: {}", name)
        }
    }

    fn determine_move(&self, opponent: &Move) -> Move {
        return match self {
            Tactic::Draw => *opponent,
            Tactic::Win => match opponent {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Tactic::Lose => match opponent {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }
}

enum Outcome {
    Draw,
    Win,
    Lose
}

impl Outcome {
    fn calculate(opponent: &Move, me: &Move) -> Outcome {
        return match opponent {
            Move::Rock => match me {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Lose,
            },
            Move::Scissors => match me {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Draw,
            },
            Move::Paper => match me {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Win,
            }
        }
    }

    fn value(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }
}

fn part1(decisions: &Vec<&str>) -> u32 {
    let opponent = Move::from(decisions.get(0).unwrap());
    let me = Move::from(decisions.get(1).unwrap());
    return Outcome::calculate(&opponent, &me).value() + me.value();
}

fn part2(decisions: &Vec<&str>) -> u32 {
    let opponent = Move::from(decisions.get(0).unwrap());
    let me = Tactic::from(decisions.get(1).unwrap());
    let my_move = me.determine_move(&opponent);
    return Outcome::calculate(&opponent, &my_move).value() + my_move.value()
}


fn main() {
    let file = File::open("2_input").unwrap();

    let mut total_score: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(tactic) = line {
            if tactic == "" {
                continue;
            }

            let decisions: Vec<&str> = tactic.split(" ").collect();
            total_score += part2(&decisions)
        }
    }

    println!("Total score: {}", total_score);
}