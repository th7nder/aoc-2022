use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Tree {
    x: usize,
    y: usize,
    height: u32,
}

struct Grid {
    trees: Vec<Vec<Tree>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn print(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                print!("{}", self.trees.get(y).unwrap().get(x).unwrap().height)
            }
            println!();
        }
    }
}

fn parse(reader: io::Lines<io::BufReader<File>>) -> Grid {
    let mut grid: Vec<Vec<Tree>> = Vec::new();

    let mut current_y: usize = 0;
    let mut current_x: usize = 0;
    for line in reader {
        if let Ok(l) = line {
            let mut row: Vec<Tree> = Vec::new();
            current_x = 0;

            for char in l.chars() {
                row.push(Tree {
                    x: current_x,
                    y: current_y,
                    height: char.to_digit(10).unwrap(),
                });

                current_x += 1;
            }

            grid.push(row);
        }

        current_y += 1;
    }

    return Grid {
        trees: grid,
        max_x: current_x,
        max_y: current_y,
    };
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Top,
    Down,
}

fn determine_max_height(tree: &Tree, grid: &Grid, direction: Direction) -> u32 {
    return match direction {
        Direction::Left => {
            if tree.x == 0 {
                tree.height
            } else {
                max(
                    tree.height,
                    determine_max_height(
                        &grid.trees.get(tree.y).unwrap().get(tree.x - 1).unwrap(),
                        grid,
                        direction,
                    ),
                )
            }
        }
        Direction::Right => {
            if tree.x == grid.max_x - 1 {
                tree.height
            } else {
                max(
                    tree.height,
                    determine_max_height(
                        &grid.trees.get(tree.y).unwrap().get(tree.x + 1).unwrap(),
                        grid,
                        direction,
                    ),
                )
            }
        }
        Direction::Down => {
            if tree.y == grid.max_y - 1 {
                tree.height
            } else {
                max(
                    tree.height,
                    determine_max_height(
                        &grid.trees.get(tree.y + 1).unwrap().get(tree.x).unwrap(),
                        grid,
                        direction,
                    ),
                )
            }
        }
        Direction::Top => {
            if tree.y == 0 {
                tree.height
            } else {
                max(
                    tree.height,
                    determine_max_height(
                        &grid.trees.get(tree.y - 1).unwrap().get(tree.x).unwrap(),
                        grid,
                        direction,
                    ),
                )
            }
        }
    };
}

fn visible_trees(tree: &Tree, grid: &Grid, direction: Direction, stop: u32) -> u32 {
    println!("Going {:?} from {:?}", direction, tree);
    return match direction {
        Direction::Left => {
            if tree.x == 0 {
                0
            } else {
                let next_tree = grid.trees.get(tree.y).unwrap().get(tree.x - 1).unwrap();
                if next_tree.height >= stop {
                    return 1
                } else {
                    let neighbour = visible_trees(
                        next_tree,
                        grid,
                        direction,
                        stop
                    );

                    return 1 + neighbour;
                }
            }
        }
        Direction::Right => {
            if tree.x == grid.max_x - 1 {
                0
            } else {
                let next_tree = grid.trees.get(tree.y).unwrap().get(tree.x + 1 ).unwrap();
                if next_tree.height >= stop {
                    return 1
                } else {
                    return 1 + visible_trees(
                        next_tree,
                        grid,
                        direction,
                        stop
                    );
                }
            }
        }
        Direction::Down => {
            if tree.y == grid.max_y - 1 {
                0
            } else {
                let next_tree = grid.trees.get(tree.y + 1).unwrap().get(tree.x).unwrap();
                if next_tree.height >= stop {
                    return 1
                } else {
                    return 1 + visible_trees(
                        next_tree,
                        grid,
                        direction,
                        stop
                    );
                }
            }
        }
        Direction::Top => {
            if tree.y == 0 {
                0
            } else {
                let next_tree = grid.trees.get(tree.y - 1).unwrap().get(tree.x).unwrap();
                if next_tree.height >= stop {
                    return 1
                } else {
                    return 1 + visible_trees(
                        next_tree,
                        grid,
                        direction,
                        stop
                    );
                }
            }
        }
    };
}

