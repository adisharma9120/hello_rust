struct Calculator {
    value: i32,
}

impl Calculator {
    fn new(start: i32) -> Self {
        Self { value: start }
    }

    fn add(&mut self, x: i32) {
        self.value += x;
    }

    fn subtract(&mut self, x: i32) {
        self.value -= x;
    }

    fn divide(&mut self, x: i32) -> Option<i32> {
        if x == 0 {
            None  // return None if divide by zero
        } else {
            self.value /= x;
            Some(self.value)
        }
    }

    fn result(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut calc = Calculator::new(10);
    calc.add(5);
    calc.subtract(3);

    match calc.divide(0) {
        Some(v) => println!("After division: {}", v),
        None => println!("Cannot divide by zero!"),
    }

    println!("Final result: {}", calc.result());
}
