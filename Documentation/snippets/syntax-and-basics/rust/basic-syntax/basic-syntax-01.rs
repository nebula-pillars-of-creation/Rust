fn main() {
    let x = 5; // statement
    let y = {
        let x = 3;
        x + 1 // expression
    };

    println!("The value of y is: {}", y);
}
