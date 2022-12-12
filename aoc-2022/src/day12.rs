use std::cmp::min;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
    height: i32,
    letter: char,
}

impl Node {
    fn new(x: i32, y: i32, letter: char) -> Node {
        let a_ascii = 97;
        match letter {
            'E' => Node {
                x, y, letter, height: ('z' as u32 - a_ascii).try_into().unwrap(),
            },
            'S' => Node {
                x, y, letter, height: 0,
            },
            rest =>  Node {
                x,
                y,
                letter: rest,
                height: (rest as u32 - a_ascii).try_into().unwrap(),
            }
        }
       
    }

    fn is_end(&self) -> bool {
        self.letter == 'E'
    }

    fn is_start(&self) -> bool {
        self.letter == 'S'
    }

}


#[derive(Debug)]
struct HeightMap {
    width: i32,
    height: i32,
    
    nodes: Vec<Vec<Node>>,
    start: (i32, i32),
    end: (i32, i32),

}

impl HeightMap {
    fn get(&self, x: i32, y: i32) -> &Node {
        self.nodes.get(y as usize).unwrap().get(x as usize).unwrap()
    }
}

fn parse(reader: io::Lines<io::BufReader<File>>) -> HeightMap {
    let mut nodes = Vec::new();
    let mut x = 0;
    let mut y = 0;

    let mut end = (0, 0);
    let mut start = (0, 0);
    for line in reader {
        x = 0;
        if let Ok(l) = line {
            let mut row = Vec::new();

            for letter in l.chars() {
                let node = Node::new(
                    x,
                    y,
                    letter
                );
                if node.is_end() {
                    end = (x, y);
                } 
                if node.is_start() {
                    start = (x, y);
                }
                row.push(node);
                x += 1;
            }

            nodes.push(row);
            y += 1;
        }
    }

    HeightMap { width: x, height: y, nodes, start, end }
}


fn load(path: &str) -> HeightMap {
    let file = match File::open(path) {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();

    parse(reader)
}


fn find_shortest_path(hm: &HeightMap, start: (i32, i32)) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back(hm.get(start.0, start.1));

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();

    let mut level = 0;
    while queue.len() > 0 {
        let mut level_size = queue.len();

        while level_size > 0 {
            let node = queue.pop_front().unwrap();
            // println!("visiting node: {:?}", node);
            level_size -= 1;

            if visited.contains(&(node.x, node.y)) {
                continue;
            }

            visited.insert((node.x, node.y));

            for dir in vec![(1,0), (-1, 0), (0, -1), (0, 1)] {
                let next_pos = (node.x + dir.0, node.y + dir.1);
                // println!("checking posi: {:?}", next_pos);
                if next_pos.0 >= 0 && next_pos.0 < hm.width 
                    && next_pos.1 >= 0 && next_pos.1 < hm.height {
                    let next_node = hm.get(next_pos.0, next_pos.1);
                    
                    // println!("next_node: {:?}", next_node);
                    if next_node.height - node.height > 1 {
                        continue;
                    }

                    queue.push_back(next_node);

                    // println!("inserting: {:?}", next_pos);
                    distances.insert(
                        next_pos,
                        min(level + 1, *distances.get(&next_pos).unwrap_or(&999999))
                    );
                }
            }
        }
        level += 1;
    }

    
    return *distances.get(&(hm.end.0, hm.end.1)).unwrap_or(&999999)
}

pub fn solve() {
    let hm = load("inputs/12_input");

    let res = find_shortest_path(&hm, hm.start);
    println!("res: {}", res);

    let mut distances = Vec::new();
    for y in 0..hm.height {
        for x in 0..hm.width {
            let start = (x, y);
            let start_node = hm.get(x, y);
            if start_node.height > 0 {
                continue;
            }

            distances.push(find_shortest_path(&hm, start));
        }
    }

    distances.sort();
    println!("{:?}", distances);
}

mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let hm = load("inputs/12_base");

        assert_eq!((0, 0), hm.start);
        assert_eq!((5, 2), hm.end);
        assert_eq!(8, hm.width);
        assert_eq!(5, hm.height);
        assert_eq!(0, hm.get(hm.start.0, hm.start.1).height);
        assert_eq!(25, hm.get(hm.end.0, hm.end.1).height);
    }

    #[test]
    fn base() {
        let hm = load("inputs/12_base");

        let res = find_shortest_path(&hm, hm.start);
        assert_eq!(31, res);
    }
}