struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    // variable way
    // let width1 = 30;
    // let height1 = 50;

    // tuple way
    // let rect1 = (30,50);

    //struct way
    let rect1 = Rectangle{
        width:30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle)->u32{
    rectangle.width * rectangle.height
}
