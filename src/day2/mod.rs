use std::fs;

static RED_CAP: i32 = 12;
static GREEN_CAP: i32 = 13;
static BLUE_CAP: i32 = 14;

pub fn main() {
    let content = fs::read_to_string("/Users/antoniochairesmonroy/code/advent/y2023/src/day2/input.txt")
        .expect("Could not open file");
 
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
                // println!("The hand is: {formatted}");
                let n: i32 = formatted.split(" ").next().unwrap().parse().unwrap();
                // println!("The n part is: {n}");
                match hand.split(" ").last().unwrap() {
                    "red" => {
                        if n > RED_CAP {
                            is_valid = false;
                        }
                    },
                    "green" => {
                        if n > GREEN_CAP {
                            is_valid = false;
                        }
                        
                    },
                    "blue" => {
                        if n > BLUE_CAP {
                            is_valid = false;
                        }
                    },
                    _ => {
                    }
                };
            }

        }
        if is_valid {
            sum += id;
        }

    }
    println!("sum: {sum}")

}
