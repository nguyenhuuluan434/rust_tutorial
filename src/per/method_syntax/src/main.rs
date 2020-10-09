fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {:#?} square pixels.",
        rect1.area()
    );
    rect1.set_width(10);
    println!(
        "The area of the rectangle is {:#?} square pixels.",
        rect1.area()
    );
    rect1.set_width(100);
    println!(
        "The area of the rectangle is {:#?} square pixels.",
        rect1.perimeter()
    );

    let new_square= Rectangle::square(10);
    println!(
        "The area of the rectangle is {:#?} square pixels.",
        new_square.perimeter()
    );
    println!(
        "The area of the rectangle is {:#?} square pixels.",
        rect1.can_hold(&new_square)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> i64 {
        (self.height * self.width) as i64
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

impl Rectangle {
    fn perimeter(&self) -> i64 {
        ((self.height + self.width) * 2) as i64
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //this
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
