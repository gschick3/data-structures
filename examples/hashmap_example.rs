use data_structures::map::HashMap;

fn main() {
    let keys = ["one", "two", "three", "four", "five", "six"]; // keys can be any length
    let values = [1, 2, 3, 4, 5, 6];
    let mut h = HashMap::new();
    for i in 0..keys.len() {
        h.insert(keys[i], values[i]);
    }

    println!("three: {}", h.get("three").expect("value not found."));
    println!("five: {}", h.get("five").expect("value not found."));
    println!("three: {}", h.get("three").expect("value not found.")); // accessing elements more than once
    // println!("ten: {}", h.get("ten").expect("value not found.")); // trying to access keys that don't exist will return None

    println!("{:?}", h.get_keys());
}