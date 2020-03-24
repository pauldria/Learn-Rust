struct Rectangle {
    width:  u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width:  30,
        height: 50
    };

    let ptr_to_rect1 = &rect1;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(ptr_to_rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
