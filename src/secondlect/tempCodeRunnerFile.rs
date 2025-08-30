enum Shape{
 Rectangle(f64 ,f64), // width , heigth
 Circle(f64), // radius
}

fn main()
{
    let my_shape = Shape::Rectangle(10.0,20.0);
 print_area(my_shape)
}
 fn print_area(_shape: Shape){
    println!("hii ther");
 }