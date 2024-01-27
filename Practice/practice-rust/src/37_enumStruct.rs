// 27- Define an enum for different types of shapes (circle, square, triangle) and associate it with a struct containing relevant properties.

enum Shapes {
    Square,
    Rectangle,
    Triangle
}

struct Objects {
    side:u32,
    shape:Shapes
}
fn main() {
    let obj1: Objects = Objects{
        side:4,
        shape:Shapes::Square,
    };
    let obj2:Objects = Objects {
        side:4,
        shape:Shapes::Rectangle,
    };
    let obj3:Objects = Objects {
        side:3,
        shape:Shapes::Triangle
    };

    println!("{}, {}, {}", obj1.side, obj2.side, obj3.side);

}