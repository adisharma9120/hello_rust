/* trait Summarized {
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
 */

 trait Summarized {
    fn summarize(&self) -> String;
}

#[derive(Clone)]  // 👈 ab Book clone ho sakta hai
struct Book {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summarized for Book {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summarized for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// 1) impl Trait syntax
fn print_summary_impl(item: &impl Summarized) {
    println!("(impl Trait) {}", item.summarize());
}

// 2) Trait Bound syntax
fn print_summary_bound<T: Summarized>(item: &T) {
    println!("(Trait Bound) {}", item.summarize());
}

// 3) where clause with multiple trait bounds
fn print_summary_where<T>(item: &T)
where
    T: Summarized + Clone,
{
    println!("(where clause) {}", item.summarize());
}

fn main() {
    let book = Book {
        title: String::from("Rust Book"),
        author: String::from("Aditya"),
    };

    let tweet = Tweet {
        username: String::from("aditya_dev"),
        content: String::from("Rust is awesome!"),
    };

    print_summary_impl(&book);
    print_summary_bound(&tweet);

    let cloned_book = book.clone(); // ✅ ab ye kaam karega
    print_summary_where(&cloned_book);
}
