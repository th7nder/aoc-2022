use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

#[derive(Debug)]
struct Elf {
    start_section: u32,
    end_section: u32 
}

impl Elf {
    fn fully_contains(&self, other: &Elf) -> bool {
        return self.start_section <= other.start_section 
            && self.end_section >= other.end_section
    }

    fn partially_overlaps(&self, other: &Elf) -> bool {
        
        return other.start_section >= self.start_section && other.start_section <= self.end_section
                || other.end_section >= self.start_section && other.end_section <= self.end_section
    }
}



pub fn part1() {
    let file = match File::open("inputs/4_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file
    };

    let re = Regex::new(r"(?P<elf1_start>\d+)-(?P<elf1_end>\d+),(?P<elf2_start>\d+)-(?P<elf2_end>\d+)").unwrap();

    let reader = io::BufReader::new(file).lines();
    let mut overlaps : u32 = 0;
    
    for line in reader {
        if let Ok(pair) = line {

            let cap = re.captures(&pair).unwrap();
            let first_elf = Elf {
                start_section: cap.name("elf1_start").unwrap().as_str().parse().unwrap(),
                end_section: cap.name("elf1_end").unwrap().as_str().parse().unwrap(),
            };
            let second_elf = Elf {
                start_section: cap.name("elf2_start").unwrap().as_str().parse().unwrap(),
                end_section: cap.name("elf2_end").unwrap().as_str().parse().unwrap(),
            };

            
            overlaps += if first_elf.fully_contains(&second_elf) || second_elf.fully_contains(&first_elf) {
                1
            } else {
                0
            }
        }
    };

    println!("Overlaps: {}", overlaps)
}

pub fn part2() {
    let file = match File::open("inputs/4_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file
    };

    let re = Regex::new(r"(?P<elf1_start>\d+)-(?P<elf1_end>\d+),(?P<elf2_start>\d+)-(?P<elf2_end>\d+)").unwrap();

    let reader = io::BufReader::new(file).lines();
    let mut overlaps : u32 = 0;
    
    for line in reader {
        if let Ok(pair) = line {

            let cap = re.captures(&pair).unwrap();
            let first_elf = Elf {
                start_section: cap.name("elf1_start").unwrap().as_str().parse().unwrap(),
                end_section: cap.name("elf1_end").unwrap().as_str().parse().unwrap(),
            };
            let second_elf = Elf {
                start_section: cap.name("elf2_start").unwrap().as_str().parse().unwrap(),
                end_section: cap.name("elf2_end").unwrap().as_str().parse().unwrap(),
            };

            
            overlaps += if first_elf.partially_overlaps(&second_elf) || second_elf.partially_overlaps(&first_elf) {
                1
            } else {
                0
            }
        }
    };

    println!("Overlaps: {}", overlaps)
}