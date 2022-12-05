use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::{VecDeque};

struct Move {
    count: u32,
    from: usize,
    to: usize
}



impl Move {
    fn parse(line: &str) -> Move {
        let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let cap = re.captures(line).unwrap();

        return Move {
            count: cap.name("count").unwrap().as_str().parse().unwrap(),
            from: cap.name("from").unwrap().as_str().parse().unwrap(),
            to: cap.name("to").unwrap().as_str().parse().unwrap(),
        }
    }
}

fn parse(reader: io::Lines<io::BufReader<File>>) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let mut moves = Vec::new();
    let mut crates: Vec<VecDeque<char>> = vec![VecDeque::new()];
    
    let mut parsing_moves = false;

    for line in reader {
        if let Ok(entry) = line {
            if entry == "" {
                parsing_moves = true;
                continue;
            }

            if parsing_moves {
                println!("parsing moves {}", entry);
                moves.push(Move::parse(&entry))
            } else {
                let mut start_crate = false;
                let mut crate_idx = 0;

                for (i, token) in entry.chars().enumerate() {
                    if (i + 1) % 4 == 0 {
                        crate_idx += 1;
                    }

                    if start_crate && token != ']' {
                        println!("token: {}", token);
                        while crate_idx >= crates.len() {
                            crates.push(VecDeque::new());
                            println!("new crate! {}", crate_idx);
                        }
                        match crates.get_mut(crate_idx) {
                            Some(c) => {
                                c.push_back(token)
                            },
                            None => {
                                panic!("shouldn't happen")
                            }
                        }
                    } else if token == '[' {
                        start_crate = true;
                    } else if token == ']' {
                        start_crate = false;
                    }
                }
            }
        }
    };

    return (crates, moves)
}


pub fn part1() {
    let file = match File::open("inputs/5_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file
    };

    let reader = io::BufReader::new(file).lines();


    let (mut crates, moves) = parse(reader);

    for m in moves {
        for _ in 0..m.count {
            let from = crates.get_mut(m.from - 1).unwrap();
            let token = from.pop_front().unwrap();
            let to = crates.get_mut(m.to - 1).unwrap();

            to.push_front(token);
        }
    }

    for mut c in crates {
        let token = c.pop_front().unwrap();
        print!("{}", token);
    }
}


pub fn part2() {
    let file = match File::open("inputs/5_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file
    };

    let reader = io::BufReader::new(file).lines();


    let (mut crates, moves) = parse(reader);

    for m in moves {
        let mut tokens = VecDeque::new();
        for _ in 0..m.count {
            let from = crates.get_mut(m.from - 1).unwrap();
            let token = from.pop_front().unwrap();

            tokens.push_front(token);
        }

        let to = crates.get_mut(m.to - 1).unwrap();
        for token in tokens {
            to.push_front(token);
        }
    }

    for mut c in crates {
        let token = c.pop_front().unwrap();
        print!("{}", token);
    }
}