use std::{collections::{VecDeque, HashSet}, rc::Rc, cell::{RefCell, RefMut}, fs::File};
use std::io::{self, BufRead};

#[derive(Debug)]
enum Value {
    List(Vec<Node>),
    Simple(u32),
    Empty,
}


#[derive(Debug)]
struct Node {
    value: Value,
}

impl Node {
    fn parse(line: &str) -> Node {
        let mut stack: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut current_node: Option<Rc<RefCell<Node>>> = None;

        for char in line.chars() { 
            if char == '[' {
                match current_node.clone() {
                    Some(n) => {
                        // remember where we were
                        stack.push_front(n.clone());
                    },
                    None => {}
                }
                current_node = Some(Rc::new(RefCell::new(Node::new_list())));

            } else if char == ']' {
                println!("Popping from the stack");
                let mut old_current_node = current_node.clone();
                current_node = stack.pop_front();

                match current_node.clone() {
                    Some(current_node) => {
                        match old_current_node.take() {
                            Some(old_current_node) => {
                                match Rc::try_unwrap(old_current_node) {
                                    Ok(old_current_node) => {
                                        let old_current_node = old_current_node.into_inner();

                                        let mut current_node: RefMut<Node> = current_node.borrow_mut();
                                        match current_node.value {
                                            Value::List(ref mut values) => {
                                                values.push(old_current_node);
                                            },
                                            Value::Simple(_) => todo!(),
                                            Value::Empty => todo!(),
                                        }
                                        
                                    },
                                    Err(_) => todo!(),
                                }
                            }
                            None => todo!(),
                        }
                    },
                    None => {
                        // set root
                        current_node = old_current_node;
                        // do nothing, previous was not a list
                    },
                }
            } else if char == ',' {
                continue;
            } else {
                if let Some(number) = char.to_digit(10) {
                    if let Some(current_node) = current_node.clone() {
                        match current_node.borrow_mut().value {
                            Value::List(ref mut nodes) =>  {
                                println!("Pushing: {}", number);
                                nodes.push(Node::simple(number));
                            },
                            Value::Simple(_) => panic!("Shoulnt be simple"),
                            Value::Empty => todo!(),
                        }
                    } else {
                        panic!("Shouldn't happen.")
                    }
                } else {
                    panic!("Shouldn't happen, char: {}", char);
                }
            }
        }

        match current_node.take() {
            Some(x) => {
                match Rc::try_unwrap(x) {
                    Ok(d) => {
                        let x = d.into_inner();
                        return x
                    },
                    Err(_) => todo!(),
                }
            }
            None => todo!(),
        }
    }

    fn ordered(&self, right: &Node) -> bool {
        let mut left_queue: VecDeque<&Value> = VecDeque::new();
        let mut right_queue: VecDeque<&Value> = VecDeque::new();

        left_queue.push_back(&self.value);
        right_queue.push_back(&right.value);

        let mut last_compare = false;
    
        while left_queue.len() > 0 {
            let left_value = left_queue.pop_front().unwrap();
            if right_queue.len() == 0 {
                println!("returning because of the empty queue");
                return last_compare;
            }
            let right_value = right_queue.pop_front().unwrap();

            last_compare = false;
            match left_value {
                Value::List(left_values) => {
                    println!("!!!! left is list, {:?}", left_value);

                    left_values.iter().for_each(|node| left_queue.push_back(&node.value));
                    match right_value {
                        Value::List(right_values) => {
                            println!("right is list, {:?}", right_value);
                            right_values.iter().for_each(|node| right_queue.push_back(&node.value));
                        }
                        // push back, vs push front
                        Value::Simple(_) => { 
                            println!("right is simple, {:?}", right_value);
                            right_queue.push_back(right_value)
                        }
                        Value::Empty => todo!(),
                    }
                },
                Value::Simple(left_v) => {
                    println!("!!! left is simple {:?}", left_value);
                    match right_value {
                        Value::List(right_values) => { 
                            println!("!!! left is simple, but right is list, {:?}", right_value);
                            left_queue.push_front(left_value);
                            right_values.iter().for_each(|node| right_queue.push_back(&node.value))
                        },
                        Value::Simple(right_v) => {
                            println!("comparing: {}, {}", left_v, right_v);
                            if left_v > right_v {
                                return false;
                            } else if left_v < right_v {
                                last_compare = true;
                            }
                        },
                        Value::Empty => todo!(),
                    }
                },
                Value::Empty => todo!(),
            }
        }

        return true;
    }

    fn new() -> Node {
        Node {
            value: Value::Empty,
        }
    }

    fn new_list() -> Node {
        Node {
            value: Value::List(vec![]),
        }
    }

    fn simple(num: u32) -> Node {
        Node {
            value: Value::Simple(num),
        }
    }

