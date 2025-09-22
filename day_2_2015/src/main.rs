use std::fs;

fn main() {
    let input = fs::read_to_string("C:\\Users\\missn\\Desktop\\Rust Projects\\day_2_2015\\src\\20x3x11.txt").expect("Error reading file");
    let mut total_wrapping_paper: u64 = 0;
    let mut total_ribbon: u64 = 0;
    for l in input.lines() {
        let nums: Vec<&str> = l.split('x').collect();
        let mut nums_converted: Vec<u64> = nums.iter().map(|s| s.parse::<u64>().unwrap()).collect();
        nums_converted.sort();
        println!("{:?}", nums_converted);
        total_wrapping_paper += get_total_paper_needed(&nums_converted);
        total_ribbon += get_total_ribbon_needed(&nums_converted);
    }

    println!("Total wrapping paper needed is {} square feet.", total_wrapping_paper);
    println!("Total ribbon needed is {} feet.", total_ribbon);

}

fn get_total_paper_needed(vec: &Vec<u64>) -> u64 {
    2*(vec[0]*vec[1] + vec[0]*vec[2] + vec[1]*vec[2]) + vec[0]*vec[1]
}

fn get_total_ribbon_needed(vec: &Vec<u64>) -> u64 {
    2*(vec[0] + vec[1]) + vec[0]*vec[1]*vec[2]
}
