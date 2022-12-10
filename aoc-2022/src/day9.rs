use vector2d::Vector2D;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use strum_macros::EnumIter;
use strum::IntoEnumIterator; // 0.17.1


const DEBUG: bool = false;

#[derive(Debug, EnumIter)]
enum Direction {
    Overlap,
    Up,
    Down,
    Right,
    Left,
    RightUp,
    LeftUp,
    LeftDown,
    RightDown,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Overlap => Direction::Overlap,
            // if tail is Up, move tail down
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            // .H
            // ..
            // T.
            // if tails is Right Up, move
            Direction::RightUp => Direction::LeftDown,
            Direction::LeftUp => Direction::RightDown,
            Direction::RightDown => Direction::LeftUp,
            Direction::LeftDown => Direction::RightUp,
        }
    }

    fn delta(&self) -> Vector2D<i32> {
        match self {
            Direction::Overlap => Vector2D::new(0, 0),
            Direction::Up => Vector2D::new(0, 1),
            Direction::Down => Vector2D::new(0, -1),
            Direction::Right => Vector2D::new(1, 0),
            Direction::Left => Vector2D::new(-1, 0),
            Direction::RightUp => Vector2D::new(1, 1),
            Direction::LeftUp => Vector2D::new(-1, 1),
            Direction::RightDown => Vector2D::new(1, -1),
            Direction::LeftDown => Vector2D::new(-1, -1),
        }
    }
}

struct Move {
    direction: Direction,
    times: u32,
}

#[derive(Debug, Copy, Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    // 3 - 4 + 1 - 0
    fn touching(&self, other: &Knot) -> bool {
        for direction in Direction::iter() {
            let delta = direction.delta();
            if self.x + delta.x == other.x && self.y + delta.y == other.y {
                return true;
            }
        }
        return false;
    }

    // from the Head perspective
    fn relative(&self, head: &Knot) -> Direction {
        let tail = self;
        if tail.x == head.x && tail.y == head.y {
            Direction::Overlap
        } else if tail.x == head.x && tail.y > head.y {
            Direction::Up
        } else if tail.x == head.x && tail.y < head.y {
            Direction::Down
        } else if tail.x < head.x && tail.y == head.y {
            Direction::Left
        } else if tail.x > head.x && tail.y == head.y {
            Direction::Right
        } else if tail.x < head.x && tail.y < head.y {
            Direction::LeftDown
        } else if tail.x > head.x && tail.y < head.y {
            Direction::RightDown
        } else if tail.x < head.x && tail.y > head.y {
            Direction::LeftUp
        } else if tail.x > head.x && tail.y > head.y {
            Direction::RightUp
        } else {
            panic!("Unexpected position");
        }
    }
}

