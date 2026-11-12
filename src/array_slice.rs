// use std::mem;
// The type of a slice is marked as &[T]
fn analyze_slice(slice: &[i32]) {
    if slice.is_empty() {
        println!("slice.is_empty");
        return;
    }
    println!("first element of the slice: {}", slice[0]);
    println!("slice len :{}", slice.len());
}
pub fn array_slice_code_start() {
    let xs: [i32; 5] = [32, 1, 2, 3, 4];
    analyze_slice(&xs);
    println!("xs array index 1:{}", xs[0]);
    let ys: [i32; 500] = [0; 500];
    println!("ys array length {}", ys.len());
    println!("xs address: {:p}", xs.as_ptr());
    analyze_slice(&xs[2..2]); //the slice length is zero if use slice[0] panic
    analyze_slice(&xs[2..3]); // if xs 2..3 -> slice[0] = 2 like this ruler
}