fn visibility_score(tree: &Tree, grid: &Grid) -> u32 {
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;
    if tree.x > 0 {
        let next_tree = grid.trees.get(tree.y).unwrap().get(tree.x - 1).unwrap();
        if next_tree.height >= tree.height {
            left = 1;
        } else {
            left = 1 + visible_trees(
                next_tree,
                grid,
                Direction::Left,
                tree.height
            );
        }
    }
    if tree.x < grid.max_x - 1 {
        let next_tree = grid.trees.get(tree.y).unwrap().get(tree.x + 1).unwrap();
        if next_tree.height >= tree.height {
            right = 1;
        } else {
            right = 1 + visible_trees(
                next_tree,
                grid,
                Direction::Right,
                tree.height
            );
        }
    }
    if tree.y > 0 {
        let next_tree = grid.trees.get(tree.y + 1).unwrap().get(tree.x).unwrap();
        if next_tree.height >= tree.height {
            bottom = 1;
        } else {
            bottom = 1 + visible_trees(
                next_tree,
                grid,
                Direction::Down,
                tree.height
            );
        }
    }
    if tree.y < grid.max_y - 1 {
        let next_tree = grid.trees.get(tree.y - 1).unwrap().get(tree.x).unwrap();
        if next_tree.height >= tree.height {
            top = 1;
        } else {
            top = 1 + visible_trees(
                next_tree,
                grid,
                Direction::Top,
                tree.height
            );
        }
    }

    println!(
        "SELF: {:?} |||||||||||| Left: {}, Right: {}, Top: {}, Bottom: {}",
        tree,
        left, right, top, bottom
    );


    return left * right * top * bottom;
}


fn visible_outside(tree: &Tree, grid: &Grid) -> bool {
    if tree.x == 0 || tree.x == grid.max_x - 1 || tree.y == 0 || tree.y == grid.max_y - 1 {
        return true;
    }

    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;
    if tree.x > 0 {
        left = determine_max_height(
                &grid.trees.get(tree.y).unwrap().get(tree.x - 1).unwrap(),
                grid,
                Direction::Left,
            );
    }
    if tree.x < grid.max_x - 1 {
        right = determine_max_height(
                &grid.trees.get(tree.y).unwrap().get(tree.x + 1).unwrap(),
                grid,
                Direction::Right,
            );
    }
    if tree.y > 0 {
        top = determine_max_height(
                &grid.trees.get(tree.y - 1).unwrap().get(tree.x).unwrap(),
                grid,
                Direction::Top,
            );
    }
    if tree.y < grid.max_y - 1 {
        bottom = determine_max_height(
                &grid.trees.get(tree.y + 1).unwrap().get(tree.x).unwrap(),
                grid,
                Direction::Down,
            );
    }

    // println!(
    //     "SELF: {:?} |||||||||||| Left: {}, Right: {}, Top: {}, Bottom: {}",
    //     tree,
    //     left, right, top, bottom
    // );

    return tree.height > left || tree.height > right || tree.height > top || tree.height > bottom;
}

fn part1(grid: &Grid) {
    let mut visible = 0;
    let mut edge_visible = 0;
    for row in &grid.trees {
        for tree in row {
            if tree.x == 0 || tree.x == grid.max_x - 1 || tree.y == 0 || tree.y == grid.max_y - 1 {
                edge_visible += 1;
            } else if visible_outside(tree, grid) {
                println!("Visible tree: {:?}", tree);
                visible += 1;
            }
        }
    }

    println!("Visible: {}, edge_visible: {}", visible, edge_visible);
    println!("Result: {}", visible + edge_visible);
}

