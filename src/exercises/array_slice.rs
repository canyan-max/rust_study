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
fn fill_bytes(array: &mut [u8], data: u8) {
    for i in 0..array.len() {
        array[i] = data;
    }
}
fn fill_bytes_iter_mut(array: &mut [u8], data: u8) {
    for byte in array.iter_mut() {
        *byte = data;
    }
}
fn check_buffer_is_valid(array: &[u8]) {
    match array.first() {
        Some(byte) => {
            println!("byte={}", byte);
        }
        None => {
            println!("none byte");
        }
    }
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
    let mut buff: [u8; 3] = [0x11, 0x22, 0x33];
    for i in 0..buff.len() {
        println!("{:02x}", buff[i]);
    }
    for byte in buff.iter() {
        println!("iter ={:02x}", *byte);
    }
    println!("start buffer index + 1");
    for byte in buff.iter_mut() {
        *byte += 1;
    }
    for i in 0..buff.len() {
        println!("{:02x}", buff[i]);
    }
    println!("end buffer index + 1");
    for (index, byte) in buff.iter().enumerate() {
        print!("index={} byte={:02x} ", index, byte);
    }

    let frist: u8 = buff[0];
    let frist_ref: &u8 = &buff[1];
    let part: &[u8] = &buff[1..3];
    println!("frist={}", frist);
    println!("frist_ref={}", frist_ref);
    println!("part len ={}", part.len());
    for (i, j) in part.iter().enumerate() {
        println!("index={}byte={}", i, j);
    }
    let part2: &mut [u8] = &mut buff[1..3];
    part2[0] = 0xaa;
    println!("part2[0]{:02X}", part2[0]);
    for (index, byte) in buff.iter().enumerate() {
        print!("buffe : index={} byte={:02x} ", index, byte);
    }
    fill_bytes(&mut buff, 0xff);
    for (index, byte) in buff.iter().enumerate() {
        print!("buffe : index={} byte={:02x} ", index, byte);
    }
}
#[cfg(test)]
mod test {
    // use crate::exercises::array_slice::check_buffer_is_valid;
    use super::check_buffer_is_valid;
    #[test]
    fn test_check_buffer_is_valid() {
        let buffer: [u8; 4] = [0; 4];
        check_buffer_is_valid(&buffer);
        check_buffer_is_valid(&[]);
    }
}
