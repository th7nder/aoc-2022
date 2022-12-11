use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
};

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
    Simple(usize),
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

#[derive(Debug)]
struct Item {
    worry_level: usize,
}

#[derive(Debug)]
struct Test {
    divisible_by: usize,
    target_true: usize,
    target_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
    inspections: usize,
}

fn round(monkeys: &mut Vec<Monkey>) -> HashMap<usize, VecDeque<Item>> {
    let mut receive_queue: HashMap<usize, VecDeque<Item>> = HashMap::new();

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
            let left_value = match monkey.operation.left {
                Value::Old => item.worry_level,
                Value::Simple(v) => v,
            };
            let right_value = match monkey.operation.right {
                Value::Old => item.worry_level,
                Value::Simple(v) => v,
            };

            item.worry_level = match monkey.operation.operator {
                Operator::Multiply => left_value * right_value,
                Operator::Add => left_value + right_value,
            };
            item.worry_level /= 3;

            // not sure?
            monkey.inspections += 1;
            if item.worry_level % monkey.test.divisible_by == 0 {
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

    for (idx, mut monkey) in monkeys.iter_mut().enumerate() {
        if let Some(queue) = receive_queue.get_mut(&idx) {
            while let Some(item) = queue.pop_front() {
                monkey.items.push_back(item);
            }
        }
    }

    return receive_queue;
}

pub fn solve() {}

mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let mut monkeys = vec![Monkey {
            items: vecdeque![Item { worry_level: 79 }, Item { worry_level: 98 }],
            operation: Operation {
                operator: Operator::Multiply,
                left: Value::Old,
                right: Value::Simple(19),
            },
            test: Test {
                divisible_by: 23,
                target_true: 2,
                target_false: 3,
            },
            inspections: 0,
        }];

        let receive_queue = round(&mut monkeys);

        println!("{:?}", receive_queue);
        assert_eq!(2, receive_queue.get(&3).unwrap().len())
    }

    #[test]
    fn base() {
        let mut monkeys = vec![
            Monkey {
                items: vecdeque![Item { worry_level: 79 }, Item { worry_level: 98 }],
                operation: Operation {
                    operator: Operator::Multiply,
                    left: Value::Old,
                    right: Value::Simple(19),
                },
                test: Test {
                    divisible_by: 23,
                    target_true: 2,
                    target_false: 3,
                },
                inspections: 0,
            },
            Monkey {
                items: vecdeque![Item { worry_level: 54 }, Item { worry_level: 65 }, Item { worry_level: 75 }, Item { worry_level: 74 }],
                operation: Operation {
                    operator: Operator::Add,
                    left: Value::Old,
                    right: Value::Simple(6),
                },
                test: Test {
                    divisible_by: 19,
                    target_true: 2,
                    target_false: 0,
                },
                inspections: 0,
            },
            Monkey {
                items: vecdeque![Item { worry_level: 79 }, Item { worry_level: 60 }, Item { worry_level: 97 }],
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
                items: vecdeque![Item { worry_level: 74 }],
                operation: Operation {
                    operator: Operator::Add,
                    left: Value::Old,
                    right: Value::Simple(3),
                },
                test: Test {
                    divisible_by: 17,
                    target_true: 0,
                    target_false: 1,
                },
                inspections: 0,
            },
        ];

        for _ in 0..20 {
            round(&mut monkeys);
        }
        
        println!("{:?}", monkeys);
    }
}
