// dist(position, sensor) <= dist(sensor, closest_beacon) -> can't be in this sensor range
// cant_be = false;
// for position
//    for every sensor:
//      if dist(position, sensor) <= dist(sensor, closest_beacon)
//          cant_be = true;

use std::cmp::{min, max};
use std::collections::HashSet;
use std::{fs::File};
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn manhattan(&self, other: &Position) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)).try_into().unwrap()
    }

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

struct Sensor {
    position: Position,
    closest_beacon: Beacon,
}

struct Beacon {
    position: Position,
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: HashSet<(i32, i32)>,
    min_x_of_any_sensor: i32,
    max_x_of_any_sensor: i32,
}

impl Map {
    fn new() -> Map {
        Map {
            sensors: vec![],
            min_x_of_any_sensor: 0,
            max_x_of_any_sensor: 0,
            beacons: HashSet::new()
        }
    }

    fn add_sensor(&mut self, sensor: Sensor) {
        let distance_to_nearest = sensor.position.manhattan(&sensor.closest_beacon.position);

        self.beacons.insert(sensor.closest_beacon.position.to_tuple());

        self.min_x_of_any_sensor = min(self.min_x_of_any_sensor, sensor.position.x - distance_to_nearest);
        self.max_x_of_any_sensor = max(self.max_x_of_any_sensor, sensor.position.x + distance_to_nearest);

        self.sensors.push(sensor);
    }

    // is reachable by any sensor
    fn is_reachable_by_any_sensor(&self, position: &Position) -> bool {
        for sensor in self.sensors.iter() {
            if position.manhattan(&sensor.position) <= sensor.position.manhattan(&sensor.closest_beacon.position) {
                return true;
            }
        }

        return false;
    }

    fn unavailable_beacon_positions(&self, y: i32) -> usize {
        let mut unavailable_positions = 0;
        println!("Scanning from: {} to: {}", self.min_x_of_any_sensor, self.max_x_of_any_sensor);
        for x in self.min_x_of_any_sensor..=self.max_x_of_any_sensor {
            let current_position = Position::new(x, y);

            let mut cant_be = false;

            if self.beacons.contains(&current_position.to_tuple()) {
                cant_be = false;
            } else {
                if self.is_reachable_by_any_sensor(&current_position) {
                    cant_be = true;
                }
            }

            if cant_be {
                // println!("Can't be at: {:?}", current_position);
                unavailable_positions += 1;
            }
        }

        unavailable_positions
    }


