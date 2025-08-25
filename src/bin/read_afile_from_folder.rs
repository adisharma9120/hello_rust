use std::fs;

fn main() {
    // Path to the file inside a folder (example: "data/hello.txt")
    let file_path = "data/hello.txt";

    // Try to read the file
    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(text) => {
            println!("File content:\n{}", text);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
