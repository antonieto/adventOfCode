use crate::util::get_path;
use std::{fs, collections::HashSet};
use regex::Regex;

const INPUT_PATH: &str = "day4/input.txt";

pub fn main() {
    let path = get_path(INPUT_PATH);
    let contents = fs::read_to_string(path)
        .expect("Could not read file");

    let mut sum = 0;
    for line in contents.split("\n") {
        let re = Regex::new(r"(:)|(\|)")
            .expect("Invalid regex");

        let collection: Vec<&str> = re.split(line).skip(1).collect();
        let mut winner_set: HashSet<&str> = HashSet::new();
        let mut exponent: i32 = -1;
        
        if collection.len() != 2 {
            continue;
        }
        
        let sep_split = Regex::new(r"\s+").unwrap(); // Excuse the unwrap :b
        
        for num in sep_split.split(collection[0].trim()) {
            winner_set.insert(num);
        }

        for num in collection[1].trim().split(" ") {
            if winner_set.get(num).is_some() && num != " " {
                exponent += 1;
            }
        }

        if exponent > -1 {
            let base: i32 = 2;
            let true_exponent: u32 = exponent.try_into().expect("Couldn't parse i32 into u32");
            let exponentiated = base.pow(true_exponent);
            sum += exponentiated; 
        }
        
    }
    
    println!("Day 4, 1: sum: {sum}");
}

