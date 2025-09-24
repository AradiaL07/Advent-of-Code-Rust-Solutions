use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("C:\\Users\\(username)\\Desktop\\Rust Projects\\day_6_2015\\src\\turn off 660,55 through 986,197.txt").expect("Error reading file");
    let mut lights_on: HashMap<(u32, u32), u32> = HashMap::new();
    for x in 0..1000 {
        for y in 0..1000 {
            lights_on.insert((x, y), 0);
        }
    }

    for l in input.lines() {
        let words: Vec<&str> = l.split_whitespace().collect();
        if words[0] == "toggle" {
            let start_coords: Vec<&str> = words[1].split(',').collect();
            let end_coords: Vec<&str> = words[3].split(',').collect();
            let start_x = start_coords.get(0).unwrap().parse::<u32>().unwrap();
            let start_y = start_coords.get(1).unwrap().parse::<u32>().unwrap();
            let end_x = end_coords.get(0).unwrap().parse::<u32>().unwrap();
            let end_y = end_coords.get(1).unwrap().parse::<u32>().unwrap();

            for i in start_x..=end_x {
                for j in start_y..=end_y {
                    let brightness = lights_on.entry((i, j)).or_insert(0);
                    *brightness += 2;
                }
            }
             
            } else if words [1] == "off" {
            let start_coords: Vec<&str> = words[2].split(',').collect();
            let end_coords: Vec<&str> = words[4].split(',').collect();
            let start_x = start_coords.get(0).unwrap().parse::<u32>().unwrap();
            let start_y = start_coords.get(1).unwrap().parse::<u32>().unwrap();
            let end_x = end_coords.get(0).unwrap().parse::<u32>().unwrap();
            let end_y = end_coords.get(1).unwrap().parse::<u32>().unwrap();
                for i in start_x..=end_x {
                for j in start_y..=end_y {
                    let brightness = lights_on.entry((i, j)).or_insert(0);
                    if *brightness > 0 {
                        *brightness -= 1;
                    }
                }
            }

            
        } else {
            let start_coords: Vec<&str> = words[2].split(',').collect();
            let end_coords: Vec<&str> = words[4].split(',').collect();
            let start_x = start_coords.get(0).unwrap().parse::<u32>().unwrap();
            let start_y = start_coords.get(1).unwrap().parse::<u32>().unwrap();
            let end_x = end_coords.get(0).unwrap().parse::<u32>().unwrap();
            let end_y = end_coords.get(1).unwrap().parse::<u32>().unwrap();
             for i in start_x..=end_x {
                for j in start_y..=end_y {
                    let brightness = lights_on.entry((i, j)).or_insert(0);
                    *brightness += 1;
                }
            }
            
        }
        println!("Done with one line");
    
        
    
    let mut current_brightness = 0;
        for i in &lights_on {
            current_brightness += i.1;
        }
    println!("The amount of lights on is {}", current_brightness);

    }
}