fn display(tail: &Knot, head: &Knot, counter: &HashSet<String>) {
    for y in (0..200).rev() {
        for x in 0..200 {
            if tail.x == x && tail.y == y {
                print!("T");
            } else if head.x == x && head.y == y {
                print!("H");
            } else if counter.contains(&format!("{}|{}", x, y).to_string()) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();
}

impl Move {
    fn execute(&self, tail: &mut Knot, head: &mut Knot, counter: &mut HashSet<String>) {
        if DEBUG {
            display(tail, head, &counter);
        }

        counter.insert(format!("{}|{}", tail.x, tail.y));

        for _ in 0..self.times {
            let delta = self.direction.delta();
            head.x += delta.x;
            head.y += delta.y;


            if DEBUG {
                display(tail, head, &counter);
            }

            if tail.touching(head) {
                if DEBUG {
                    display(tail, head, &counter);
                }
                continue;
            }
            if DEBUG {
                println!("Not touching");
            }

            let tail_delta = tail.relative(head).opposite().delta();
            tail.x += tail_delta.x;
            tail.y += tail_delta.y;

            counter.insert(format!("{}|{}", tail.x, tail.y));
            if DEBUG {
                display(tail, head, &counter);
            }
        }
    }

    fn execute_multiple(&self, tails: &mut Vec<Knot>, head: &mut Knot, counter: &mut HashSet<String>) {
        counter.insert(format!("{}|{}", tails.last().unwrap().x, tails.last().unwrap().y));

        let number_of_tails = tails.len();

        for _ in 0..self.times {
            let delta = self.direction.delta();
            head.x += delta.x;
            head.y += delta.y;


            for index in 0..number_of_tails {
                let current_head: Knot = if index == 0 {
                    head.clone()
                } else {
                    tails.get(index - 1).unwrap().clone()
                };

                let mut current_tail = tails.get_mut(index).unwrap();

                if current_tail.touching(&current_head) {
                    continue;
                }

                let tail_delta = current_tail.relative(&current_head).opposite().delta();
                current_tail.x += tail_delta.x;
                current_tail.y += tail_delta.y;

                if index == number_of_tails - 1 {
                    counter.insert(format!("{}|{}", current_tail.x, current_tail.y));
                }
            }
        }
    }

    fn from(letter: &str, times: u32) -> Move {
        Move {
            direction: match letter {
                "D" => Direction::Down,
                "U" => Direction::Up,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!("Unknown move!")
            },
            times: times
        }
    }
}

fn parse(reader: io::Lines<io::BufReader<File>>) -> Vec<Move> {
    let move_regex = Regex::new(r"^(?P<letter>[A-Z]) (?P<times>\d+).*").unwrap();

    let mut moves = Vec::new();
    for line in reader {
        if let Ok(l) = line {
            let captures = move_regex.captures(&l).unwrap();
            let letter = captures.name("letter").unwrap().as_str();
            let times: u32 = captures.name("times").unwrap().as_str().parse().unwrap();

            moves.push(Move::from(letter, times));
        }
    }

    moves
}

fn part1(reader: io::Lines<io::BufReader<File>>) {
    let mut head = Knot { x: 0, y: 0 };
    let mut tail = Knot { x: 0, y: 0 };

    let head_moves = parse(reader);

    let mut counter = HashSet::new();

    for head_move in head_moves {
        head_move.execute(&mut tail, &mut head, &mut counter);
    }


    println!("Head: {:?}, Tail: {:?}, count: {:?}", head, tail, counter.len());
}

fn display_multiple(head: &Knot, tails: &Vec<Knot>) {
    for y in (-10..15).rev() {
        for x in -11..15 {
            let mut somethings_there = false;
             if head.x == x && head.y == y {
                print!("H");
            } else {
                for (index, tail) in tails.iter().enumerate() {
                    if tail.x == x && tail.y == y {
                        print!("{}", index + 1);
                        somethings_there = true;
                        break;
                    }
                }
            } 
                
            if !somethings_there && x == 0 && y == 0 {
                print!("s");
            } else if !somethings_there {
                print!(".");
            }
        }
        println!();
    }
}

fn part2(reader: io::Lines<io::BufReader<File>>) {
    let (mut head, mut tails) = knots_multiple(9);

    let head_moves = parse(reader);

    let mut counter = HashSet::new();

    for head_move in head_moves {
        head_move.execute_multiple(&mut tails, &mut head, &mut counter);
    }

    // display_multiple(&head, &tails);


    println!("Head: {:?}, count: {:?}", head, counter.len());
}

pub fn solve() {
    let file = match File::open("inputs/9_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();
    // part1(reader);
    part2(reader);

}

fn knots() -> (Knot, Knot) {
    let head = Knot { x: 0, y: 0 };
    let tail = Knot { x: 0, y: 0 };

    (head, tail)
}

fn knots_multiple(tails_size: usize) -> (Knot, Vec<Knot>) {
    let head = Knot { x: 0, y: 0 };
    let mut tails: Vec<Knot>  = Vec::new();

    for _ in 0..tails_size {
        tails.push(Knot {
            x: 0,
            y: 0
        })
    }

    (head, tails)
}

mod tests {
    use super::*;

    #[test]
    fn follows_right() {
        let (mut head, mut tail) = knots();

        let head_move = Move {
            direction: Direction::Right,
            times: 4,
        };

        head_move.execute(&mut tail, &mut head, &mut HashSet::new());

        assert_eq!(4, head.x);
        assert_eq!(0, head.y);
        assert_eq!(3, tail.x);
        assert_eq!(0, tail.y);
    }

    #[test]
    fn base_case() {
        let (mut head, mut tail) = knots();
        let mut counter = HashSet::new();
        let head_moves = vec![
            Move {
                direction: Direction::Right,
                times: 4,
            },
            Move {
                direction: Direction::Up,
                times: 4,
            },
            Move {
                direction: Direction::Left,
                times: 3,
            },
            Move {
                direction: Direction::Down,
                times: 1,
            },
            Move {
                direction: Direction::Right,
                times: 4,
            },
            Move {
                direction: Direction::Down,
                times: 1,
            },
            Move {
                direction: Direction::Left,
                times: 5,
            },
            Move {
                direction: Direction::Right,
                times: 2,
            },
        ];
    
        for head_move in head_moves {
            head_move.execute(&mut tail, &mut head, &mut counter);
        }
    
        assert_eq!(2, head.x);
        assert_eq!(2, head.y);
        assert_eq!(1, tail.x);
        assert_eq!(2, tail.y);
        assert_eq!(13, counter.len())
    }

    #[test]
    fn multiple_heads() {
        let (mut head, mut tails) = knots_multiple(3);
        let mut counter = HashSet::new();

        let head_move = Move {
            direction: Direction::Right,
            times: 4,
        };

        head_move.execute_multiple(&mut tails, &mut head, &mut counter);

        // 
        assert_eq!(4, head.x);
        assert_eq!(0, head.y);

        assert_eq!(3, tails.get(0).unwrap().x);
        assert_eq!(0, tails.get(0).unwrap().y);

        assert_eq!(2, tails.get(1).unwrap().x);
        assert_eq!(0, tails.get(1).unwrap().y);

        assert_eq!(1, tails.get(2).unwrap().x);
        assert_eq!(0, tails.get(2).unwrap().y);
    }

    #[test]
    fn multiple_heads_base_case() {
        let (mut head, mut tails) = knots_multiple(10);
        let mut counter = HashSet::new();
        let head_moves = vec![
            Move {
                direction: Direction::Right,
                times: 4,
            },
            Move {
                direction: Direction::Up,
                times: 4,
            },
            Move {
                direction: Direction::Left,
                times: 3,
            },
            Move {
                direction: Direction::Down,
                times: 1,
            },
            Move {
                direction: Direction::Right,
                times: 4,
            },
            Move {
                direction: Direction::Down,
                times: 1,
            },
            Move {
                direction: Direction::Left,
                times: 5,
            },
            Move {
                direction: Direction::Right,
                times: 2,
            },
        ];
    
        for head_move in head_moves {
            head_move.execute_multiple(&mut tails, &mut head, &mut counter);
        }
    
        assert_eq!(2, head.x);
        assert_eq!(2, head.y);
        assert_eq!(1, counter.len())
    }

}

// Head (0, 0)
// Tail (0, 0)
// Up      +(0, 1)
// Down    +(0, -1)
// Right   +(1, 0)
// Left    +(-1, 0)
// RightUp +(1, 1)
// LeftUp  +(-1, 1)
// RightDown +(1, -1)
// LeftDown +(0, -1)

// touching?
// tail.x == head.x
// tail.y == head.y
// tail.x + 1 == head.x
// tail.x - 1 == head.x
// tail.y + 1 == head.y
// tail.y - 1 == head.y

// Where is the Tail, from the Head perspective
// H(0, 0) T(0, 0) -> Overlap tail.x == head.x && tail.y == head.y

// H(0, -1) T(0, 0) -> Up tail.x == head.x && tail.y > head.y
// H(0, 1) T(0, 0) -> Down tail.x == head.x && tail.y < head.y

// H(1, 0) T(0, 0) -> Left tail.x < head.x && tail.y == head.y
// H(-1, 0) T(0, 0) -> Right tail.x > head.x && tail.y == head.y

// H(1, 1) T(0, 0) -> LeftDown tail.x < head.x && tail.y < head.y
// H(-1, 1) T(0, 0) ->  RightDown tail.x > head.x && tail.y < head.y

// H(1, -1) T(0, 0) -> LeftUp   tail.x < head.x && tail.y > head.y
// H(-1, -1) T(0, 0) -> RightUp  tailx > head.x && tail.y > head.y

// is touching, do not move
// we move Head
// determine where we are,
// if touching -> do not move
// else -> move in the counter direction.
