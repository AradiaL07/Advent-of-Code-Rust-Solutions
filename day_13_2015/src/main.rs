use itertools::Itertools;
use std::collections::HashMap;
use std::{fs};
fn main() {
    let people = [
        "Alice", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory", "You",
    ];

    let relation_list =

        fs::read_to_string(".\\input.txt")
            .unwrap();

    let mut relations: HashMap<(&str, &str), i32> = HashMap::new();
    let mut highest_happiness = 0;

    generate_relations(&relation_list, &mut relations);

    for possible_arrangement in people.iter().permutations(9) {
        let current_permutation_happiness = find_happiness(&possible_arrangement, &relations);
        if highest_happiness < current_permutation_happiness {
            highest_happiness = current_permutation_happiness;
        }
    }

    println!("Highest happiness: {}", highest_happiness);
}

fn generate_relations<'a>(list_of_relations: &'a str, relations: &mut HashMap<(&'a str, &'a str), i32>) {
    for l in list_of_relations.lines() {
        let tokens: Vec<&str> = l.split_whitespace().collect();

        match tokens.as_slice() {
            [
                person_1,"would", "gain", x,"happiness","units","by","sitting","next","to", person_2,
            ] => {
                relations.insert((person_1, person_2), x.parse::<i32>().unwrap());
            }

            [
                person_1,"would","lose",x,"happiness","units","by","sitting","next", "to",person_2,
            ] => {
                relations.insert((person_1, person_2), -1 * x.parse::<i32>().unwrap());
            }

            _ => {}
        }
    }
}

fn find_happiness(seating_arrangement: &Vec<&&str>, relations: &HashMap<(&str, &str), i32>) -> i32 {
    let mut happiness = 0;

    for j in 0..seating_arrangement.len()-1 {

        happiness += relations.get(&(seating_arrangement[j], seating_arrangement[j + 1])).unwrap();
        happiness += relations.get(&(seating_arrangement[j + 1], seating_arrangement[j])).unwrap();

    }
    
    //table is circular, so we must also calculate happiness between the first and last person (and vice versa)
    happiness += relations.get(&(seating_arrangement[seating_arrangement.len()-1], seating_arrangement[0])).unwrap();
    happiness += relations.get(&(seating_arrangement[0], seating_arrangement[seating_arrangement.len()-1])).unwrap();

    return happiness;

}
