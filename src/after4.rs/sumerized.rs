/* trait Summarized {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summarized for User {
    fn summarize(&self) -> String {
        format!("Name is: {}, and the age is: {}", self.name, self.age)
    }
}

fn main() {
    let user = User {
        name: String::from("Aditya"),
        age: 32,
    };

    println!("{}", user.summarize());
}
 */
trait Describable {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("'{}' by {} ({} pages)", self.title, self.author, self.pages)
    }
}

fn main() {
    let book = Book {
        title: String::from("Mathematics for Class 12"),
        author: String::from("R.D. Sharma"),
        pages: 1250,
    };

    println!("{}", book.describe());
}