    // this won't work, need to scale it.
    fn can_be(&self, max: i32) -> Position {
        println!("Scanning from: {} to: {}", 0, max);


        for (idx, sensor) in self.sensors.iter().enumerate() {
            let distance = sensor.position.manhattan(&sensor.closest_beacon.position);

            let right_edge = Position::new(sensor.position.x + distance + 1, sensor.position.y);
            let bottom_edge = Position::new(sensor.position.x, sensor.position.y + distance + 1);

            let left_edge = Position::new(sensor.position.x - distance - 1, sensor.position.y);
            let top_edge = Position::new(sensor.position.x, sensor.position.y - distance - 1);

            let mut starting_position = right_edge.clone();
            // Scans bottom-left in the right direction
            while starting_position.x != bottom_edge.x && starting_position.y != bottom_edge.y {
                for x in starting_position.x..=max {
                    let current_position = Position::new(x, starting_position.y);
    
                    if self.beacons.contains(&current_position.to_tuple()) {
                        continue;
                    }
    
                    if self.is_reachable_by_any_sensor(&current_position) {
                        // should we break here?
                        break;
                    }
    
                    return current_position.clone();
                }

        
                starting_position.x -= 1;
                starting_position.y += 1;
                if starting_position.x < 0 || starting_position.x > max || starting_position.y < 0 || starting_position.y > max {
                    break;
                }
                // println!("Sensor: [{}], Down-left, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);
            }
            println!("Sensor: [{}], Down-left, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);


            starting_position = right_edge.clone();
            // Scans top-left, in the right direction
            while starting_position.x != top_edge.x && starting_position.y != top_edge.y {
                for x in starting_position.x..=max {
                    let current_position = Position::new(x, starting_position.y);
    
                    if self.beacons.contains(&current_position.to_tuple()) {
                        continue;
                    }
    
                    if self.is_reachable_by_any_sensor(&current_position) {
                        // should we break here?
                        break;
                    }
    
                    return current_position.clone();
                }

        
                starting_position.x -= 1;
                starting_position.y -= 1;
                if starting_position.x < 0 || starting_position.x > max || starting_position.y < 0 || starting_position.y > max {
                    break;
                }
                // println!("Sensor: [{}], Up-left, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);
            }
            println!("Sensor: [{}], Up-left, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);

            starting_position = left_edge.clone();
            // Scans up-right, in the left direction
            while starting_position.x != top_edge.x && starting_position.y != top_edge.y {
                for x in (0..=starting_position.x).rev() {
                    let current_position = Position::new(x, starting_position.y);
    
                    if self.beacons.contains(&current_position.to_tuple()) {
                        continue;
                    }
    
                    if self.is_reachable_by_any_sensor(&current_position) {
                        // should we break here?
                        break;
                    }
    
                    return current_position.clone();
                }

        
                starting_position.x += 1;
                starting_position.y -= 1;
                if starting_position.x < 0 || starting_position.x > max || starting_position.y < 0 || starting_position.y > max {
                    break;
                }
                // println!("Sensor: [{}], Up-right, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);
            }
            println!("Sensor: [{}], Up-right, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);


            starting_position = left_edge.clone();
            // Scans down-right, in the left direction
            while starting_position.x != bottom_edge.x && starting_position.y != bottom_edge.y {
                for x in (0..=starting_position.x).rev() {
                    let current_position = Position::new(x, starting_position.y);
    
                    if self.beacons.contains(&current_position.to_tuple()) {
                        continue;
                    }
    
                    if self.is_reachable_by_any_sensor(&current_position) {
                        // should we break here?
                        break;
                    }
    
                    return current_position.clone();
                }

        
                starting_position.x += 1;
                starting_position.y += 1;
                if starting_position.x < 0 || starting_position.x > max || starting_position.y < 0 || starting_position.y > max {
                    break;
                }
                // println!("Sensor: [{}], Down-right, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);
            }
            println!("Sensor: [{}], Down-right, Starting {:?}, vs: {:?}", idx, starting_position, bottom_edge);


        }

        panic!("Not found....");
    }


    fn from_file(inputfile: &str) -> Map {
        let file = match File::open(inputfile) {
            Err(why) => panic!("Couldn't open file {}", why),
            Ok(file) => file,
        };
    
        let reader = io::BufReader::new(file).lines();


        let mut map = Map::new();
    
        for line in reader {
            if let Ok(line) = line {
                map.add_sensor(Sensor::parse(line.as_str()));
            }
        }
    
        map
    }
}

impl Sensor {
    fn parse(str_line: &str) -> Sensor {
        let regex = Regex::new(r"^Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)$").unwrap();
        let caps = regex.captures(str_line).unwrap();

        Sensor {
            position: Position {
                x: caps.name("sensor_x").unwrap().as_str().parse().unwrap(),
                y: caps.name("sensor_y").unwrap().as_str().parse().unwrap(),
            },
            closest_beacon: Beacon {
                position: Position {
                    x: caps.name("beacon_x").unwrap().as_str().parse().unwrap(),
                    y: caps.name("beacon_y").unwrap().as_str().parse().unwrap(),
                },
            },
        }
    }
}

pub fn solve() {
    let map = Map::from_file("inputs/15_input");

    // println!("Part 1: {}", map.unavailable_beacon_positions(2000000));
    println!("Part 2: {:?}", map.can_be(4000000));
}

mod tests {
    use super::*;

    #[test]
    fn parse() {
        let sensor = Sensor::parse("Sensor at x=2, y=18: closest beacon is at x=-2, y=15");

        assert_eq!(2, sensor.position.x);
        assert_eq!(18, sensor.position.y);
        assert_eq!(-2, sensor.closest_beacon.position.x);
        assert_eq!(15, sensor.closest_beacon.position.y);
    }

    #[test]
    fn manhattan() {
        let sensor = Position::new(8,7);
        let beacon = Position::new(2,10);
        let point = Position::new(-1, 7);

        assert_eq!(9, sensor.manhattan(&beacon));
        assert_eq!(9, sensor.manhattan(&point));
    }

    #[test]
    fn base() {
        let map = Map::from_file("inputs/15_base");

        assert_eq!(26, map.unavailable_beacon_positions(10));
    }

    #[test]
    fn base_part2() {
        let map = Map::from_file("inputs/15_base");

        let pos = map.can_be(20);
        assert_eq!(14, pos.x);
        assert_eq!(11, pos.y);
    }
}
