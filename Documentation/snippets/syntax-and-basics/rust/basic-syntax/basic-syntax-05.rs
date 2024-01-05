fn add_two(x: i32) -> i32 {
    x + 2 // expression; no need for a return statement
}

fn main() {
    let num = add_two(5);
    println!("The number is: {}", num);
}
