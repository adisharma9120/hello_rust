struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    // this is fn implementation
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}
fn main() {
    let rect = Rect {
        width: 32,
        height: 10,
    };
    let mut rect = Rect {
        width: 10,
        height: 5,
    };
    rect.double_size();
    println!("New width: {}, height: {}", rect.width, rect.height);

    println!("The Area of rectangle is {}", rect.area());
    println!("The perimeter of the rectangle is {}", rect.perimeter());
}
