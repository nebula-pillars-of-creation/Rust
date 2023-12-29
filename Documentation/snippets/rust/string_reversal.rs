/// Reverses a given string.
///
/// # Arguments
///
/// * `s` - A string slice that holds the string to be reversed.
///
/// # Returns
///
/// * A new `String` that is the reverse of `s`.
///
/// # Errors
///
/// * This function will return an error if the string is empty.
fn reverse_string(s: &str) -> Result<String, &'static str> {
    if s.is_empty() {
        return Err("Input string must not be empty.");
    }

    Ok(s.chars().rev().collect::<String>())
}

// Example usage:
fn main() {
    let original_string = "hello";
    match reverse_string(original_string) {
        Ok(reversed) => println!("Reversed string: {}", reversed),
        Err(e) => println!("Error: {}", e),
    }
}


fn reverse_string(s: &mut [u8]) {
    let len = s.len();
    for i in 0..len / 2 {
        s.swap(i, len - i - 1);
    }
}

// Example usage:
fn main() {
    let mut original_string = String::from("hello").into_bytes();
    reverse_string(&mut original_string);
    let reversed_string = String::from_utf8(original_string).expect("Invalid UTF-8");
    println!("Reversed string: {}", reversed_string);
}
