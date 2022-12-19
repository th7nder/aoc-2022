use std::collections::hash_map;
use std::{collections::HashMap};
use std::{fs::File};
use std::io::{self, BufRead};

use regex::Regex;

pub fn solve() {

}


#[derive(Debug)]
struct Volcano {
    valves: HashMap<String, Valve>,
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
    
        Volcano { valves }
    }


    fn simulate(&self) {
        let mut state = HashMap::new();
        for valve in self.valves.keys() {
            state.insert(valve.clone(), false);
        }

        self.decision("AA", 0, 0, state);
    }

    // this is bad, we need to rethink the approach
    fn decision(&self, current_valve: &str, minute: i32, released_pressure: i32, state: HashMap<String, bool>) {
        if minute == 30 {
            return
        }
        // TODO: not sure, when to + 1 and when not to plus one?
        let minute = minute + 1;
        let mut released_pressure = released_pressure;

        let mut everything_open = true;
        for (valve, opened) in &state {
            if *opened {
                released_pressure += self.valves.get(valve).unwrap().flow_rate;
            } else if self.valves.get(valve).unwrap().flow_rate != 0 {
                everything_open = false;
            }
        }

        if minute == 30 {
            return;
        }

        if everything_open {
            println!("everything open, minute: {}, pressure: {}", minute, released_pressure);
            // self.decision(current_valve, minute, released_pressure, state.clone());
            return;
        }

        let current_valve = self.valves.get(current_valve).unwrap();
        if current_valve.flow_rate != 0 && !state.get(&current_valve.name).unwrap() {
            let mut next_state = state.clone();
            next_state.insert(current_valve.name.clone(), true);
            self.decision(&current_valve.name, minute, released_pressure, next_state)
        }

        for valve in &current_valve.connected_to {
            self.decision(valve, minute, released_pressure, state.clone());            
        }
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
        volcano.simulate();
        assert!(volcano.valves.len() > 0)
    }
}