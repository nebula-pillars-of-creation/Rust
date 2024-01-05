let high: u8 = 255;
let (overflow_value, did_overflow) = high.overflowing_add(1); // This will wrap to 0, and `did_overflow` will be true
println!("Overflowing value: {}, did overflow: {}", overflow_value, did_overflow);
