use serde_json::Value;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("C:\\Users\\missn\\Desktop\\Rust Projects\\day_12_2015\\src\\input.txt")
            .unwrap();

    let json: Value = serde_json::from_str(&input).unwrap();

    let total = sum_json(&json);
    println!("sum {}", total);
}

fn sum_json(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),

        Value::Array(a) => a.iter().map(sum_json).sum(),

        Value::Object(o) => {
            if o.values().any(|v| *v == Value::String("red".to_string())) {
                0
            } else {
                o.values().map(sum_json).sum()
            }
        }
        _ => 0,
    }
}
