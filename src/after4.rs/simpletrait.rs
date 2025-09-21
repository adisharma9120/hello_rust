trait Summarized {
    fn summarize(&self) -> String;
}

struct Book {
    title: String,
    author: String,
}

impl Summarized for Book {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let book = Book {
        title: String::from("Manifestation"),
        author: String::from("Aditya"),
    };

    println!("{}", book.summarize());
}
