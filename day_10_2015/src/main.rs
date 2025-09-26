

fn main() {
    let mut input = "1113222113".to_string();
    for _i in 0..50 {
        let mut new_input = "".to_string();
        let input_chars: Vec<char> = input.chars().collect();
        let mut current_char_count: u32 = 0;
        for i in 0..input_chars.len() - 1 {
            current_char_count += 1;

            if input_chars[i] != input_chars[i + 1] {
                new_input.push_str(&current_char_count.to_string());
                new_input.push(input_chars[i]);

                current_char_count = 0;
            }
            if i == input_chars.len() - 2 {
                let second_last_char = input_chars[i];
                if second_last_char == input_chars[i + 1] {
                    new_input.push_str(&current_char_count.to_string());
                    new_input.push(second_last_char);
                } else {
                    new_input.push('1');
                    new_input.push(input_chars[input_chars.len()-1]);
                }
            }
        }

        input = new_input;
    }
    println!("Output length of the look-and-say sequence is {}", input.len());
}
