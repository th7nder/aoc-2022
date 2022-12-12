use std::{
    collections::{HashMap, VecDeque}, borrow::Borrow
};
use num_bigint::BigUint;

macro_rules! vecdeque {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(vecdeque!(@single $rest)),*]));

    ($($value:expr,)+) => { vecdeque!($($value),+) };
    ($($value:expr),*) => {
        {
            let _cap = vecdeque!(@count $($value),*);
            let mut _map = ::std::collections::VecDeque::with_capacity(_cap);
            $(
                _map.push_back($value);
            )*
            _map
        }
    };
    ($value:expr;$count:expr) => {
        {
            let c = $count;
            let mut _map = ::std::collections::VecDeque::with_capacity(c);
            for _ in 0..c {
                _map.push_back($value);
            }
            _map
        }
    };
}

#[derive(Debug)]
enum Value {
    Old,
    Simple(u128),
}

impl Value {
    fn from(v: u128) -> Value {
        Value::Simple(v)
    }
}

#[derive(Debug)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    left: Value,
    right: Value,
}

// ((79 * 19) + 3 + 6) * 19
// ((79 * 19 * 19) + 9 * 19) 

// how to check if its divisible?
// ((80 * 19 * 19) + 171) * 1/2 
// coefs contain this num, and sum is divisible

#[derive(Debug)]
struct Item {
    worry_level: u128,
    worries: HashMap<u128, u128>,
    name: String,
}

impl Item {
    fn new(worry_level: u128) -> Item {
        Item {
            worry_level,
            name: "".to_string(),
            worries: HashMap::new(),
        }
    }

    fn new_named(worry_level: u128, name: &str) -> Item {
        Item {
            worry_level: worry_level,
            worries: HashMap::new(),
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: u128,
    target_true: usize,
    target_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    inspections: u128,
}

fn round(monkeys: &mut Vec<Monkey>, very_worried: bool) {
    let mut receive_queue: HashMap<usize, VecDeque<Item>> = HashMap::new();

    let divisors = monkeys.iter()
        .map(|m| m.test.divisible_by)
        .collect::<Vec<u128>>();

    // hardcoding 10, because why not
    for idx in 0..10 {
        receive_queue.insert(idx, VecDeque::new());
    }

    for (idx, mut monkey) in monkeys.iter_mut().enumerate() {
        if let Some(queue) = receive_queue.get_mut(&idx) {
            while let Some(item) = queue.pop_front() {
                monkey.items.push_back(item);
            }
        }

        while let Some(mut item) = monkey.items.pop_front() {
            monkey.inspections += 1;

            let divisible: bool;
            if !very_worried {
                let left_value = match &monkey.operation.left {
                    Value::Old => &item.worry_level,
                    Value::Simple(v) => &v,
                };
                let right_value = match &monkey.operation.right {
                    Value::Old => &item.worry_level,
                    Value::Simple(v) => &v,
                };
    
                item.worry_level = match monkey.operation.operator {
                    Operator::Multiply => left_value * right_value,
                    Operator::Add => left_value + right_value,
                };
                item.worry_level /= 3u128;

                let remainder = &item.worry_level % monkey.test.divisible_by;
                divisible = remainder == 0
            } else {
                match &monkey.operation.right {
                    Value::Old => {
                        match monkey.operation.operator {
                            Operator::Multiply => {
                                for divisor in &divisors {
                                    let value = item.worries.get(divisor).unwrap_or(&item.worry_level);
                                    item.worries.insert(*divisor, value * value % divisor);
                                }
                            },
                            Operator::Add => {
                                panic!("sholdn't happen");
                            }
                        };
                    },
                    Value::Simple(v) => {
                        match monkey.operation.operator {
                            Operator::Multiply => {
                                for divisor in &divisors {
                                    let value = item.worries.get(divisor).unwrap_or(&item.worry_level);
                                    item.worries.insert(*divisor, value * v % divisor);
                                }
                            },
                            Operator::Add => {
                                for divisor in &divisors {
                                    let value = item.worries.get(divisor).unwrap_or(&item.worry_level);
                                    item.worries.insert(*divisor, (value + v) % divisor);
                                }
                            }
                        };
                    }
                };

                let remainder = item.worries.get(&monkey.test.divisible_by).unwrap() % monkey.test.divisible_by;

        
                divisible = remainder == 0;
            }

            
            if divisible {
                receive_queue
                    .get_mut(&monkey.test.target_true)
                    .unwrap()
                    .push_back(item);
            } else {
                receive_queue
                    .get_mut(&monkey.test.target_false)
                    .unwrap()
                    .push_back(item);
            }
        }
    }

    for (idx, monkey) in monkeys.iter_mut().enumerate() {
        if let Some(queue) = receive_queue.get_mut(&idx) {
            while let Some(item) = queue.pop_front() {
                monkey.items.push_back(item);
            }
        }
    }
}

fn puzzle_input() -> Vec<Monkey> {
    vec![
            Monkey {
                items: vecdeque![Item::new(85), Item::new(79), Item::new(63), Item::new(72)],
                operation: Operation {
                    operator: Operator::Multiply,
                    left: Value::Old,
                    right: Value::from(17),
                },
                test: Test {
                    divisible_by: 2,
                    target_true: 2,
                    target_false: 6,
                },
                inspections: 0,
            },
            Monkey {
                items: vecdeque![Item::new(53), Item::new(94), Item::new(65), Item::new(81), Item::new(93), Item::new(73), Item::new(57), Item::new(92)],
                operation: Operation {
                    operator: Operator::Multiply,
                    left: Value::Old,
                    right: Value::Old,
                },
                test: Test {
                    divisible_by: 7,
                    target_true: 0,
                    target_false: 2,
                },
                inspections: 0,
                
            },
            Monkey {
                items: vecdeque![Item::new(62), Item::new_named(63,"midlands")],
                operation: Operation {
                    operator: Operator::Add,
                    left: Value::Old,
                    right: Value::from(7),
                },
                test: Test {
                    divisible_by: 13,
                    target_true: 7,
                    target_false: 6,
                },
                inspections: 0,
                
            },
            Monkey {
                items: vecdeque![Item::new(57), Item::new(92), Item::new(56)],
                operation: Operation {
                    operator: Operator::Add,
                    left: Value::Old,
                    right: Value::from(4),
                },
                test: Test {
                    divisible_by: 5,
                    target_true: 4,
                    target_false: 5,
                },
                inspections: 0,
                
            },
            Monkey {
                items: vecdeque![Item::new(67)],
                operation: Operation {
                    operator: Operator::Add,
                    left: Value::Old,
                    right: Value::from(5),
                },
                test: Test {
                    divisible_by: 3,
                    target_true: 1,
                    target_false: 5,
                },
                inspections: 0,
                
            },
            Monkey {
                items: vecdeque![Item::new(85), Item::new(56), Item::new(66), Item::new(72), Item::new(57), Item::new(99)],
                operation: Operation {
                    operator: Operator::Add,
                    left: Value::Old,
                    right: Value::from(6),
                },
                test: Test {
                    divisible_by: 19,
                    target_true: 1,
                    target_false: 0,
                },
                inspections: 0,
                
            },
            Monkey {
                items: vecdeque![Item::new(86), Item::new(65), Item::new(98), Item::new(97), Item::new(69)],
                operation: Operation {
                    operator: Operator::Multiply,
                    left: Value::Old,
                    right: Value::from(13),
                },
                test: Test {
                    divisible_by: 11,
                    target_true: 3,
                    target_false: 7,
                },
                inspections: 0,
                
            },
            Monkey {
                items: vecdeque![Item::new(87), Item::new(68), Item::new(92), Item::new(66), Item::new(91), Item::new(50), Item::new(68)],
                operation: Operation {
                    operator: Operator::Add,
                    left: Value::Old,
                    right: Value::from(2),
                },
                test: Test {
                    divisible_by: 17,
                    target_true: 4,
                    target_false: 3,
                },
                inspections: 0,
                
            },
        ]
}

pub fn solve() {
    let mut monkeys = puzzle_input();

    let result = play(&mut monkeys, true, 125);
    println!("Result: {}", result);
}

fn play(mut monkeys: &mut Vec<Monkey>, very_worried: bool, rounds: usize) -> u128 {
    for i in 0..rounds {
        println!("| Round #{:03}", i);
        // println!("--------------------- Round: {}", i);
        
        // for (idx, monkey) in monkeys.iter_mut().enumerate() {
        //     // print!("[M:{},I:{}]\t", idx, monkey.items.len());
        //     for item in &monkey.items {
        //         print!("{}|", item.worry_level);
        //     }
        // }
        // println!();
        // println!();
        round(&mut monkeys, very_worried);
    }

    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));
    let first = monkeys.get(0).unwrap();
    let second = monkeys.get(1).unwrap();

