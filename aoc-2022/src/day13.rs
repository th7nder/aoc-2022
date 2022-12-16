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
    Incorrect,
}

impl OrderingResult {
    fn good(&self) -> bool {
        match self {
            OrderingResult::Same => true,
            OrderingResult::Correct => true,
            OrderingResult::Incorrect => false,
        }
    }
}

impl Node {
    fn parse(line: &str) -> Node {
        let mut stack: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut current_node: Option<Rc<RefCell<Node>>> = None;

        let mut current_level = 0;
        let mut current_str = "".to_string();
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
                if current_str.len() > 0 {
                    let number = current_str.parse::<u32>().unwrap();
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
    
                    current_str = "".to_string();
                }

                current_level -= 1;
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
                if current_str.len() > 0 {
                    let number = current_str.parse::<u32>().unwrap();
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
    
                    current_str = "".to_string();
                }
            } else {
                current_str.push(char);
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
        println!(
            "Ordering node at level: {} = {} vs {}",
            self.level,
            self.print(),
            right.print()
        );
        match self.value {
            // starting with [1,2,3,4]
            Value::List(ref values) => {
                // println!("Left value is list: {:?}", self.value);
                for (left_node_idx, left_node) in values.iter().enumerate() {
                    // println!(
                    // "Processing node index: {}, at level: {} = {:?}",
                    // left_node_idx, left_node.level, left_node
                    // );

                    match left_node.value {
                        Value::List(_) => {
                            println!("Left node is list, so going deeper.");
                            match right.value {
                                // [1,2,3,4] vs [1,2,3]
                                // [1,2,3,4] vs []
                                Value::List(ref right_deep_values) => {
                                    if let Some(right_deep_node) =
                                        right_deep_values.get(left_node_idx)
                                    {
                                        match right_deep_node.value {
                                            // [1,2,3] vs [[1,2,3]] => [1] vs [1,2,3]
                                            Value::List(_) => {
                                                println!("Right deep value is a list");
                                                match left_node.ordered_recursive(right_deep_node) {
                                                    OrderingResult::Same => {}
                                                    res => {
                                                        return res;
                                                    }
                                                }
                                            }
                                            Value::Simple(right_deep_value_number) => {
                                                println!("Right deep value is simple");
                                                let new_node = Node::simple_list(
                                                    vec![right_deep_value_number],
                                                    self.level + 1,
                                                );
                                                match left_node.ordered_recursive(&new_node) {
                                                    OrderingResult::Same => {}
                                                    res => {
                                                        return res;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        println!("There is no items");
                                        // we ran out of the items
                                        return OrderingResult::Incorrect;
                                    }
                                }
                                Value::Simple(_) => todo!("Shouldn't happen?"),
                            }

                            // // TODO: should we pass the current path here?
                            // match left_node.ordered_recursive(right) {
                            //     // Same, means that what?
                            //     // There was the same number of items on the left and on the right.
                            //     // WE need to proceed further.
                            //     OrderingResult::Same => {}
                            //     res => {
                            //         return res;
                            //     }
                            // }
                        }
                        // as well ass [1,2,3] vs []
                        // [1,2,3,4] vs [1,2,3,4]
                        // but also
                        // [1,2,3,4] vs [[1],2,[3,4]]
                        // => [1] vs [1]
                        // => 2 vs 2
                        // => [3] s [3,4]
                        // => 4 vs nothing
                        Value::Simple(left_value_number) => {
                            println!("Left node is a number, so we want to compare.");

                            match right.value {
                                // [1,2,3,4] vs [1,2,3]
                                // [1,2,3,4] vs []
                                Value::List(ref right_deep_values) => {
                                    if let Some(right_deep_node) =
                                        right_deep_values.get(left_node_idx)
                                    {
                                        match right_deep_node.value {
                                            // [1,2,3] vs [[1,2,3]] => [1] vs [1,2,3]
                                            Value::List(_) => {
                                                let new_node = Node::simple_list(
                                                    vec![left_value_number],
                                                    self.level + 1,
                                                );
                                                match new_node.ordered_recursive(right_deep_node) {
                                                    OrderingResult::Same => {}
                                                    res => {
                                                        return res;
                                                    }
                                                }
                                            }
                                            Value::Simple(right_deep_value_number) => {
                                                println!(
                                                    "Comparing: {} to {}",
                                                    left_value_number, right_deep_value_number
                                                );
                                                if left_value_number < right_deep_value_number {
                                                    return OrderingResult::Correct;
                                                } else if left_value_number
                                                    > right_deep_value_number
                                                {
                                                    return OrderingResult::Incorrect;
                                                }
                                            }
                                        }
                                    } else {
                                        println!("There is no items");
                                        // we ran out of the items
                                        return OrderingResult::Incorrect;
                                    }
                                }
                                Value::Simple(_) => todo!("Shouldn't happen?"),
                            }
                        }
                    }
                }
                println!("Reached end of the life.");



                if values.len() == 0 {
                    return match right.value {
                        Value::List(ref right_values) => {
                            if right_values.len() == 0 {
                                OrderingResult::Same
                            } else {
                                OrderingResult::Correct
                            }
                        }
                        Value::Simple(_) => OrderingResult::Correct
                    }
                } else {
                    return match right.value {
                        Value::List(ref right_values) => {
                            if values.len() < right_values.len() {
                                OrderingResult::Correct
                            } else {
                                OrderingResult::Same
                            }
                        },
                        Value::Simple(_) => todo!(),
                    }
                }

                // if it reached the end
                // we might want to return Same, because upper function call will need to continue.
                // we might want to return Correct, because it was empty.
            }
            // 1, 2
            // 1, [[2]]
            // 1, [2]
            Value::Simple(_) => {
                todo!("Shouldn't happen");
            }
        }
    }

    fn ordered_recursive_old(&self, right: &Node) -> OrderingResult {
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
                                                    OrderingResult::Same => {}
                                                    OrderingResult::Correct => {}
                                                    res => {
                                                        return res;
                                                    }
                                                }
                                            }
                                            Value::Simple(right_inner_value_number) => {
                                                match left_node.ordered_recursive(
                                                    &Node::simple_list(
                                                        vec![right_inner_value_number],
                                                        right_node.level,
                                                    ),
                                                ) {
                                                    OrderingResult::Same => {}
                                                    OrderingResult::Correct => {}
                                                    res => {
                                                        return res;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // [1,2,3] vs [[1], 2, 3]
                                    // [1,2,3] vs [[1,2],3 ]
                                    Value::Simple(left_inner_value_number) => {
                                        match right_node.value {
                                            // 1 vs [1]
                                            // 1 vs [1,2]
                                            Value::List(_) => {
                                                let new_node = Node::simple_list(
                                                    vec![left_inner_value_number],
                                                    left_node.level,
                                                );
                                                // 1 vs [1]
                                                // turns into [1] vs [1]
                                                println!(
                                                    "Converting {:?} into {:?}",
                                                    left_node, new_node
                                                );
                                                match new_node.ordered_recursive(right_node) {
                                                    OrderingResult::Same => {}
                                                    OrderingResult::Correct => {}
                                                    res => {
                                                        return res;
                                                    }
                                                }
                                            }
                                            Value::Simple(right_inner_value_number) => {
                                                println!(
                                                    "Comparring {} vs {}",
                                                    left_inner_value_number,
                                                    right_inner_value_number
                                                );
                                                if left_inner_value_number
                                                    < right_inner_value_number
                                                {
                                                    return OrderingResult::Correct;
                                                } else if left_inner_value_number
                                                    > right_inner_value_number
                                                {
                                                    return OrderingResult::Incorrect;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                return OrderingResult::Incorrect;
                            }
                        }
                    }
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
                                    match left_node.ordered_recursive(&Node::simple_list(
                                        vec![right_value_number],
                                        right.level,
                                    )) {
                                        OrderingResult::Same => {}
                                        OrderingResult::Correct => {}
                                        res => {
                                            return res;
                                        }
                                    }
                                }
                                Value::Simple(left_value_number) => {
                                    println!(
                                        "Comparing: {} to {}",
                                        left_value_number, right_value_number
                                    );
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
                    }
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
            level,
        }
    }

    fn print(&self) -> String {
        match self.value {
            Value::List(ref l) => {
                let mut xd = "[".to_string();
                for (idx, n) in l.iter().enumerate() {
                    xd.push_str(&n.print());
                    if idx != l.len() - 1 {
                        xd.push(',');
                    }
                }
                xd.push(']');

                xd
            }
            Value::Simple(num) => format!("{}", num),
        }
    }
}

pub fn solve() {
    let file = match File::open("inputs/13_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();

    let mut ordered = vec![];
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
                if left.unwrap().ordered_recursive(&right.unwrap()).good() {
                    ordered.push(pair_index);
                }
                left = None;
                right = None;
            }
        }
    }
    // 12 -> incorrect
    // 25 -> correct
    // 42 -> correct
    // 44 > incorrect

    if left.is_some() && left.unwrap().ordered_recursive(&right.unwrap()).good() {
        ordered.push(pair_index);
    }

    println!("{:?}, {}", ordered, ordered.iter().sum::<usize>());
}

mod tests {
    use super::*;

    #[test]
    fn base_case_1() {
        let left = Node::parse("[1,2,3,4]");
        let right = Node::parse("[1,2,3,4]");

        assert_eq!(OrderingResult::Same, left.ordered_recursive(&right));
    }

    #[test]
    fn base_case_2() {
        let left = Node::parse("[1,2,3]");
        let right = Node::parse("[1,2,3,4]");

        assert_eq!(OrderingResult::Same, left.ordered_recursive(&right));
    }

    #[test]
    fn base_case_3() {
        let left = Node::parse("[1,2,3,4]");
        let right = Node::parse("[1,2,3]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn base_case_4() {
        let left = Node::parse("[1,5,3]");
        let right = Node::parse("[1,2,3,4]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_0() {
        let left = Node::parse("[5,[4,1]]");
        let right = Node::parse("[5,2,3]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_1() {
        let left = Node::parse("[1,2,3,[4,1]]");
        let right = Node::parse("[1,2,3,4,6]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_2() {
        let left = Node::parse("[1,2,3,[4],1]");
        let right = Node::parse("[1,2,3,5,6]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_3() {
        let left = Node::parse("[1,2,3]");
        let right = Node::parse("[1,[2],3]");

        assert_eq!(OrderingResult::Same, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_4() {
        // [1,[2],3]
        let left = Node::parse("[1,[2],3]");
        let right = Node::parse("[1,[2,3]]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_5() {
        // [1,[2],3]
        let left = Node::parse("[1,2,3]");
        let right = Node::parse("[1,[2,3]]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
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

        assert_eq!(OrderingResult::Same, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_case_woot_3() {
        let left = Node::parse("[]");
        let right = Node::parse("[3]");

        assert_eq!(OrderingResult::Same, left.ordered_recursive(&right));
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

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_case_woot_5() {
        //        let left = Node::parse(" [[ ],    [0] , [[ ]] ]");
        //        let right = Node::parse("[[0],   [[4]],       ]");
        let left = Node::parse("[[],[0],[[]]]");
        let right = Node::parse("[[0],[[4]]]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_case_woot_6() {
        //        let left = Node::parse(" [[ ],    [0] , [[ ]] ]");
        //        let right = Node::parse("[[0],   [[4]],       ]");
        let left = Node::parse("[[],[1],[[[1,3],2,1,3]]]");
        let right = Node::parse("[[[],6,[3,8]],[]]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_12() {
        let left = Node::parse("[[1,[[7,6,3,4],9,[]]],[6],[],[[10],[3,[7,9],[8,0,1,6,7],3,[7,8,4,5]],3],[[4,[8,1,0,7],6]]]");
        let right = Node::parse("[[[1]],[4,5,2,[0]]]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_25() {
        let left = Node::parse("[[[9,[2,4,3]],6],[4,9],[8]]");
        let right = Node::parse("[[10,[],7],[10],[[[1,6],[4,0],9,8],[[6,1,5,6],2],10,5],[],[[8,[8,5],[1,6,6,4,10]],5,8,6]]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_7() {
        let left = Node::parse("[[2,0],[[],[[3,0,6],6,2,6],8,5,[0,[10,0,10,10,8],[4,5,1]]],[[[6,7,0,6,10],[8],[1],6,7],0,6,[10,5,4,[4,2,9],0],[[2,7,8,6,7]]],[[[7,8,6,3],0,[4,3,3,10,8],[4]],[8],2,1,[1,7,[2,3,6],[7,3],9]]]");
        let right = Node::parse("[[6],[5],[],[6,0],[[],[9,[10,5],10,[4,3,0,6,6]]]]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }

    #[test]
    fn parse_xd() {
        let left = Node::parse("[[2,0],3]");

        assert_eq!("[[2,0],3]", left.print());
    }

    #[test]
    fn complex_91() {
        let left = Node::parse("[[],[1],[3],[[[7,8,8,4],0],[7,5],[5,[2,0,5,10],[7]],10],[9,[10,7,[10,1,10,8]],5,7,0]]");
        let right = Node::parse("[[],[],[[],10,[[6],[4,5],[2,2],[7,7]]],[[10,[8]],[[6,9],[3],8],2,[[0,9,1,3],0,5,2,3],[8,5,7,10]]]");

        assert_eq!(OrderingResult::Incorrect, left.ordered_recursive(&right));
    }

    #[test]
    fn complex_79() {
        let left = Node::parse("[[[3],[4,7,1,[2,2,1,8],[1,5]],4,2],[[],3],[],[[[3,8,0,6,5],6,[0]],4]]");
        let right = Node::parse("[[[3,0],[[0,0,10,4],[4,6,4,5,2]],[[7],7,[10,7,2],[2,6,3],6],9]]");

        assert_eq!(OrderingResult::Correct, left.ordered_recursive(&right));
    }
}

/*
13_base
What are the indices of the pairs that are already in the right order?
 (The first pair has index 1, the second pair has index 2, and so on.)
 In the above example, the pairs in the right order are 1, 2, 4, and 6; the sum of these indices is 13.
*/
