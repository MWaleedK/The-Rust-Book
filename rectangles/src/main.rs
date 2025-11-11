fn main() {
    let rect1: (u32, u32) = (3, 8);

    let value = area(rect1);
    println!("The area of the rectangle is {} square pixels", value);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