    first.inspections * second.inspections
}

fn test_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vecdeque![Item::new_named(79, "konrado"), Item::new_named(98, "moreno")],
            operation: Operation {
                operator: Operator::Multiply,
                left: Value::Old,
                right: Value::from(19),
            },
            test: Test {
                divisible_by: 23,
                target_true: 2,
                target_false: 3,
            },
            inspections: 0,
            
        },
        Monkey {
            items: vecdeque![Item::new_named(54, "felicita"), Item::new(65), Item::new(75), Item::new(74)],
            operation: Operation {
                operator: Operator::Add,
                left: Value::Old,
                right: Value::from(6),
            },
            test: Test {
                divisible_by: 19,
                target_true: 2,
                target_false: 0,
            },
            inspections: 0,
            
        },
        Monkey {
            items: vecdeque![Item::new_named(79, "airport"), Item::new(60), Item::new(97)],
            operation: Operation {
                operator: Operator::Multiply,
                left: Value::Old,
                right: Value::Old,
            },
            test: Test {
                divisible_by: 13,
                target_true: 1,
                target_false: 3,
            },
            inspections: 0,
            
        },
        Monkey {
            items: vecdeque![Item::new_named(74, "midlands")],
            operation: Operation {
                operator: Operator::Add,
                left: Value::Old,
                right: Value::from(3),
            },
            test: Test {
                divisible_by: 17,
                target_true: 0,
                target_false: 1,
            },
            inspections: 0,
            
        },
    ]
}

mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let mut monkeys = vec![Monkey {
            items: vecdeque![Item::new(79), Item::new(98)],
            operation: Operation {
                operator: Operator::Multiply,
                left: Value::Old,
                right: Value::from(19),
            },
            test: Test {
                divisible_by: 23,
                target_true: 2,
                target_false: 3,
            },
            inspections: 0,
            
        }];

        round(&mut monkeys, false);

        // println!("{:?}", receive_queue);
        // assert_eq!(2, receive_queue.get(&3).unwrap().len())
    }

    #[test]
    fn base() {
        assert_eq!(10605, play(&mut test_monkeys(), false, 20));
    }

    #[test]
    fn base_part2() {
        assert_eq!(2713310158, play(&mut test_monkeys(), true, 10000));
    }

    #[test]
    fn  test_part2() {
        assert_eq!(2713310158, play(&mut puzzle_input(), true, 10000));
    }
}
