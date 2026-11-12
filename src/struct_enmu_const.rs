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
pub fn struct_run() {
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

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPrees(char),
    Paste(String),
    Click { x: i32, y: u32 },
}
enum ThisLongLongNameEnumTestName {
    Add,
    Sub,
}
type Operations = ThisLongLongNameEnumTestName;
impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Sub => x - y,
        }
    }
}
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => {
            println!("PageLoad");
        }
        WebEvent::PageUnload => {
            println!("PageUnload");
        }
        WebEvent::KeyPrees(c) => {
            println!("Press {}", c);
        }
        WebEvent::Paste(s) => {
            println!("pasted \"{}\".", s);
        }
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}
pub fn enum_run() {
    let pageload: WebEvent = WebEvent::PageLoad;
    let pageunload: WebEvent = WebEvent::PageUnload;
    let paste: WebEvent = WebEvent::Paste("text".to_string());
    let key_press: WebEvent = WebEvent::KeyPrees('X');
    let click: WebEvent = WebEvent::Click { x: 20, y: 80 };
    inspect(pageload);
    inspect(pageunload);
    inspect(paste);
    inspect(key_press);
    inspect(click);
    // println!("this is longlong enmu name type test coding ");
    // let mut longlongname: Operations = Operations::Add;
    // longlongname = Operations::Sub;
    //
    let test: Operations = Operations::Add;
    println!("{}", test.run(20, 50));
}
