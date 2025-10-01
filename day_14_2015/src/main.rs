use std::{collections::HashMap, fs};

fn main() {
    let reindeer_stats = fs::read_to_string(".\\input.txt").expect("err");
    let mut parsed_reindeer_stats = parse_reindeer_stats(reindeer_stats);
    println!("The amount of points the winner has is {}", reindeer_race(&mut parsed_reindeer_stats, 2503));
}

fn parse_reindeer_stats(to_parse: String) -> HashMap<String, (i32, i32, i32, i32)> {
    let mut parsed_reindeer: HashMap<String, (i32, i32, i32, i32)> = HashMap::new();
    for l in to_parse.lines() {
        let tokens: Vec<&str> = l.split_whitespace().collect();
        match tokens.as_slice() {
            [
                reindeer_name,
                "can",
                "fly",
                speed,
                "km/s",
                "for",
                flight_length,
                "seconds,",
                "but",
                "then",
                "must",
                "rest",
                "for",
                rest_length,
                "seconds.",
            ] => {
                parsed_reindeer.insert(
                    reindeer_name.to_string(),
                    (
                        speed.parse().unwrap(),
                        flight_length.parse().unwrap(),
                        rest_length.parse().unwrap(),
                        0,
                    ),
                );
            }
            _ => (),
        }
    }
    parsed_reindeer
}


fn reindeer_distance(reindeer_name: String, reindeer_stats: &(i32, i32, i32, i32), time: i32) -> (String, i32) {
    let mut remaining_time = time;
    let mut total_distance = 0;
    'outer: loop {
        for _i in 0..reindeer_stats.1 {
            total_distance += reindeer_stats.0;
            remaining_time -= 1;
            if remaining_time <= 0 {
                break 'outer;
            }
        }
        for _i in 0..reindeer_stats.2 {
            remaining_time -= 1;
            if remaining_time <= 0 {
                break 'outer;
            }
        }
    }
    return (reindeer_name, total_distance);
}

fn reindeer_race(reindeer: &mut HashMap<String, (i32, i32, i32, i32)>, time: i32) -> i32{
    let mut remaining_time = 0;
    while remaining_time <= time {
        let mut current_timestamp: Vec<(String, i32)> = Vec::new();
        for r in &mut *reindeer {
            current_timestamp.push(reindeer_distance(r.0.clone(), r.1, remaining_time))
        }
        let mut current_highest_distance = 0;
        for r in &current_timestamp {
            if r.1 >= current_highest_distance {
                current_highest_distance = r.1;
            }
        }
        for r in &current_timestamp {
            if r.1 == current_highest_distance {
                reindeer.get_mut(&r.0).unwrap().3 += 1;
            }
        }
        remaining_time += 1;
    }
    let mut winning_points = 0;
    for r in reindeer {
        if r.1.3 >= winning_points {
            winning_points = r.1.3;
        }
    }
    return winning_points;
    
}