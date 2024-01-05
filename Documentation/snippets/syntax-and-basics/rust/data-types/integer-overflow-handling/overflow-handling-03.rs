let high: u8 = 255;
if let Some(checked) = high.checked_add(1) {
    println!("Checked value: {}", checked);
} else {
    println!("Overflow occurred!");
}
