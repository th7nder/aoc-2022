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

pub fn solve() {
    let file = match File::open("inputs/8_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();
    let grid = parse(reader);
    grid.print();

    part1(&grid);
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
}
