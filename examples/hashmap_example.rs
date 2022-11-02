use data_structures::map::HashMap;

fn main() {
    let keys = ["one", "two", "three", "four", "five", "six"];
    let values = [1, 2, 3, 4, 5, 6];
    let mut h = HashMap::new();
    for i in 1..keys.len() {
        h.insert(keys[i], values[i]);
    }

    println!("three: {}", h.get("three").expect("value not found."));
    println!("five: {}", h.get("five").expect("value not found."));
    println!("three: {}", h.get("three").expect("value not found."));
    println!("ten: {}", h.get("ten").expect("value not found."));
}