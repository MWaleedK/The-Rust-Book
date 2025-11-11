#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        height: dbg!(3 * scale),
        width: 8,
    };

    let value = area(&rect1);
    println!("The area of the rectangle is {} square pixels", value);
    dbg!("rect1 has values {:?}", &rect1); //takes ownership so an explicit reference is provided
    println!("rect1 has values {:#?}", rect1); //takes reference
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
