use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

enum Operation {
    Noop,
    Addx(i32),
}

impl Operation {
    fn cost(&self) -> i32 {
        match self {
            Operation::Noop => 1,
            Operation::Addx(_) => 2,
        }
    }
}

struct Device {
    operations: Vec<Operation>,
    current_cycle: i32,

    x: i32,
    
    display: HashMap<i32, String>,
}

impl Device {
    fn draw(&self) {
        println!("99999999999999999999999999999999999999999999999999999999999999999999999999999999");

        for index in 0..240 {
            if index != 0 && index % 40 == 0 {
                println!();
            }

            print!("{}", self.display.get(&index)
                .unwrap_or(&".".to_string())
            );
        }

        println!();
    }

    fn execute(&mut self) -> i32 {
        let mut strength: i32 = 0;
        for operation in &self.operations {
            for _ in 0..operation.cost() {
                if self.current_cycle == 20 || (self.current_cycle - 20) % 40 == 0 {
                    // println!("Strength at cycle [{}]: {}", self.current_cycle, self.current_cycle * self.x);
                    strength += self.current_cycle * self.x;
                }

                let column = (self.current_cycle - 1) % 40;
                if column == self.x 
                    || column == self.x - 1
                    || column == self.x + 1 {
                        self.display.insert(self.current_cycle - 1, "#".to_string());
                    }



                self.current_cycle += 1;
            }

            match operation {
                Operation::Noop => {}
                Operation::Addx(value) => {
                    self.x += *value;
                }
            }

        }

        self.draw();

        // println!("Total strength: {}, last_cycle: {}", strength, self.current_cycle);
        return strength;
    }
}

fn parse(filename: &str) -> Vec<Operation> {
    let file = match File::open(filename) {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();

    let op_regex = Regex::new(r"^(?P<op_name>\w+)\s?(?P<first_arg>-?\d+)?.*").unwrap();

    let mut ops = Vec::new();
    for line in reader {
        if let Ok(l) = line {
            let captures = op_regex.captures(&l).unwrap();
            let op_name = captures.name("op_name").unwrap().as_str();
            let first_arg = captures.name("first_arg");

            ops.push(match op_name {
                "noop" => Operation::Noop,
                "addx" => Operation::Addx(first_arg.unwrap().as_str().parse().unwrap()),
                _ => panic!("Unknown op."),
            })
        }
    }

    ops
}

pub fn solve() {
    let operations = parse("inputs/10_input");

    let mut device = Device {
        operations: operations,
        current_cycle: 1,
        x: 1,

        display: HashMap::new()
    };

    device.execute();
}

mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let operations = vec![Operation::Noop, Operation::Addx(3), Operation::Addx(-5)];

        let mut device = Device {
            operations: operations,
            current_cycle: 1,
            x: 1,
            display: HashMap::new()
        };

        device.execute();

        assert_eq!(-1, device.x);
    }
    
    #[test]
    fn base_case() {
        let operations = parse("inputs/10_base");

        let mut device = Device {
            operations: operations,
            current_cycle: 1,
            x: 1,
            display: HashMap::new()
        };

        assert_eq!(13140, device.execute());
    }
}
