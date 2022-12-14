use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{hash_map, HashSet, VecDeque};
use std::rc::Rc;
use std::{collections::HashMap};
use std::{fs::File};
use std::io::{self, BufRead};

use regex::Regex;

pub fn solve() {
    let volcano = Volcano::from_file("inputs/16_input");

    println!("Starting volcano: {:?}", volcano);
    volcano.start_with_elephant();
}


#[derive(Debug)]
struct Volcano {
    valves: HashMap<String, Valve>,
    max_pressure: Rc<RefCell<i32>>,
}

impl Volcano {
    fn from_file(inputfile: &str) -> Volcano {
        let file = match File::open(inputfile) {
            Err(why) => panic!("Couldn't open file {}", why),
            Ok(file) => file,
        };
    
        let reader = io::BufReader::new(file).lines();


        let mut valves: HashMap<String, Valve> = HashMap::new();
    
        for line in reader {
            if let Ok(line) = line {
                let regex = Regex::new(r"^Valve (?P<valve_name>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (.*)$").unwrap();
                let caps = regex.captures(&line).unwrap();

                let valve_name = caps.name("valve_name").unwrap().as_str().to_string();
                let flow_rate: i32 = caps.name("flow_rate").unwrap().as_str().parse().unwrap();

                let valves_names = caps.get(3).unwrap().as_str().split(", ");

                let valve = match valves.entry(valve_name.clone()) {
                    hash_map::Entry::Occupied(o) => o.into_mut(),
                    hash_map::Entry::Vacant(v) => v.insert(Valve::new(valve_name, flow_rate)),
                };
                for valve_name in valves_names {
                    valve.connected_to.push(valve_name.to_string());

                }
            }
        }
    
        Volcano { valves, max_pressure: Rc::new(RefCell::new(0)) }
    }

    fn start(&self) {
        let opened: HashSet<&str> = HashSet::new();
        self.simulate("AA", opened, 1, 0, vec![]);

        println!("Max pressure: {}", *self.max_pressure.borrow_mut());
    }


    fn start_with_elephant(&self) {
        let opened: HashSet<&str> = HashSet::new();
        self.simulate_with_elephant("AA", "AA", opened, 5, 5, 0);

        println!("Max pressure: {}", *self.max_pressure.borrow_mut());
    }


    fn simulate(&self, current_valve: &str, opened: HashSet<&str>, current_minute: i32, max_pressure: i32, path: Vec<(&str, i32, i32)>) {
        let maximum_pressures = self.calculate_shortest_paths(
            current_valve, 
            current_minute, 
            opened.clone()
        );

        if maximum_pressures.len() == 0 {
            let mut volcano_pressure = self.max_pressure.borrow_mut();
            if max_pressure > *volcano_pressure {
                *volcano_pressure = max_pressure;
            }
            // println!("Can't go nowhere, {}, {:?}, path: {:?}", max_pressure, current_minute, path);
        }

        for (valve, (needed_minutes, pressure)) in &maximum_pressures {
            let mut opened = opened.clone();
            opened.insert(*valve);
            if current_minute + needed_minutes < 30 {
                let mut path = path.clone();
                path.push((valve, *needed_minutes, *pressure));
                self.simulate(valve, opened, current_minute + needed_minutes, max_pressure + pressure, path.clone());
            }
        }
    }

