#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width:  size,
            height: size
        };
    }
}

fn main() {
    let rect1 = Rectangle {
        width:  30,
        height: 50
    };

    let square = Rectangle::square(40);

    println!("The rectangle is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("The square is {:?}", square);
    println!(
        "The area of the square is {} square pixels.",
        square.area()
    );
}