    fn simple_list(nums: Vec<u32>) -> Node {
        Node {
            value: Value::List(nums.iter()
                .map(|v| 
                   Node::simple(*v)
                )
                .collect()
            )
        }
    }
}

pub fn solve() {
    let file = match File::open("inputs/13_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();

    let mut ordered = HashSet::new();
    let mut pair_index: usize = 0;
    let mut left: Option<Node> = None;
    let mut right: Option<Node> = None;
    for line in reader {

        if let Ok(line) = line {
            if left.is_none() {
                pair_index += 1;
                left = Some(Node::parse(&line));
            } else if right.is_none() {
                right = Some(Node::parse(&line))
            } else {
                if left.unwrap().ordered(&right.unwrap()) {
                    ordered.insert(pair_index);
                }
                left = None;
                right = None;
            }
        }
    }

    if left.is_some() && left.unwrap().ordered(&right.unwrap()) {
        ordered.insert(pair_index);
    }

    
    println!("{:?}, {}", ordered, ordered.iter().sum::<usize>());
}

mod tests {
    use super::*;

    #[test]
    fn left_greater_than_right() {
        let left = Node::simple(100);
        let right = Node::simple(0);

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn right_greater_than_left() {
        let left = Node::simple(0);
        let right = Node::simple(1);

        assert_eq!(true, left.ordered(&right));
    }

    #[test]
    fn right_runs_out_of_the_items() {
        let left = Node::simple_list(vec![1]);
        let right = Node::simple_list(vec![]);

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn simple_case() {
        let left = Node::simple_list(vec![1]);
        let right = Node::simple_list(vec![2]);

        assert_eq!(true, left.ordered(&right));
    }


    #[test]
    fn simple_case_longer_lists() {
        let left = Node::simple_list(vec![1, 1, 3, 1, 1]);
        let right = Node::simple_list(vec![1, 1, 5, 1, 1]);

        assert_eq!(true, left.ordered(&right));
    }

    #[test]
    fn complex_case() {
        // [[1], [2, 3, 4]]
        // [[1], 4]
        let left = Node {
            value: Value::List(vec![
                Node::simple_list(vec![1]),
                Node::simple_list(vec![2,3,4]),
            ])
        };
        let right = Node {
            value: Value::List(vec![
                Node::simple_list(vec![1]),
                Node::simple(4),
            ])
        };

        assert_eq!(true, left.ordered(&right));
    }

    #[test]
    fn complex_case_2() {
        // [9] vs 
        // [[8,7,6]]
        let left = Node {
            value: Value::List(vec![
                Node::simple(9)
            ])
        };
        let right = Node {
            value: Value::List(vec![
                Node::simple_list(vec![8, 7, 6]),
            ])
        };

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn left_without_itmes() {
        // [[4,4],4,4] 
        // [[4,4],4,4,4]

        let left = Node {
            value: Value::List(vec![
                Node::simple_list(vec![4, 4]),
                Node::simple(4),
                Node::simple(4)
            ])
        };
        let right = Node {
            value: Value::List(vec![
                Node::simple_list(vec![4, 4]),
                Node::simple(4),
                Node::simple(4),
                Node::simple(4),
            ])
        };

        assert_eq!(true, left.ordered(&right));
    }

    #[test]
    fn right_without_itmes() {
        // [7, 7, 7, 7] 
        // [7, 7, 7] 

        let left = Node::simple_list(vec![7, 7, 7, 7]);
        let right = Node::simple_list(vec![7, 7, 7]);

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn right_simple_without_itmes() {
        // [] 
        // [3] 

        let left = Node::simple_list(vec![]);
        let right = Node::simple_list(vec![3]);

        assert_eq!(true, left.ordered(&right));
    }

    #[test]
    fn empty_recurrent_lists() { 
        //  [[]]
        let left = Node::parse("[[[]]]");
        let right = Node {
            value: Value::List(
                vec![
                    Node {
                        value: Value::List(
                            vec![]
                        )
                    }
                ]
            )
        };

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn very_complex_example() {
        let left = Node::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let right = Node::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]");

        assert_eq!(false, left.ordered(&right));
    }


    #[test]
    fn failing() {
        let left = Node::parse("[[1],[2,3,4]]");
        let right = Node::parse("[[1],4]");


        assert_eq!(true, left.ordered(&right));
    }

    #[test]
    fn parse() {
        let input = "[[1,2],3,4,5]";
        let expected = Node {
            value: Value::List(vec![
                Node::simple_list(vec![1, 2]),
                Node::simple(3),
                Node::simple(4),
                Node::simple(5)
            ])
        };

        println!("expected: {:?}", expected);
        let parsed = Node::parse(input);
        println!("actual: {:?}", parsed);
        assert_eq!(true, expected.ordered(&parsed));
    } 
}

/*
13_base
What are the indices of the pairs that are already in the right order?
 (The first pair has index 1, the second pair has index 2, and so on.)
 In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.
*/