    fn simulate_with_elephant(&self, current_valve: &str, current_elephant: &str, opened: HashSet<&str>, my_minute: i32, elephants_minute: i32, max_pressure: i32) {
        let maximum_pressures = self.calculate_shortest_paths(
            current_valve, 
            my_minute, 
            opened.clone()
        );
        let maximum_pressures_elephant = self.calculate_shortest_paths(
            current_elephant, 
            elephants_minute, 
            opened.clone()
        );

        if maximum_pressures.len() == 0 && maximum_pressures_elephant.len() == 0 {
            let mut volcano_pressure = self.max_pressure.borrow_mut();
            if max_pressure > *volcano_pressure {
                println!("Can't go nowhere, {}, my: {}, elephants: {:?}", max_pressure, my_minute, elephants_minute);
                *volcano_pressure = max_pressure;
            }
        }

        // we need to get the shortest for us
        // then shortest for elephant
        // and try all of the combinations
        let mut xd_pressures: Vec<(&str, i32, i32, i32)> = vec![];
        for (valve, (minutes, my_pressure)) in &maximum_pressures {
            xd_pressures.push((*valve, *minutes, *my_pressure, my_pressure / minutes));            
        }
        xd_pressures.sort_by(|a, b| b.3.cmp(&a.3));

        for (idx, (my_valve, my_needed_minutes, my_pressure, _)) in xd_pressures.iter().enumerate() {
            let mut opened = opened.clone();
            if idx >= 3 {
                continue;
            }
            opened.insert(*my_valve);

            for (elephant_valve, (elephant_needed_minutes, elephant_pressure)) in &maximum_pressures_elephant {
                if opened.contains(elephant_valve) || *elephant_needed_minutes > 6 {
                    continue;
                }
                let mut opened = opened.clone();
                opened.insert(&elephant_valve);

                let mut max_pressure = max_pressure;


                let mut next_my_valve = current_valve;
                let mut next_my_minute = my_minute;
                if my_minute + my_needed_minutes < 30 {
                    max_pressure += my_pressure;
                    next_my_valve = my_valve;
                    next_my_minute += my_needed_minutes;
                }
                let mut next_elephant_valve = current_elephant;
                let mut next_elephant_minutes = elephants_minute;
                if elephants_minute + elephant_needed_minutes < 30 {
                    max_pressure += elephant_pressure;
                    next_elephant_valve = elephant_valve;
                    next_elephant_minutes += elephant_needed_minutes;
                }
            

                self.simulate_with_elephant(next_my_valve, next_elephant_valve, opened, next_my_minute, next_elephant_minutes, max_pressure)
            }
        }
    }

    // how abouttt
    // we know how much pressure will each valve release and 
    // let's take every possible solution XD 
    // basically, calculate the shortest paths at each step and try em all

    fn calculate_shortest_paths(&self, current_valve: &str, starting_minute: i32, opened: HashSet<&str>) -> HashMap<&str, (i32, i32)> {
        let current_valve = self.valves.get(current_valve).unwrap();

        let mut visited: HashSet<&str> = HashSet::new();
        let mut queue: VecDeque<&str> = VecDeque::new();
        queue.push_back(&current_valve.name);

        let max_minutes = 30;

        let mut current_minute = starting_minute;
        let mut maximum_pressures: HashMap<&str, (i32, i32)> = HashMap::new();

        let mut everything_open = true;
        for (name, valve) in &self.valves {
            if valve.flow_rate != 0 && !opened.contains(name.as_str()) {
                everything_open = false;
            }
        }
        if everything_open {
            return maximum_pressures;
        }


        // this algorithm just takes the shortest path with the maximum reward
        // it doesnt take time into dimension.
        while queue.len() > 0 {
            if current_minute == 30 {
                break;
            }
            let mut valves_to_process = queue.len();

            while valves_to_process > 0 {
                let current_valve = self.valves.get(queue.pop_front().unwrap()).unwrap();

                visited.insert(&current_valve.name);
                
                // let's traverse the graph and find the valve with the maximum flow rate
                if !opened.contains(current_valve.name.as_str()) {
                    let minutes_after_opening = max_minutes - current_minute;
                    let current_possible_pressure = current_valve.flow_rate * minutes_after_opening;
                    if current_possible_pressure > maximum_pressures.get(current_valve.name.as_str()).or(Some(&(0, 0))).unwrap().1 {
                        maximum_pressures.insert(&current_valve.name, (current_minute - starting_minute + 1, current_possible_pressure));
                    }
                }

                for neighbour in &current_valve.connected_to {
                    if visited.contains(neighbour.as_str()) {
                       continue; 
                    }
                    queue.push_back(neighbour);
                }

                valves_to_process -= 1;
            }

            current_minute += 1;
        }

        // println!("Maximum pressure: {:?}", maximum_pressures);
        maximum_pressures
    }

}


#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    connected_to: Vec<String>
}

impl Valve {
    fn new(name: String, flow_rate: i32) -> Valve {
        Valve {
            name,
            flow_rate,
            connected_to: vec![]
        }
    }
}



mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let volcano = Volcano::from_file("inputs/16_base");

        println!("Starting volcano: {:?}", volcano);
        volcano.start();
        assert!(volcano.valves.len() > 0)
    }

    #[test]
    fn sanity_part2() {
        let volcano = Volcano::from_file("inputs/16_base");

        println!("Starting volcano: {:?}", volcano);
        volcano.start_with_elephant();
        assert!(volcano.valves.len() > 0)
    }
}