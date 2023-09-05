use std::env;
use std::fs;

fn main() {
    // Get the file name from the command line args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Read the file into a string
    let contents = 
    match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading the contents of the file: {}", e);
            return;
        } 
    };

    // Split string into words and count the number of "and"
    let count = contents.split_whitespace()
    .filter(|&word| word == "and").count();

    // Print result
    println!("The word 'and' = {} times times in the contents of the file", count);
}

