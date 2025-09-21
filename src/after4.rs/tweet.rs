trait Summarized {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

// Trait implemented for NewsArticle
impl Summarized for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - by {}", self.headline, self.author)
    }
}

// Trait implemented for Tweet
impl Summarized for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Function jo kisi bhi Summarized trait ko accept kare
fn print_summary(item: &impl Summarized) {
    println!("Summary: {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust becomes the most loved language"),
        author: String::from("Aditya"),
    };

    let tweet = Tweet {
        username: String::from("aditya_dev"),
        content: String::from("Learning Rust traits is fun!"),
    };

    print_summary(&article);
    print_summary(&tweet);
}
