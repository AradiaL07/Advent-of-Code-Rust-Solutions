use std::{collections::HashMap};
fn main() {
    let input = std::fs::read_to_string("C:\\Users\\(username)\\Desktop\\Rust Projects\\day_7_2015\\src\\NOT dq - dr.txt").expect("err");
    let mut wires: HashMap<String, u16> = HashMap::new();
    
    for i in 0..=1 {
    let mut unassigned_wires = true;
    while unassigned_wires {
        unassigned_wires = false;
        for l in input.lines() {
        let current_line: Vec<String> = l.split_whitespace().map(|s| s.to_string()).collect();
          if (i == 1) & (current_line.last().unwrap() == "b") {
            continue;
          }  
            if current_line.len() == 3 {
                //println!("Trying to assign {} to {}", current_line[2], current_line[0]);
                let parse_test: Result<u16, _> = current_line[0].parse();
                match parse_test {
                    Ok(_i) => {
                    
                        wires.insert(current_line[2].clone(), parse_test.unwrap());
                        
                    }
                    Err(_e) => {
                        if !(wires.contains_key(&current_line[0])) {
                            unassigned_wires = true;
                            continue;
                        } else {
                            wires.insert(current_line[2].clone(), *wires.get(&current_line[0]).unwrap());
                        }
                    }
                }
                
            } else if current_line.len() == 4 {
                not(&current_line, &mut wires, &mut unassigned_wires);
            } else if current_line[1] == "AND" {
                and(&current_line, &mut wires, &mut unassigned_wires);
            } else if current_line[1] == "OR" {
                or(&current_line, &mut wires, &mut unassigned_wires);
            } else if current_line[1] == "RSHIFT" {
                rshift(&current_line, &mut wires, &mut unassigned_wires);
            } else {
                lshift(&current_line, &mut wires, &mut unassigned_wires);
            }
    }
    
    }
    if i == 0 {
        wires.clear();
        wires.insert(String::from("b"), 3176);
    }
}
println!("The value of a is {:?}", wires.get("a"));

    
}

fn not(current_line: &Vec<String>, wires: &mut HashMap<String, u16>, unassigned_wires: &mut bool) {
     if !(wires.contains_key(&current_line[1])) {
            *unassigned_wires = true;
             return;
        } else {
            wires.insert(current_line[3].clone(), !wires.get(&current_line[1]).unwrap());
        }
} 

fn and (current_line: &Vec<String>, wires: &mut HashMap<String, u16>, unassigned_wires: &mut bool) {
    
    let bool_1 = current_line[0].parse::<u16>().is_ok();
    let bool_2 = current_line[2].parse::<u16>().is_ok();

    if !(bool_1 || bool_2) {
         if !(wires.contains_key(&current_line[0])) || !(wires.contains_key(&current_line[2])) {
         *unassigned_wires = true;
         return;
        }
    
        wires.insert(current_line[4].clone(), wires.get(&current_line[0]).unwrap() & wires.get(&current_line[2]).unwrap());

    } else if bool_1 && bool_2 {
        wires.insert(current_line[4].clone(), current_line[2].parse::<u16>().unwrap() & current_line[0].parse::<u16>().unwrap());
        
    } else if bool_2 {
        if !((wires.contains_key(&current_line[0]))) {
         *unassigned_wires = true;
         return;
        }
    
        wires.insert(current_line[4].clone(), current_line[2].parse::<u16>().unwrap() & wires.get(&current_line[0]).unwrap());
    } else {
       if !((wires.contains_key(&current_line[2]))) {
         *unassigned_wires = true;
         return;
        }
    
        wires.insert(current_line[4].clone(), current_line[0].parse::<u16>().unwrap() & wires.get(&current_line[2]).unwrap());

    }
}

fn or (current_line: &Vec<String>, wires: &mut HashMap<String, u16>, unassigned_wires: &mut bool) {
    let bool_1 = current_line[0].parse::<u16>().is_ok();
    let bool_2 = current_line[2].parse::<u16>().is_ok();

    if !(bool_1 || bool_2) {
         if !wires.contains_key(&current_line[0]) || !wires.contains_key(&current_line[2]) {
         *unassigned_wires = true;
         return;
        }
    
        wires.insert(current_line[4].clone(), wires.get(&current_line[0]).unwrap() | wires.get(&current_line[2]).unwrap());

    } else if bool_1 && bool_2 {
        wires.insert(current_line[4].clone(), current_line[2].parse::<u16>().unwrap() | current_line[0].parse::<u16>().unwrap());
        
    } else if bool_2 {
        if !((wires.contains_key(&current_line[0]))) {
         *unassigned_wires = true;
         return;
        }
    
        wires.insert(current_line[4].clone(), current_line[2].parse::<u16>().unwrap() | wires.get(&current_line[0]).unwrap());
    } else {
       if !((wires.contains_key(&current_line[2]))) {
         *unassigned_wires = true;
         return;
        }
    
        wires.insert(current_line[4].clone(), current_line[0].parse::<u16>().unwrap() | wires.get(&current_line[2]).unwrap());

    }
    
   
}

fn rshift (current_line: &Vec<String>, wires: &mut HashMap<String, u16>, unassigned_wires: &mut bool) {
    if !(wires.contains_key(&current_line[0])) {
         *unassigned_wires = true;
         return;
    }
    
     wires.insert(current_line[4].clone(), wires.get(&current_line[0]).unwrap() >> current_line[2].parse::<u16>().unwrap());
}

fn lshift (current_line: &Vec<String>, wires: &mut HashMap<String, u16>, unassigned_wires: &mut bool) {
    if !(wires.contains_key(&current_line[0])) {
         *unassigned_wires = true;
         return;
    }
    
     wires.insert(current_line[4].clone(), wires.get(&current_line[0]).unwrap() << current_line[2].parse::<u16>().unwrap());
}