fn part2(grid: &Grid) {
    let mut max_score = 0;
    for row in &grid.trees {
        for tree in row {
            if tree.x == 0 || tree.x == grid.max_x - 1 || tree.y == 0 || tree.y == grid.max_y - 1 {
                continue;
            } else {
                let score = visibility_score(tree, grid);
                max_score = max(max_score, score);
                
                println!("score: {}, tree: {:?}", score, tree);
            }
        }
    }

    println!("Score: {}", max_score);
}


pub fn solve() {
    let file = match File::open("inputs/8_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();
    let grid = parse(reader);
    grid.print();

    // part1(&grid);
    part2(&grid);
}

mod tests {
    use super::*;

    fn grid() -> Grid {
        let file = match File::open("inputs/8_input_sanity") {
            Err(why) => panic!("Couldn't open file {}", why),
            Ok(file) => file,
        };

        let reader = io::BufReader::new(file).lines();
        let grid = parse(reader);
        return grid;
    }

    fn grid_big() -> Grid {
        let file = match File::open("inputs/8_input") {
            Err(why) => panic!("Couldn't open file {}", why),
            Ok(file) => file,
        };

        let reader = io::BufReader::new(file).lines();
        let grid = parse(reader);
        return grid;
    }

    #[test]
    fn visible_edge() {
        let grid = grid();

        assert_eq!(
            true,
            visible_outside(grid.trees.get(0).unwrap().get(0).unwrap(), &grid)
        );
        assert_eq!(
            true,
            visible_outside(
                grid.trees.get(grid.max_y - 1).unwrap().get(0).unwrap(),
                &grid
            )
        );
        assert_eq!(
            true,
            visible_outside(
                grid.trees
                    .get(grid.max_y - 1)
                    .unwrap()
                    .get(grid.max_x - 1)
                    .unwrap(),
                &grid
            )
        );
        assert_eq!(
            true,
            visible_outside(
                grid.trees.get(0).unwrap().get(grid.max_x - 1).unwrap(),
                &grid
            )
        );
    }

    #[test]
    fn not_visible_near_the_edge() {
        let grid = grid();

        assert_eq!(
            false,
            visible_outside(grid.trees.get(1).unwrap().get(3).unwrap(), &grid)
        );
        assert_eq!(
            false,
            visible_outside(grid.trees.get(2).unwrap().get(2).unwrap(), &grid)
        );
    }

    #[test]
    fn not_visible_in_the_middle() {
        let grid = grid();

        assert_eq!(
            false,
            visible_outside(grid.trees.get(2).unwrap().get(2).unwrap(), &grid)
        );
    }

    #[test]
    fn visible_in_the_top_left() {
        let grid = grid();

        assert_eq!(
            true,
            visible_outside(grid.trees.get(1).unwrap().get(1).unwrap(), &grid)
        );
    }

    #[test]
    fn visible_in_the_top_middle() {
        let grid = grid();

        assert_eq!(
            true,
            visible_outside(grid.trees.get(1).unwrap().get(2).unwrap(), &grid)
        );
    }


    #[test]
    fn visibility_score_top_mid() {
        let grid = grid();

        assert_eq!(
            4,
            visibility_score(grid.trees.get(1).unwrap().get(2).unwrap(), &grid)
        );
    }

    #[test]
    fn visibility_score_bottom_mid() {
        let grid = grid();

        assert_eq!(
            8,
            visibility_score(grid.trees.get(3).unwrap().get(2).unwrap(), &grid)
        );
    }

    #[test]
    fn visibility_score_next_1() {
        let grid = grid();

        assert_eq!(
            1,
            visibility_score(grid.trees.get(3).unwrap().get(1).unwrap(), &grid)
        );
    }

    #[test]
    fn visibility_score_next_2() {
        let grid = grid();

        assert_eq!(
            1,
            visibility_score(grid.trees.get(2).unwrap().get(2).unwrap(), &grid)
        );
    }



    #[test]
    fn visibility_score_gird_big() {
        let grid = grid_big();

        println!("{:?}", grid.trees.get(77).unwrap());
        assert_eq!(
            1,
            visibility_score(grid.trees.get(77).unwrap().get(43).unwrap(), &grid)
        );
    }
}
