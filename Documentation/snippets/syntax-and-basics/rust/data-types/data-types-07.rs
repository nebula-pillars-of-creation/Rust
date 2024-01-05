fn main() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8; // explicit conversion
    let character = integer as char; // char conversion
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}
