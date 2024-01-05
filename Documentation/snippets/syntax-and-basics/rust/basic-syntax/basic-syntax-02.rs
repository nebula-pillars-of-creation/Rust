fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; // this line will cause a compile-time error
    println!("The value of x is: {}", x);
}
