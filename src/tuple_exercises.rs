use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (ineger, boolean) = pair;

    (boolean, ineger)
}
fn split_u16(value: u16) -> (u8, u8) {
    let [high, low] = value.to_be_bytes();
    (high, low)
}
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}
fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
pub fn tuple_code_start() {
    println!("tuple_code_start");
    //
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 'a');
    println!("{}", long_tuple.0); // dont use long_tuple[0]
    println!("{}", long_tuple.1);
    println!("{}", long_tuple.2);
    //
    let tuple_of_tuple = ((1u8, 2u16), (3u8, 4u16));
    println!("{:?}", tuple_of_tuple.0);
    println!("{:?}", tuple_of_tuple.1);
    //
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("pair reverse {:?}", reverse(pair));
    // tuple deconstruct
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    //
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix\n:{}", matrix);
    println!("transpose\n:{}", transpose(matrix));
    let cat: u16 = 0x1234;
    println!("split_u16 cat \n:{:#X?}", split_u16(cat));
    println!("{:?}", (5u32,)); // tuple (5,)
    println!("{:?}", (5u32)); // not tuple (5)
}
