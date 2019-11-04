#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.width > other.height && self.height > other.width)
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };
    let rect_01 = Rectangle { width: 10, height: 20 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("rect is {:#?}", rect);

    println!("rect can hold rect_01:{}", rect.can_hold(&rect_01));
}
