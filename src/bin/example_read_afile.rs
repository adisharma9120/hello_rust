use std::fs;

fn main() {
    // Path to the file inside a folder (example: "files/hello.txt")
    let path = "files/hello.txt";

    // Try to read the file
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
}
