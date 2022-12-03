
// parse each line
// divide into compartments
// hashmap for first compartment
// go through first compartment, add to hashmap
// go through the second compartment, if in hashmap -> add to total priority (map a letter to a priority)
// O(N) time, O(1) memory 
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::convert::TryInto;

fn get_value(letter: &char) -> u32 {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    for (index, c) in alphabet.iter().enumerate() {
        if letter == c {
            return (index + 1).try_into().unwrap();
        }
    }
    panic!("nope");
}

fn part1() {
    let file = File::open("3_input").unwrap();

    let mut total_priority: u32 = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(rucksack) = line {
            if rucksack == "" {
                continue;
            }

            let mut first_compartment = HashMap::new();
            let mut second_compartment = HashMap::new();
            let half = rucksack.chars().count() / 2;
            for (index, letter) in rucksack.chars().enumerate() {
                if index < half {
                    first_compartment.insert(letter, true);                    
                } else {
                    if first_compartment.contains_key(&letter) {
                        if !second_compartment.contains_key(&letter) {
                            second_compartment.insert(letter, true);
                            println!("Found duplicate! {}, score: {}", letter, get_value(&letter));     
                            total_priority += get_value(&letter);                
                        }
                    }
                }
            }
        }
    }

    println!("Total prio: {}", total_priority);
}

fn part2() {
    let file = File::open("3_input").unwrap();

    let mut total_priority: u32 = 0;
    let mut group = 0;
    let mut shared_backpack = HashMap::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(rucksack) = line {
            if rucksack == "" {
                continue;
            }
            group += 1;
            
            if group == 4 {
                shared_backpack.clear();
                group = 1;
                println!("resetting");
            }
            let mut current_backpack = HashMap::new();

            for letter in rucksack.chars() {
               if !current_backpack.contains_key(&letter) {
                current_backpack.insert(letter, true);

                match shared_backpack.get(&letter) {
                    Some(count) => {
                        if *count == 2 {
                            println!("Found common across all three {}", letter);
                            total_priority += get_value(&letter);
                        } 

                        shared_backpack.insert(letter, *count + 1);
                    },
                    None => {
                        shared_backpack.insert(letter, 1);
                    },
                }
               }
            }

        }
    }

    println!("Total prio: {}", total_priority);
}

fn main() {
    part2();
}