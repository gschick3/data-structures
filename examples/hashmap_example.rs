use data_structures::map::HashMap;

fn main() {
    let keys = ["three", "four", "five", "six"]; // keys can be any length
    let values = [3, 4, 5, 6];
    let mut h = HashMap::from(vec![("one", 1), ("two", 2)]);
    for i in 0..keys.len() {
        h.insert(keys[i], values[i]);
    }

    println!("three: {}", h.get("three").expect("value not found."));
    println!("two: {}", h.get("two").expect("value not found."));
    println!("three: {}", h.get("three").expect("value not found.")); // accessing elements more than once
    // println!("ten: {}", h.get("ten").expect("value not found.")); // trying to access keys that don't exist will return None

    println!("{:?}", h.get_keys());
}