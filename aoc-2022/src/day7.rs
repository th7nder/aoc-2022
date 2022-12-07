use regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::rc::Rc;

#[derive(Debug)]
struct Entry {
    name: String,
    entries: Vec<Rc<RefCell<Entry>>>,
    size: usize,
}

impl Entry {
    fn print(&self) -> String {
        if self.entries.len() == 0 {
            return self.name.clone();
        } else {
            return String::from("[")
                + &self.name
                + ": "
                + &self
                    .entries
                    .iter()
                    .map(|tn| tn.borrow().name.clone())
                    .collect::<Vec<String>>()
                    .join(",")
                + "]";
        }
    }
}

fn calculate_directory_sizes(cell: Rc<RefCell<Entry>>, part1: &mut usize) -> usize {
    let mut entry = cell.borrow_mut();

    let mut calculated_size = 0;
    for entry in &entry.entries {
        calculated_size += calculate_directory_sizes(Rc::clone(entry), part1);
    }

    if entry.entries.len() > 0 {
        entry.size = calculated_size;

        println!("name: {}, size: {}", entry.name, entry.size);
        if entry.size <= 100000 {
            *part1 += entry.size;
        }
    }

    return entry.size;
}

fn parse(reader: io::Lines<io::BufReader<File>>) -> Rc<RefCell<Entry>> {
    let mut stack = VecDeque::new();

    let cd_regex = Regex::new(r"\$ cd (?P<directory>.*)").unwrap();
    let dir_regex = Regex::new(r"dir (?P<directory>.*)").unwrap();
    let file_regex = Regex::new(r"(?P<filesize>\d+) (?P<filename>.*)").unwrap();

    let root = Rc::new(RefCell::new(Entry {
        name: "/".to_string(),
        entries: vec![],
        size: 0,
    }));

    let mut next_current_dir: Option<Rc<RefCell<Entry>>> = None;
    let mut current_dir = root.clone();

    let mut browsing = false;
    for line in reader {
        if let Ok(l) = line {
            if l == "" || l.starts_with("$ cd /") {
                continue;
            }
            if l.starts_with("$ ls") {
                browsing = true;
                continue;
            }
            if browsing && l.starts_with("$") {
                browsing = false;
            }

            match next_current_dir {
                Some(ref next) => {
                    current_dir = Rc::clone(&next);
                    next_current_dir = None;
                }
                None => {},
            }

            if browsing {
                if l.starts_with("dir") {
                    let cap = dir_regex.captures(&l).unwrap();
                    let dir_name = cap.name("directory").unwrap().as_str();

                    let mut directory = current_dir.borrow_mut();

                    // maybe they can repeat itself
                    directory.entries.push(Rc::new(RefCell::new(Entry {
                        name: dir_name.to_string(),
                        entries: vec![],
                        size: 0,
                    })));
                } else {
                    let cap = file_regex.captures(&l).unwrap();
                    let file_name = cap.name("filename").unwrap().as_str();
                    let file_size: usize = cap.name("filesize").unwrap().as_str().parse().unwrap();

                    let mut directory = current_dir.borrow_mut();

                    // maybe they can repeat itself
                    directory.entries.push(Rc::new(RefCell::new(Entry {
                        name: file_name.to_string(),
                        entries: vec![],
                        size: file_size,
                    })));
                }
            }

            if l.starts_with("$ cd") {
                println!("Xd: {}", l);
                let cap = cd_regex.captures(&l).unwrap();
                let dir_name = cap.name("directory").unwrap().as_str();

                if dir_name == ".." {
                    current_dir = stack.pop_back().unwrap();
                } else {
                    let mut directory = current_dir.borrow();

                    for nested_dir in &directory.entries {
                        let x = nested_dir.borrow();
                        if x.name == dir_name {
                            stack.push_back(Rc::clone(&current_dir));

                            next_current_dir = Some(Rc::clone(nested_dir));
                        }
                    }
                }
            }
        }
    }
    return root;
}

pub fn solve() {
    let file = match File::open("inputs/7_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();
    let root = parse(reader);

    println!("x: {}", root.borrow().print());

    let mut part1: usize = 0;
    calculate_directory_sizes(root, &mut part1);

    println!("{}", part1);
}

mod tests {
    use super::*;

    #[test]
    fn sanity_algorithm() {
        let root = Rc::new(RefCell::new(Entry {
            name: "/".to_string(),
            entries: vec![
                Rc::new(RefCell::new(Entry {
                    name: "a".to_string(),
                    entries: vec![
                        Rc::new(RefCell::new(Entry {
                            name: "e".to_string(),
                            entries: vec![Rc::new(RefCell::new(Entry {
                                name: "i".to_string(),
                                entries: vec![],
                                size: 584,
                            }))],
                            size: 0,
                        })),
                        Rc::new(RefCell::new(Entry {
                            name: "f".to_string(),
                            entries: vec![],
                            size: 29116,
                        })),
                        Rc::new(RefCell::new(Entry {
                            name: "g".to_string(),
                            entries: vec![],
                            size: 2557,
                        })),
                        Rc::new(RefCell::new(Entry {
                            name: "h.lst".to_string(),
                            entries: vec![],
                            size: 62596,
                        })),
                    ],
                    size: 0,
                })),
                Rc::new(RefCell::new(Entry {
                    name: "b.txt".to_string(),
                    entries: vec![],
                    size: 14848514,
                })),
            ],
            size: 0,
        }));
        let mut part1: usize = 0;
        calculate_directory_sizes(root, &mut part1);
        assert_eq!(95437, part1);
    }
}
