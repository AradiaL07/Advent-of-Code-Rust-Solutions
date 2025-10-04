use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string(".\\input.txt").expect("error reading file");
    let mut current_status = parse_input(input);
    for _i in 0..100 {
        advance_game(&mut current_status);
    }
    println!("{}", count_on_cells(&current_status));
    
}

fn parse_input(input: String) -> HashMap<(i32, i32), bool> {
    let mut x_coord = 0;
    let mut y_coord = 0;
    let mut initial_status: HashMap<(i32, i32), bool> = HashMap::new();

    for l in input.lines() {
        for c in l.chars() {
            if c == '#' {
                initial_status.insert((x_coord, y_coord), true);
            } else {
                initial_status.insert((x_coord, y_coord), false);
            }
            x_coord += 1;
        }
        x_coord = 0;
        y_coord += 1;
    }
    initial_status.insert((0, 0), true);
    initial_status.insert((99, 0), true);
    initial_status.insert((0, 99), true);
    initial_status.insert((99, 99), true);
    
    return initial_status;
}

fn advance_game(current_status: &mut HashMap<(i32, i32), bool>) {
    let current_status_clone = current_status.clone();
    for b in current_status.clone() {
        let mut on_neighboring_cells = 0;
        let current_x = b.0.0;
        let current_y = b.0.1;
        //these horrible 8 if statements are for checking if adjecent cells are on. i couldnt think of a better way of doing this
        if *current_status_clone.get(&(current_x-1, current_y)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        }
        if *current_status_clone.get(&(current_x+1, current_y)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        }
        if *current_status_clone.get(&(current_x, current_y+1)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        }
        if *current_status_clone.get(&(current_x, current_y-1)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        
        }
        if *current_status_clone.get(&(current_x-1, current_y-1)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        
        }
        if *current_status_clone.get(&(current_x+1, current_y-1)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        
        }
        if *current_status_clone.get(&(current_x-1, current_y+1)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        
        }
        if *current_status_clone.get(&(current_x+1, current_y+1)).unwrap_or(&false) {
            on_neighboring_cells += 1;
        
        }
        
        if !current_status_clone.get(&(current_x, current_y)).unwrap() && on_neighboring_cells == 3 {
            current_status.insert((current_x, current_y), true);
            
        }
        if *current_status_clone.get(&(current_x, current_y)).unwrap() && !(on_neighboring_cells == 2 || on_neighboring_cells == 3) {
            current_status.insert((current_x, current_y), false);
        }
        current_status.insert((0, 0), true);
        current_status.insert((99, 0), true);
        current_status.insert((0, 99), true);
        current_status.insert((99, 99), true);
    }
}

fn count_on_cells(current_status: &HashMap<(i32, i32), bool>) -> i32 {
    let mut on_count = 0;
    for b in current_status {
        if *b.1 {
            on_count += 1;
        }
    }
    
    return on_count;
}