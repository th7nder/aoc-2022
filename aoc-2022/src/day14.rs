use std::cmp::{min, max};
use std::{collections::HashMap, ops::RangeInclusive, fs::File};
use std::io::{self, BufRead};

use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TileType {
    Air,
    Rock,
    Sand,
    Source,
}

struct Grid {
    tiles: HashMap<(i32, i32), TileType>,
    max_tile_y: i32,
    min_tile_x: i32,
    max_tile_x: i32,
}

struct Line {
    x: i32,
    y: i32,
}

impl Line {
    fn new(x: i32, y: i32) -> Line {
        Line { x, y }
    }

    fn parse(str_lines: &str) -> Vec<Line> {
        let dir_regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();

        let mut lines = vec![];
        for str_line in str_lines.split(" -> ") {
            let caps = dir_regex.captures(str_line).unwrap();

            lines.push(Line::new(
                caps.name("x").unwrap().as_str().parse().unwrap(),
                caps.name("y").unwrap().as_str().parse().unwrap(),
            ))
        }

        lines
    }
}

fn range(a: i32, b: i32) -> RangeInclusive<i32> {
    if a <= b {
        a..=b
    } else {
        b..=a
    }
}

impl Grid {
    fn new() -> Grid {
        let mut grid = Grid {
            tiles: HashMap::new(),
            max_tile_y: 0,
            min_tile_x: 500,
            max_tile_x: 500
        };

        grid.tiles.insert((500, 0), TileType::Source);
        grid
    }

    fn insert_tile(&mut self, pos: (i32, i32), tile: TileType) {
        if tile == TileType::Rock {
            self.max_tile_y = max(self.max_tile_y, pos.1);
        }
        if tile == TileType::Sand || tile == TileType::Rock {
            self.min_tile_x = min(self.min_tile_x, pos.0);
            self.max_tile_x = max(self.max_tile_x, pos.0);
        }

        self.tiles.insert(pos, tile);
    }

    fn clean_tile(&mut self, pos: (i32, i32)) {
        // Source can't be cleaned
        self.tiles.insert(pos, TileType::Air);
    }

    // 503,4 -> 502,4 -> 502,9 -> 494,9
    fn insert_lines(&mut self, lines: Vec<Line>) {
        let mut current_line = lines.first().unwrap();
        for idx in 1..lines.len() {
            let next_line = lines.get(idx).unwrap();

            // 503, 4 -> 503, 5
            // 503, 4 -> 503, 6
            // 1..5
            // 5..1
            // join horizontal
            if current_line.x == next_line.x {
                for y in range(current_line.y, next_line.y) {
                    self.insert_tile((current_line.x, y), TileType::Rock);
                }
            } else if current_line.y == next_line.y {
                for x in range(current_line.x, next_line.x) {
                    self.insert_tile((x, current_line.y), TileType::Rock)
                }
            } else {
                panic!("Shouldn't happen.");
            }

            current_line = next_line;
        }
    }

    fn get_tile(&self, pos: &(i32, i32)) -> &TileType {
        match self.tiles.get(&pos) {
            Some(tile) => tile,
            None => {
                if pos.1 == self.max_tile_y + 2 {
                    &TileType::Rock
                } else {
                    &TileType::Air
                }
            },
        }
    }

    fn drop_sand(&mut self) -> (SandState, (i32, i32)) {
        let mut current_pos = (500, 0);

        let mut moved;
        let mut last_state = SandState::Moved;

        while last_state == SandState::Moved {
            moved = false;
            for next_possible_pos in self.next_possible_positions(current_pos) {
                match self.get_tile(&next_possible_pos) {
                    TileType::Air => {
                        self.clean_tile(current_pos);
                        self.insert_tile(next_possible_pos, TileType::Sand);
                        current_pos = next_possible_pos;
                        moved = true;
                        break;
                    },
                    TileType::Rock => continue,
                    TileType::Sand => continue,
                    TileType::Source => panic!("Can't happen"),
                }
            }
            
            last_state = if current_pos.1 >= self.max_tile_y + 2 {
                SandState::Abyss
            } else if moved {
                SandState::Moved
            } else {
                SandState::Settled
            }
        }

        (last_state, current_pos)
    }

    fn next_possible_positions(&self, current_pos: (i32, i32)) -> Vec<(i32, i32)> {
        vec![
            // DOWN
            (current_pos.0, current_pos.1 + 1),
            // LEFT DOWN
            (current_pos.0 - 1, current_pos.1 + 1),
            // RIGHT DOWN
            (current_pos.0 + 1, current_pos.1 + 1)
        ]
    }

