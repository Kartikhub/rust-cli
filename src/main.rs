use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let file_path = &args[2];
    
    println!("query is {search_query}");
    println!("Provided file path is {file_path}");
}
