fn main() {
    let width = 30;
    let height = 50;

    let area = calculate_area(width, height);

    println!("The area of the rectangle is {} square pixels.", area);
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height // No semicolon here means this is an expression that returns the result
}