    fn part1(&mut self) -> usize {
        let mut finished_sands = 0;
        loop {
            let (state, _) = self.drop_sand();
            if state == SandState::Abyss {
                break;
            } else if state == SandState::Moved {
                panic!("shouldn't happen");
            }
            
            finished_sands += 1;
        }

        finished_sands
    }

    fn part2(&mut self) -> usize {
        let mut finished_sands = 0;
        loop {
            let (state, pos) = self.drop_sand();
            // self.print();
            // println!();
            if state == SandState::Settled && pos.0 == 500 && pos.1 == 0 {
                println!("Settled at the source.");
                break;
            } else if state != SandState::Settled {
                panic!("shouldn't happen");
            }
            
            finished_sands += 1;
        }

        finished_sands + 1
    }

    fn print(&self) {
        for y in 0..=self.max_tile_y + 2 {
            for x in self.min_tile_x - 5..=self.max_tile_x + 5 {
                match self.get_tile(&(x, y)) {
                    TileType::Air => print!("."),
                    TileType::Rock => print!("#"),
                    TileType::Sand => print!("o"),
                    TileType::Source => print!("+"),
                }
            }
            println!();
        }
    }
}


#[derive(PartialEq, Eq, Debug)]
enum SandState {
    Moved,
    Settled,
    Abyss,
}

pub fn solve() {
    let file = match File::open("inputs/14_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file).lines();

    let mut grid = Grid::new();
    for line in reader {
        if let Ok(line) = line {
            grid.insert_lines(Line::parse(line.as_str()))
        }
    }

    let p2 = grid.part2();
    println!("Part2: {}", p2);
}

mod tests {
    use super::*;

    #[test]
    fn line_parser() {
        let lines = Line::parse("503,4 -> 502,4 -> 502,9 -> 494,9");

        assert_eq!(4, lines.len());
        assert_eq!(503, lines.first().unwrap().x);
        assert_eq!(4, lines.first().unwrap().y);
        assert_eq!(502, lines.get(1).unwrap().x);
        assert_eq!(4, lines.get(1).unwrap().y);

    }
    #[test]
    fn sanity() {
        let mut grid = Grid::new();

        grid.insert_lines(vec![
            Line::new(498, 4),
            Line::new(498, 6),
            Line::new(496, 6),
        ]);
        // 503,4 -> 502,4 -> 502,9 -> 494,9
        grid.insert_lines(vec![
            Line::new(503, 4),
            Line::new(502, 4),
            Line::new(502, 9),
            Line::new(494, 9),
        ]);

        assert_eq!(TileType::Source, *grid.get_tile(&(500, 0)));

        assert_eq!(TileType::Rock, *grid.get_tile(&(498, 4)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(498, 5)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(498, 6)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(497, 6)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(496, 6)));

        assert_eq!(TileType::Rock, *grid.get_tile(&(503, 4)));

        for y in 4..=9 {
            assert_eq!(TileType::Rock, *grid.get_tile(&(502, y)));
        }
        for x in 494..=502 {
            assert_eq!(TileType::Rock, *grid.get_tile(&(x, 9)));
        }
    }

    #[test]
    fn sanity_with_parsing() {
        let mut grid = Grid::new();

        grid.insert_lines(Line::parse("498,4 -> 498,6 -> 496,6"));
        grid.insert_lines(Line::parse("503,4 -> 502,4 -> 502,9 -> 494,9"));

        assert_eq!(TileType::Source, *grid.get_tile(&(500, 0)));

        assert_eq!(TileType::Rock, *grid.get_tile(&(498, 4)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(498, 5)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(498, 6)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(497, 6)));
        assert_eq!(TileType::Rock, *grid.get_tile(&(496, 6)));

        assert_eq!(TileType::Rock, *grid.get_tile(&(503, 4)));

        for y in 4..=9 {
            assert_eq!(TileType::Rock, *grid.get_tile(&(502, y)));
        }
        for x in 494..=502 {
            assert_eq!(TileType::Rock, *grid.get_tile(&(x, 9)));
        }
    }

    #[test]
    fn base_case() {
        let mut grid = Grid::new();

        grid.insert_lines(Line::parse("498,4 -> 498,6 -> 496,6"));
        grid.insert_lines(Line::parse("503,4 -> 502,4 -> 502,9 -> 494,9"));

        assert_eq!(24, grid.part1())
    }

    #[test]
    fn base_case_p2() {
        let mut grid = Grid::new();

        grid.insert_lines(Line::parse("498,4 -> 498,6 -> 496,6"));
        grid.insert_lines(Line::parse("503,4 -> 502,4 -> 502,9 -> 494,9"));

        let sands = grid.part2();
        assert_eq!(93, sands)
    }
}
