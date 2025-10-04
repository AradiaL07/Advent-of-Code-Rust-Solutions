use std::{fs};

fn main() {
    let input = fs::read_to_string(".\\input.txt").unwrap();
    println!("Number: {}", find_answer(input));
}

fn find_answer(input: String) -> i32 {
    'outer: for l in input.lines() {
        let mut correct = 0;
        let filtered: String = l.chars().filter(|c| *c != ':' && *c != ',').collect();
        let tokens: Vec<&str> = filtered.split_whitespace().collect();
        let index = tokens[1];
        for i in 1..=3 {

        
            match tokens[i*2] {
                "children" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() == 3) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "cats" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() > 7) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "samoyeds" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() == 2) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "pomeranians" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() < 3) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "akitas" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() == 0) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "vizslas" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() == 0) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "goldfish" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() < 5) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "trees" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() > 3) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "cars" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() == 2) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                "perfumes" => {
                    if !(tokens[i*2+1].parse::<i32>().unwrap() == 1) {
                        continue 'outer;
                    }
                    correct += 1;
                },
                _ => ()
            }
        }
        if correct == 3 {
            return index.parse().unwrap();
        }
    }
    return 0;
}