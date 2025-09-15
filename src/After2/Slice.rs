fn  main()
{
    let name = String::from("Aditya");

    let first_name = &name[0..5];
    let last_name= &name[5..];
    println!("{}, {}", first_name, last_name);

}