use std::io::{self, BufRead};
use std::{
    cell::{RefCell, RefMut},
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    rc::Rc,
};

#[derive(Debug)]
enum Value {
    List(Vec<Node>),
    Simple(u32),
}

#[derive(Debug)]
struct Node {
    value: Value,
    level: u32,
}

#[derive(PartialEq, Eq, Debug)]
enum OrderingResult {
    Same,
    Correct,
    Incorrect
}

impl Node {
    fn parse(line: &str) -> Node {
        let mut stack: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut current_node: Option<Rc<RefCell<Node>>> = None;

        let mut current_level = 0;
        for char in line.chars() {
            if char == '[' {
                match current_node.clone() {
                    Some(n) => {
                        // remember where we were
                        stack.push_front(n.clone());
                    }
                    None => {}
                }
                current_level += 1;
                current_node = Some(Rc::new(RefCell::new(Node::new_list(current_level))));
            } else if char == ']' {
                println!("Popping from the stack");
                let mut old_current_node = current_node.clone();
                current_node = stack.pop_front();

                match current_node.clone() {
                    Some(current_node) => match old_current_node.take() {
                        Some(old_current_node) => match Rc::try_unwrap(old_current_node) {
                            Ok(old_current_node) => {
                                let old_current_node = old_current_node.into_inner();

                                let mut current_node: RefMut<Node> = current_node.borrow_mut();
                                match current_node.value {
                                    Value::List(ref mut values) => {
                                        values.push(old_current_node);
                                    }
                                    Value::Simple(_) => todo!(),
                                }
                            }
                            Err(_) => todo!(),
                        },
                        None => todo!(),
                    },
                    None => {
                        // set root
                        current_node = old_current_node;
                        // do nothing, previous was not a list
                    }
                }
            } else if char == ',' {
                continue;
            } else {
                if let Some(number) = char.to_digit(10) {
                    if let Some(current_node) = current_node.clone() {
                        match current_node.borrow_mut().value {
                            Value::List(ref mut nodes) => {
                                println!("Pushing: {}", number);
                                nodes.push(Node::simple(number, current_level));
                            }
                            Value::Simple(_) => panic!("Shoulnt be simple"),
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
            Some(x) => match Rc::try_unwrap(x) {
                Ok(d) => {
                    let x = d.into_inner();
                    return x;
                }
                Err(_) => todo!(),
            },
            None => todo!(),
        }
    }

    // need to decide when to do recursive and when to do iterative



    // we need to keep track whether it was already compared?
    fn ordered_recursive(&self, right: &Node) -> OrderingResult {
        // [1, [1,2,3]]
        // [1,2,]
        match self.value {
            // starting with [1,2,3,4]
            Value::List(ref values) => {
                match right.value {
                    Value::List(ref right_values) => {
                        println!("Comparing two lists: {:?} and {:?}", values, right_values);
                        for (left_node_idx, left_node) in values.iter().enumerate() {
                            println!("Processing: {:?}", left_node);
                            if let Some(right_node) = right_values.get(left_node_idx) {
                                match left_node.value {
                                    // [[1,2,3]], [1,2,3]
                                    Value::List(_) => {
                                        println!("Doing some weird logic");

                                        match right_node.value {
                                            Value::List(_) => {
                                                match left_node.ordered_recursive(right_node) {
                                                    OrderingResult::Same => {},
                                                    OrderingResult::Correct => {}
                                                    res => {
                                                        return res;
                                                    } 
                                                }
                                            },
                                            Value::Simple(right_inner_value_number) => {
                                                match left_node.ordered_recursive(&Node::simple_list(vec![right_inner_value_number], right_node.level)) {
                                                    OrderingResult::Same => {},
                                                    OrderingResult::Correct => {}
                                                    res => {
                                                        return res;
                                                    } 
                                                }
                                            }
                                        }

                                    },
                                    // [1,2,3] vs [[1], 2, 3]
                                    // [1,2,3] vs [[1,2],3 ]
                                    Value::Simple(left_inner_value_number) => {
                                        match right_node.value {
                                            // 1 vs [1]
                                            // 1 vs [1,2]
                                            Value::List(_) => {
                                                let new_node = Node::simple_list(vec![left_inner_value_number], left_node.level);
                                                // 1 vs [1]
                                                // turns into [1] vs [1]
                                                println!("Converting {:?} into {:?}", left_node, new_node);
                                                match new_node.ordered_recursive(right_node) {
                                                    OrderingResult::Same => {},
                                                    OrderingResult::Correct => {},
                                                    res => {
                                                        return res;
                                                    } 
                                                }
                                            },
                                            Value::Simple(right_inner_value_number) => {
                                                println!("Comparring {} vs {}", left_inner_value_number, right_inner_value_number);
                                                if left_inner_value_number < right_inner_value_number {
                                                    return OrderingResult::Correct;
                                                } else if left_inner_value_number > right_inner_value_number {
                                                    return OrderingResult::Incorrect;
                                                }
                                            },
                                        }
                                    },
                                }
                            } else {
                                return OrderingResult::Incorrect;
                            }
                        }
                    },
                    // [4,1] vs 5 -> Incorrect
                    // [] vs 5 -> Correc
                    // [4] vs 5 -> Correct
                    // [[4]] vs 5 -> process next
                    Value::Simple(right_value_number) => {
                        if values.len() > 1 {
                            return OrderingResult::Incorrect;
                        } else if values.len() == 0 {
                            return OrderingResult::Correct;
                        }

                        for (left_node_idx, left_node) in values.iter().enumerate() {
                            match left_node.value {
                                Value::List(_) => {
                                    match left_node.ordered_recursive(&Node::simple_list(vec![right_value_number], right.level)) {
                                        OrderingResult::Same => {},
                                        OrderingResult::Correct => {},
                                        res => {
                                            return res;
                                        },
                                    }
                                },
                                Value::Simple(left_value_number) => {
                                    if left_value_number < right_value_number {
                                        return OrderingResult::Correct;
                                    } else if left_value_number > right_value_number {
                                        return OrderingResult::Incorrect;
                                    }
                                }
                            }    
                        }
                        
                        // len == 0, [] vs 5
                        // len == 0, []
                        // ??
                        return OrderingResult::Correct;
                    },
                }
            }
            // 1, 2
            // 1, [[2]]
            // 1, [2]
            Value::Simple(left_value_number) => todo!(),
        }

        return OrderingResult::Correct;
    }

    // todo, create it in the recurrent manner.
    // fn ordered<'a>(&'a self, right: &Node) -> bool {
    //     let mut borrowable_values: HashMap<u32, Value> = HashMap::new();
    //     for i in 0..10 {
    //         borrowable_values.insert(i, Value::List(vec![Node::simple(i)]));
    //     }

    //     let mut left_queue: VecDeque<&Value> = VecDeque::new();
    //     let mut right_queue: VecDeque<&Value> = VecDeque::new();

    //     left_queue.push_back(&self.value);
    //     right_queue.push_back(&right.value);

    //     while left_queue.len() > 0 {
    //         println!("left queue: {:?}", left_queue);
    //         println!("right queue: {:?}", right_queue);
    //         let left_value = left_queue.pop_front().unwrap();
    //         if right_queue.len() == 0 {
    //             println!("returning because of the empty queue");
    //             return false;
    //         }
    //         let right_value = right_queue.pop_front().unwrap();

    //         // convert to a list, instead of converting to a number!
    //         match left_value {
    //             Value::List(left_values) => {
    //                 println!("!!!! left is list, {:?}", left_value);

    //                 match right_value {
    //                     Value::List(right_values) => {
    //                         let mut last_right_idx = 0;
    //                         println!("#### right is list, {:?}", right_value);
    //                         for (left_idx, left_node) in left_values.iter().enumerate().rev() {
    //                             left_queue.push_front(&left_node.value);

    //                             if let Some(right_node) = right_values.get(left_idx) {
    //                                 last_right_idx = left_idx;
    //                                 match right_node.value {
    //                                     Value::List(_) => right_queue.push_front(&right_node.value),
    //                                     Value::Simple(right_value) => match left_node.value {
    //                                         Value::List(_) => {
    //                                             left_queue.push_front(&left_node.value);
    //                                             right_queue.push_front(
    //                                                 &borrowable_values.get(&right_value).unwrap(),
    //                                             )
    //                                         }
    //                                         Value::Simple(left_value) => {
    //                                             if left_value < right_value {
    //                                                 return true;
    //                                             } else if left_value > right_value {
    //                                                 return false;
    //                                             }
    //                                         }
    //                                     },
    //                                 }
    //                                 right_queue.push_front(&right_node.value);
    //                             }
    //                         }

    //                         for idx in last_right_idx..right_values.len() {
    //                             right_queue.push_back(&right_values.get(idx).unwrap().value);
    //                         }
    //                     }
    //                     Value::Simple(v) => {
    //                         left_queue.push_front(&left_value);
    //                         right_queue.push_front(&borrowable_values.get(v).unwrap());
    //                     }
    //                 }
    //             }
    //             Value::Simple(left_v) => {
    //                 println!("!!! left is simple {:?}", left_value);
    //                 match right_value {
    //                     Value::List(_) => {
    //                         println!("### right is list {:?}", right_value);
    //                         left_queue.push_front(&borrowable_values.get(left_v).unwrap());
    //                         right_queue.push_front(right_value);
    //                     }
    //                     Value::Simple(right_v) => {
    //                         println!("comparing: {} vs {}", left_v, right_v);
    //                         if left_v > right_v {
    //                             return false;
    //                         } else if left_v < right_v {
    //                             return true;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     return true;
    // }

    fn new_list(level: u32) -> Node {
        Node {
            value: Value::List(vec![]),
            level,
        }
    }

    fn simple(num: u32, level: u32) -> Node {
        Node {
            value: Value::Simple(num),
            level,
        }
    }

    fn simple_list(nums: Vec<u32>, level: u32) -> Node {
        Node {
            value: Value::List(nums.iter().map(|v| Node::simple(*v, level + 1)).collect()),
            level
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
                if left.unwrap().ordered_recursive(&right.unwrap()) == OrderingResult::Correct {
                    ordered.insert(pair_index);
                }
                left = None;
                right = None;
            }
        }
    }

    if left.is_some() && left.unwrap().ordered_recursive(&right.unwrap()) == OrderingResult::Correct {
        ordered.insert(pair_index);
    }

    println!("{:?}, {}", ordered, ordered.iter().sum::<usize>());
}

mod tests {
    use super::*;

    #[test]
    fn base_case_1() {
        let left = Node::parse("[1,2,3,4]");
        let right = Node::parse("[1,2,3,4]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn base_case_2() {
        let left = Node::parse("[1,2,3]");
        let right = Node::parse("[1,2,3,4]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn base_case_3() {
        let left = Node::parse("[1,2,3,4]");
        let right = Node::parse("[1,2,3]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_1() {
        let left = Node::parse("[1,2,3,[4,1]]");
        let right = Node::parse("[1,2,3,5,6]");

        assert_eq!(OrderingResult::Incorrect,left.ordered_recursive(&right));
    }


    #[test]
    fn complex_2() {
        let left = Node::parse("[1,2,3,[4],1]");
        let right = Node::parse("[1,2,3,5,6]");

        assert_eq!(OrderingResult::Correct,left.ordered_recursive(&right));
    }

    #[test]
    fn complex_3() {
        let left = Node::parse("[1,2,3]");
        let right = Node::parse("[1,[2],3]");

        assert_eq!(OrderingResult::Correct,left.ordered_recursive(&right));
    }

    #[test]
    fn complex_4() {
        // [1,[2],3]
        let left = Node::parse("[1,[2],3]");
        let right = Node::parse("[1,[2,3]]");

        assert_eq!(OrderingResult::Incorrect,left.ordered_recursive(&right));
    }

    #[test]
    fn complex_5() {
        // [1,[2],3]
        let left = Node::parse("[1,2,3]");
        let right = Node::parse("[1,[2,3]]");

        assert_eq!(OrderingResult::Incorrect,left.ordered_recursive(&right));
    }

    // #[test]
    // fn left_greater_than_right() {
    //     let left = Node::simple(100);
    //     let right = Node::simple(0);

    //     assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    // }

    // #[test]
    // fn right_greater_than_left() {
    //     let left = Node::simple(0);
    //     let right = Node::simple(1);

    //     assert_eq!(OrderingResult::Incorrect,  left.ordered_recursive(&right));
    // }

    // #[test]
    // fn right_runs_out_of_the_items() {
    //     let left = Node::simple_list(vec![1]);
    //     let right = Node::simple_list(vec![]);

    //     assert_eq!(OrderingResult::Incorrect,  left.ordered_recursive(&right));
    // }

    // #[test]
    // fn simple_case() {
    //     let left = Node::simple_list(vec![1]);
    //     let right = Node::simple_list(vec![2]);

    //     assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    // }

    // #[test]
    // fn simple_case_longer_lists() {
    //     let left = Node::simple_list(vec![1, 1, 3, 1, 1]);
    //     let right = Node::simple_list(vec![1, 1, 5, 1, 1]);

    //     assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    // }

    #[test]
    fn complex_case() {
        // [[1], [2, 3, 4]]
        // [[1], 4]
        let left = Node::parse("[[1],[2,3,4]]");
        let right = Node::parse("[[1],4]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    
    #[test]
    fn complex_case_2() {
        // [9] vs
        // [[8,7,6]]
        let left = Node::parse("[9]");
        let right = Node::parse("[[8,7,6]]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_case_woot() {
        let left = Node::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let right = Node::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_case_woot_2() {
        let left = Node::parse("[[4,4],4,4]");
        let right = Node::parse("[[4,4],4,4,4]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }


    #[test]
    fn complex_case_woot_3() {
        let left = Node::parse("[]");
        let right = Node::parse("[3]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_case_woot_4() {
        let left = Node::parse("[[[]]]");
        let right = Node::parse("[[]]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_case_woot_44() {
        let left = Node::parse("[[4],3]");
        let right = Node::parse("[[5],2]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }


    #[test]
    fn complex_case_woot_5() {
        //        let left = Node::parse(" [[ ],    [0] , [[ ]] ]");
        //        let right = Node::parse("[[0],   [[4]],       ]");
        let left = Node::parse("[[],[0],[[]]]");
        let right = Node::parse("[[0],[[4]]]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }



    /* 
    #[test]
    fn complex_case_3() {
        let left = Node::parse("[9,8,7,6,5,4,8]");
        let right = Node::parse("[9,[[8],[[[7]]],6,[[5]]],4,[[3]]]");

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn complex_case_4() {
        let left = Node::parse("[9,8,7,6,5,4,8]");
        let right = Node::parse("[9,[[8],[[[7]]],6,[[5]]],4,[[]]]");

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn complex_case_finally_understood_problem() {
        let left = Node::parse("[9,8,[7,6,5],4]");
        let right = Node::parse("[9,8,7,6,5,4]");

        assert_eq!(false, left.ordered(&right));
    }

    #[test]
    fn complex_case_finally_understood_problem_v2() {
        let left = Node::parse("[[1],[2,3,4]]");
        let right = Node::parse("[[1],2,3]");

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
                Node::simple(4),
            ]),
        };
        let right = Node {
            value: Value::List(vec![
                Node::simple_list(vec![4, 4]),
                Node::simple(4),
                Node::simple(4),
                Node::simple(4),
            ]),
        };

        assert_eq!(true, left.ordered(&right));
    }

    // let's try to break it

    #[test]
    fn neighbouring_lists() {
        let left = Node::parse("[[1,2,3],[9,8,7,6,5],0]");
        let right = Node::parse("[[1,2,3],[[[9,8,7],6],5],4]");

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
        let left = Node::parse("[[[0,[2]]]]");
        let right = Node::parse("[[[[2]]]]");

        assert_eq!(true, left.ordered(&right));
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
                Node::simple(5),
            ]),
        };

        println!("expected: {:?}", expected);
        let parsed = Node::parse(input);
        println!("actual: {:?}", parsed);
        assert_eq!(true, expected.ordered(&parsed));
    } */
}

/*
13_base
What are the indices of the pairs that are already in the right order?
 (The first pair has index 1, the second pair has index 2, and so on.)
 In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.
*/
