use std::fs;

/*
 Rust ka built-in Resul
 enum Result<T, E> { 
    Ok(T),
    Err(E),
} */
fn main() {
    let greeting_file_result = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read succesfully :{}", file_content);
        }
        Err(error) => {
            println!("failed to read the file :{}", error);
        }
    }
}
