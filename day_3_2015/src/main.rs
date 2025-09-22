use std::fs;

fn main() {
    let input = fs::read_to_string("C:\\Users\\(username)\\Desktop\\Rust Projects\\day_3_2015\\src\\^^^^vv^^vv^^vv^^^v^vvvvv^^^^^vvv^^^.txt").unwrap();
    let mut houses_visited: Vec<(i32, i32)> = Vec::new();
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut current_robo_x: i32 = 0;
    let mut current_robo_y: i32 = 0;
    let mut index = 0;

    for c in input.chars() {
        if !(houses_visited.contains(&(current_x, current_y))) {
            houses_visited.push((current_x, current_y));
        } 
        if !(houses_visited.contains(&(current_robo_x, current_robo_y))) {
            houses_visited.push((current_robo_x, current_robo_y));
        }
        if index % 2 == 0 {
            if c == '^' {
                increase_y(&mut current_y);
            } else if c == 'v' {
                decrease_y(&mut current_y);
            } else if c == '>' {
                increase_x(&mut current_x);
            } else if c == '<' {
                decrease_x(&mut current_x);
            }

        } else {
            if c == '^' {
                increase_y(&mut current_robo_y);
            } else if c == 'v' {
                decrease_y(&mut current_robo_y);
            } else if c == '>' {
                increase_x(&mut current_robo_x);
            } else if c == '<' {
                decrease_x(&mut current_robo_x);
            }
        }
        index += 1;
        

    }

    println!("Santa delivered presents to {} houses.", houses_visited.len());
}

fn increase_x(x: &mut i32) {
    *x += 1;
}

fn increase_y(y: &mut i32) {
    *y += 1;
}

fn decrease_x(x: &mut i32) {
    *x -= 1;
}

fn decrease_y(y: &mut i32) {
    *y -= 1;
}
