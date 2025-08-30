/* struct Rect {
    width: u32,
    length: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    let rect = Rect {
        width: 20,
        length: 10,
    };
    println!("The area of reactangle is :{}", rect.area());
    2
}
 */

struct Rect {
    width: u32,
    height: u32,
}
/*
 this in a js class

  class React{
   constructor{width, height }
{
   this.width = width;
   this.height = height;
   }

  area(){
  return this.width*this.heigth}
     }
}
      const react1 new = new React(10, 20)
      console.log(react1.area())

*/

impl Rect {
    // 1. Method jo sirf borrow karega (read-only)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 2. Method jo mutable borrow lega (aur value badal dega)
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // 3. Method jo ownership le lega (struct consume ho jayega)
    fn destroy(self) {
        println!("Rect destroyed: {}x{}", self.width, self.height);
        // yaha ke baad 'self' use nahi kar sakte, kyunki ownership chali gayi
    }
}

fn main() {
    let rect = Rect {
        width: 10,
        height: 5,
    };

    // ----------- &self wala case -----------
    // shortcut:
    println!("Area: {}", rect.area());
    // actual expansion:
    println!("Area (expanded): {}", Rect::area(&rect));

    // ----------- &mut self wala case -----------
    let mut rect2 = Rect {
        width: 3,
        height: 4,
    };
    rect2.double_size(); // shortcut
    // expansion:
    Rect::double_size(&mut rect2);
    println!("After double_size -> {}x{}", rect2.width, rect2.height);

    // ----------- self wala case -----------
    let rect3 = Rect {
        width: 7,
        height: 2,
    };
    rect3.destroy(); // shortcut
    // expansion:
    // Rect::destroy(rect3);
    // ab rect3 use nahi ho sakta yaha, kyunki ownership consume ho gayi
}
