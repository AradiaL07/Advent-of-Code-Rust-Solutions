use std::fs;

fn main() {
    let input = fs::read_to_string(".\\input.txt").expect("error reading file");
    let ingredients = parse_input(input);
    println!("{:?}", ingredients);
    let mut current_highest_score: i64 = 0;
    for i in 0..= 100 {
        for j in 0..=100-i {
            for k in 0..=100-i-j {
                for l in 0..=100-i-j-k {
                    let mut current_cookie: Vec<(i64, i64, i64, i64, i64)> = Vec::new();
                    current_cookie.push(calculate_cookie_scores(&ingredients[0], i));
                    current_cookie.push(calculate_cookie_scores(&ingredients[1], j));
                    current_cookie.push(calculate_cookie_scores(&ingredients[2], k));
                    current_cookie.push(calculate_cookie_scores(&ingredients[3], l));
                    let test_score = calculate_total_cookie_score(&current_cookie);
                    if test_score >= current_highest_score {
                        current_highest_score = test_score;
                    }
                }
            }
        }
        
        println!("One big loop done!");
    }
    println!("Highest score was {}", current_highest_score);
}

fn parse_input(input: String) -> Vec<Ingredient> {
    let mut parsed_ingredients: Vec<Ingredient> = Vec::new();
    for l in input.lines() {
        let filtered_line: String = l.chars().filter(|c| !(*c == ':' || *c == ',')).collect();
        let tokens: Vec<&str> = filtered_line.split_whitespace().collect();
        match tokens.as_slice() {
            [
                _name,
                "capacity",
                capacity,
                "durability",
                durability,
                "flavor",
                flavor,
                "texture",
                texture,
                "calories",
                calories,
            ] => {
                let ingredient = Ingredient {
                    capacity: capacity.parse().unwrap(),
                    durability: durability.parse().unwrap(),
                    flavor: flavor.parse().unwrap(),
                    texture: texture.parse().unwrap(),
                    calories: calories.parse().unwrap(),
                };
                parsed_ingredients.push(ingredient);
            }
            _ => (),
        }
    }
    parsed_ingredients
}

fn calculate_cookie_scores(ingredient: &Ingredient, teaspoons: i64) -> (i64, i64, i64, i64, i64) {
    let total_capacity = ingredient.capacity * teaspoons;
    let total_durability = ingredient.durability * teaspoons;
    let total_flavor = ingredient.flavor * teaspoons;
    let total_texture = ingredient.texture * teaspoons;
    let total_calories = ingredient.calories * teaspoons;
    return (total_capacity, total_durability, total_flavor, total_texture, total_calories);
    
}


fn calculate_total_cookie_score(ingredients: &Vec<(i64, i64, i64, i64, i64)>) -> i64 {
    let mut total_capacity: i64 = 0;
    let mut total_durability: i64 = 0;
    let mut total_flavor: i64 = 0;
    let mut total_texture: i64 = 0;
    let mut total_calories: i64 = 0;
    for i in ingredients {
        total_capacity += i.0;
        total_durability += i.1;
        total_flavor += i.2;
        total_texture += i.3;
        total_calories += i.4;
    }

    if total_capacity < 0 || total_durability < 0 || total_flavor < 0 || total_texture < 0 || total_calories != 500 {
        return 0;
    }
    return total_capacity * total_durability * total_flavor * total_texture;
}
#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}


