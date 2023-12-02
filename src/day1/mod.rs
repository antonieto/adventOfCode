use std::fs;
use std::collections::HashMap;
use std::string::String;

pub fn main() {
    let contents = fs::read_to_string("./puzzle.txt")
        .expect("Couldn't read file");

    let mut digits: HashMap<String, i32> = HashMap::new();

    digits.insert(String::from("one"), 1);
    digits.insert(String::from("two"), 2);

    
    let parts = contents.split("\n");
    let mut sum = 0;
    for part in parts {
        let mut first = None;
        let mut last = None;
        for ch in part.chars() {
            if ch.is_digit(10) {
                if first.is_none() {
                    first = ch.to_digit(10);
                }
                last = ch.to_digit(10);
            }
        }
        let first_part = first.unwrap_or(0);
        let last_part = last.unwrap_or(0);
        sum += (first_part * 10) + last_part;
    }

    println!("Sum is: {sum}")
    
}
