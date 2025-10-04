use std::fs;

fn main() {
    let input = fs::read_to_string(".\\input.txt").expect("err");
    let containers = parse_input(input);
    let count = count_of_containers(containers, 150);
    println!("{}", count);
}

fn parse_input(input: String) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();

    for l in input.lines() {
        output.push(l.parse().unwrap());
    }

    return output;
}

fn count_of_containers(containers: Vec<i32>, to_contain: i32) -> i32 {
    let mut test = [false; 20];
    let mut count = 0;
    let mut break_count = 0;
    'outer: loop {
        let mut left = to_contain;
        for i in 0..20 {
            if test[i] {
                left -= containers[i];
            }
        }
        if left == 0 {
            let mut current_amount = 0;
            for b in test {
                if b {
                    current_amount += 1;
                }
            }
            if current_amount != 4 {
                increment_array(&mut test);
                break_count += 1;
                continue 'outer;
            }
            count += 1;
        }
        if break_count >= 2_i32.pow(20) {
            break;
        }
        increment_array(&mut test);
        break_count += 1;
    }
    return count;
}

fn increment_array(array: &mut [bool; 20]) {
    'outer: for i in (0..20).rev() {
        if !array[i] {
            array[i] = true;
            for j in i + 1..20 {
                array[j] = false;
            }
            break 'outer;
        }
    }
}
