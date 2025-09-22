use md5;
fn main() {
    let input = "bgvyzdsv";
    let mut current_int = 0;
    loop {
        
        let input_string = input.to_owned() + &current_int.to_string();
        let digest = md5::compute(input_string.as_bytes());
        let md5_string = format!("{:x}", digest);
        if &md5_string[0..6] == "000000" {
            break;
        }
        current_int += 1;
    }
    println!("The first number that gives the hash is {}", current_int);
}
