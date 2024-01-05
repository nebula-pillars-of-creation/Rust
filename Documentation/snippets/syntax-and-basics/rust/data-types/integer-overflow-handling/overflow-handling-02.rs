let high: u8 = 255;
let saturated = high.saturating_add(1); // This will saturate to 255
println!("Saturated value: {}", saturated);
