struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1: Rectangle = Rectangle {
        height: 3,
        width: 8,
    };

    let value = area(&rect1);
    println!("The area of the rectangle is {} square pixels", value);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
