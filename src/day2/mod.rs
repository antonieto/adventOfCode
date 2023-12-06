use std::fs;
use super::util::get_path;

static RED_CAP: i32 = 12;
static GREEN_CAP: i32 = 13;
static BLUE_CAP: i32 = 14;

const INPUT_PATH: &str = "day2/input.txt";


// Problem one
pub fn main() {
    let path = get_path(INPUT_PATH);
    let content = fs::read_to_string(path).expect("Could not open file");

    let lines = content.lines();

    let mut sum = 0;
    for line in lines {
        let id: i32 = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        // Split game sets
        let game_sets_str = line.split(":").last().unwrap();
        let mut is_valid = true;
        for set in game_sets_str.split("; ") {
            // Split set by ', '
            for hand in set.split(", ") {
                let formatted = hand.trim();
                let n: i32 = formatted.split(" ").next().unwrap().parse().unwrap();
                is_valid = match hand.split(" ").last().unwrap() {
                    "red" => {
                        if n > RED_CAP {
                            false
                        } else {
                            is_valid
                        }
                    }
                    "green" => {
                        if n > GREEN_CAP {
                            false
                        } else {
                            is_valid
                        }
                    }
                    "blue" => {
                        if n > BLUE_CAP {
                            false
                        } else {
                            is_valid
                        }
                    }
                    _ => is_valid,
                };
            }
        }
        if is_valid {
            sum += id;
        }
    }
    println!("sum: {sum}")
}

// Solution to day two second problem
pub fn second() {
    let path = get_path(INPUT_PATH);
    let content = fs::read_to_string(path).expect("Could not open file");

    let lines = content.lines();
    let mut acc = 0;
    for line in lines {
        // Split game sets
        let game_sets_str = line.split(":").last().unwrap();
        let mut min_blue: i32 = 0;
        let mut min_red: i32 = 0;
        let mut min_green: i32 = 0;

        for set in game_sets_str.split("; ") {
            // Split set by ', '
            for hand in set.split(", ") {
                let formatted = hand.trim();
                let n: i32 = formatted.split(" ").next().unwrap().parse().unwrap();
                match hand.split(" ").last().unwrap() {
                    "red" => {
                        min_red = i32::max(min_red, n);
                    }
                    "green" => {
                        min_green = i32::max(min_green, n);
                    }
                    "blue" => min_blue = i32::max(min_blue, n),
                    _ => {}
                };
            }
        }
        let power = min_blue * min_red * min_green;
        acc += power;
    }
    println!("sum: {acc}")
}
