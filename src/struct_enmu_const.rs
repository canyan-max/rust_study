#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}
struct Unit;
struct Tuple_struct(f32, f32);
struct Point {
    x: f32,
    y: f32,
}
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
pub fn struct_enmu_const_run() {
    let name: String = String::from("Perter");
    let age: u8 = 18;
    let person_info: Person = Person { name, age };
    println!("{:?}", person_info);

    let point: Point = Point { x: 1.2, y: 2.3 };
    println!("point.x {:?} point.y{:?}", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let pair = Tuple_struct(0.21, 0.1);
    println!("Tuple_struct : (0={}, 1={})", pair.0, pair.1);

    let Tuple_struct(integer, decimal) = pair;
    println!("Tuple_struct contains {:?} and {:?}", integer, decimal);
}
