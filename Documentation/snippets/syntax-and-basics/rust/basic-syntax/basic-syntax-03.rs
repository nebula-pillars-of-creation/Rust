fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // this is allowed because x is mutable
    println!("The value of x is: {}", x);
}
