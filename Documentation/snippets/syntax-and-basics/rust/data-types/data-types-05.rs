fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // a tuple with a 32-bit integer, a 64-bit float, and an 8-bit unsigned integer
    let (x, y, z) = tup; // destructuring to get individual values
    println!("The value of y is: {}", y);
}
