fn generate_fibonacci_sequence(n: usize) -> Vec<u32> {
    let mut sequence = Vec::new();
    if n >= 1 {
        sequence.push(0);
    }
    if n >= 2 {
        sequence.push(1);
    }

    while sequence.len() < n {
        let last = *sequence.last().unwrap();
        let second_last = *sequence.get(sequence.len() - 2).unwrap_or(&0);
        sequence.push(last + second_last);
    }

    sequence
}

// Example usage:
fn main() {
    let n = 10;
    let sequence = generate_fibonacci_sequence(n);
    println!("The Fibonacci sequence up to the {}-th number is: {:?}", n, sequence);
}
