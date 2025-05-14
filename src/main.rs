fn main() {
    println!("Hello, world!");
    process_data(&vec!["Hello".to_string(), "World".to_string()]);
}

fn process_data(data: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for item in data {
        result.push(item.clone()); // Unnecessary clone
    }
    result
}
