fn mai()
{
    let mut name = String::from("Aditya");
    name.push_str("Sharma");
     println!("Name is : {}", name);
     name.replace_range(5..name.len(), " ");
     println!("Name is : {}" , name);

}