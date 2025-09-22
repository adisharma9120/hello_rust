/* struct User<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Aditya");
    let user = User { name: &name };
    println!("The name of the user is :{}", user.name);
}
 */

struct User<'a> {
    name: &'a str,
}

fn main() {
    let user;
    {
        let name = String::from("Aditya");
        let user = User { name: &name }; // ❌ `name` dies here
    }
    // println!("The name of the user is :{}", user.name);
}


/* ser<'a> says: “I contain a reference &'a str that must live at least as long as 'a.”

But name is created inside the inner block and dropped at the end of that block.

That makes user.name point to invalid memory → Rust prevents this.

So this is not about the let user; line only — it’s about lifetimes not lining up. */