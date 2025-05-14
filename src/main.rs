fn main() {
    println!("Hello, world!");
    let strings = ["Hello".to_string(), "World".to_string()];
    process_data(&strings);
}

fn process_data(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
}
