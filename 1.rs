use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = match File::open("1_input") {
        Err(why) => panic!("Couldn't open file {}", why),
        Ok(file) => file
    };

    let reader = io::BufReader::new(file).lines();
    let mut calories = 0;
    let mut arr = Vec::new();
    for line in reader {
        if let Ok(elf_meal) = line {
            if elf_meal == "" {
                arr.push(calories);
                calories = 0;
            } else {
                calories += elf_meal.parse::<u32>().unwrap();
            }
        }
    };
    println!("Sorting");
    arr.sort();
    arr.reverse();

    println!("Top calories {}, {}, {}, sum: {}", arr.get(0).unwrap(), arr.get(1).unwrap(), arr.get(2).unwrap(), 
        arr.get(0).unwrap() + arr.get(1).unwrap() + arr.get(2).unwrap())

}