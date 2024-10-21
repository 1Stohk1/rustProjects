use std::env;

fn main() {
    let cli_args :Vec<String> = env::args().collect();

    let query = &cli_args[1];
    let file_path = &cli_args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
