struct User<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Aditya");
    let user = User { name: &name };
    println!("The name of the user is :{}", user.name);
}
