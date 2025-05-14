fn main() {
    println!("Hello, world!");
    let strings = ["Hello".to_string(), "World".to_string()];
    process_data(&strings);
}

fn process_data(data: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for item in data {
        result.push(item.clone()); // Unnecessary clone
    }
    result
}
