fn main() {
    let height: u32 = 3;
    let width: u32 = 8;

    let value = area(width, height);
    println!("The area is {}", value);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
