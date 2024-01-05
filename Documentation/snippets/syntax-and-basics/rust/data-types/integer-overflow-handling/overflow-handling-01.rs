let high: u8 = 255;
let wrapped = high.wrapping_add(1); // This will wrap to 0
println!("Wrapped value: {}", wrapped);
