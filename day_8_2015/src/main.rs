

fn main() {
    let input = std::fs::read_to_string("C:\\Users\\missn\\Desktop\\Rust Projects\\day_8_2015\\src\\sjdivfriyaaqaxd2vkmpcuyyuen.txt").expect("Err");
    let mut chars_original = 0;
    for l in input.lines() {
        let mut l_snipped = l.strip_prefix("\"").unwrap();
        l_snipped = l_snipped.strip_suffix("\"").unwrap();
        let chars: Vec<char> = l_snipped.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars.get(i).unwrap() == &'\\' {
                match chars.get(i+1).unwrap() {
                    '\"' =>  chars_original += 2,
                    '\\' => {
                        chars_original += 2;
                        i += 2;
                        continue;
                    },
                    'x' => chars_original += 1,
                    _ => (),
                }
            }
            i += 1;
        }
        chars_original += 4;
    }
    println!("Total in code is {}", chars_original);
}
