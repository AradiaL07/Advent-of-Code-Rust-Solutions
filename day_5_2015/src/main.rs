
use std::fs;
fn main() {
    let input: String = fs::read_to_string("C:\\Users\\(username)\\Desktop\\Rust Projects\\day_5_2015\\src\\uxcplgxnkwbdwhrp.txt").expect("Error reading file");
    let input_vec: Vec<&str> = input.split('\n').collect();
    let mut number_of_nice_strings = 0;
    println!("{:?}", input_vec);

    for s in &input_vec {
        if check_if_nice(s) {
            number_of_nice_strings += 1;
        }
    }
    println!("The number of nice strings is {}", number_of_nice_strings);



}

fn check_if_nice(s: &str) -> bool {
    let mut has_matching_combo = false;
    let mut has_repeat_letter = false;
     for i in 0..(s.len()-1) {
        let current_combo = format!("{}", &s[i..i+2]);
        for j in (i + 2)..(s.len()-1) {
            let test_combo = format!("{}", &s[j..j+2]);
            if current_combo == test_combo {
                has_matching_combo = true;
                break;
            }
            
        }
    }
    for i in 0..(s.len()-2) {
        if s.chars().nth(i) == s.chars().nth(i + 2) {
            has_repeat_letter = true;
        }
    }

    has_matching_combo && has_repeat_letter
}
