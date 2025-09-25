use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("C:\\Users\\missn\\Desktop\\Rust Projects\\day_9_2015\\src\\Tristram to AlphaCentauri = 34.txt").expect("err");
    let mut routes: HashMap<(&str, &str), u32> = HashMap::new();
    let mut longest_route = 0;
    for l in input.lines() {
        let tokens: Vec<&str> = l.split_whitespace().collect();
        routes.insert((tokens[0], tokens[2]), tokens[4].parse().unwrap());
        routes.insert((tokens[2], tokens[0]), tokens[4].parse().unwrap());
    }
    let cities = vec![
        "Tristram",
        "AlphaCentauri",
        "Snowdin",
        "Tambi",
        "Faerun",
        "Norrath",
        "Straylight",
        "Arbre",
    ];
    let perms = cities.iter().permutations(8);
    for x in perms {
        let mut current_route = 0;
        for i in 0..x.len()-1 {
            current_route += routes.get(&(x[i], x[i + 1])).unwrap();
        }
        if current_route > longest_route {
            longest_route = current_route;
        }
    }
    println!("Distance of longest route is {}", longest_route);
}
