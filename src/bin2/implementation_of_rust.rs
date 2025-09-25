struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    // calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // calculate perimeter
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // double the size of the rectangle
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

fn main() {
    let mut rect = Rect {
        width: 10,
        height: 5,
    };

    rect.double_size();
    println!("New width: {}, height: {}", rect.width, rect.height);

    println!("The area of the rectangle is {}", rect.area());
    println!("The perimeter of the rectangle is {}", rect.perimeter());
}
