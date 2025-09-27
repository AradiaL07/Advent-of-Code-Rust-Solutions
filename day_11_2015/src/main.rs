fn main() {
    let input_string = "vzbxxyzz";
    let mut input_vec: Vec<char> = input_string.chars().collect();
    let mut is_valid = false;
    
    while !(is_valid) {
         increment_string(&mut input_vec);
        if has_increasing_straight(&input_vec) && (has_dupe_characters(&input_vec)) && !has_banned_chars(&input_vec) {
            is_valid = true;
        }
       
        
    }
    let final_string: String = input_vec.iter().collect();
    println!("First valid password is {}", final_string);
}


fn increment_string(input: &mut Vec<char>) {
    if let Some(last_char) = input.last_mut() {
        if *last_char == 'z' {
            let mut input_clone = input.clone();
            input_clone.remove(input.len()-1);
            increment_string(&mut input_clone);
            input_clone.push('a');
            for i in 0..input.len() {
                input[i] = input_clone[i];
            }
        } else {
            let last_char = input.last_mut().unwrap();
            *last_char = (*last_char as u8+1) as char;
        }
    }
}

fn has_increasing_straight(input: &Vec<char>) -> bool {
    for (i, j) in input.iter().enumerate() {
        if i == input.len()-2 {
            break;
        }
        let test_char_1 = (*j as u8+1) as char;
        let test_char_2 = (*j as u8+2) as char;
        if (input[i+1] == test_char_1) && (input[i+2] == test_char_2) {
            return true;
        }
    }
    return false;
}

fn has_banned_chars(input: &Vec<char>) -> bool {
    if input.contains(&'i') || input.contains(&'o') || input.contains(&'l') {
        return true;
    }
    return false;
}

fn has_dupe_characters(input: &Vec<char>) -> bool {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut dupe_count = 0;
    'outer_check: for c in alphabet {
        for i in 0..input.len()-1 {
            if input[i] == c && input[i+1] == c {
                dupe_count += 1;
                continue 'outer_check;
            }
        }
        }
        
    
    if dupe_count >= 2 {
        return true;
    }
    return false;
}



