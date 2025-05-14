/// Main entry point for the application.
/// Initializes sample data and processes it.
fn main() {
    println!("Hello, world!");
    let strings = ["Hello".to_string(), "World".to_string()];
    process_data(&strings);
}

/// Processes a slice of strings and returns a new collection.
/// 
/// # Arguments
/// * `data` - A slice of strings to process
///
/// # Returns
/// A vector containing processed strings
fn process_data(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
}
