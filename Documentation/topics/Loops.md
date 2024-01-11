# Loops

## Overview

Loops are a fundamental control structure in any programming language, allowing you to execute a block of code multiple
times. Rust provides several types of loops: `while`, `for`, and `loop`. Each serves a different purpose and can be used
based on the specific requirements of your code.

## While Loop

The while loop in Rust is used to repeat a block of code as long as a condition is true. It's useful when you want to
loop until a certain condition changes, typically in situations where the number of iterations is not known in advance.

### Practical Example: Reading User Input

Imagine a scenario where you're reading user input until a specific input is received:

```text
use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    while input.trim() != "exit" {
        input.clear(); // Clearing the buffer
        print!("Enter command (type 'exit' to stop): ");
        io::stdout().flush().unwrap(); // Flushing to make sure 'print!' works as expected
        io::stdin().read_line(&mut input).expect("Failed to read line");
    }

    println!("Exiting program");
}

```

In this example, the program continuously prompts the user for input and exits the loop when the user types "exit".

## For Loop

The for loop in Rust is commonly used for iterating over elements of a collection, such as an array or a vector. It's
more concise and less error-prone compared to traditional indexing loops.

### Practical Example: Processing a List of Items

Consider a situation where you have a list of items, and you need to process each item:

```text
fn main() {
    let items = vec!["apple", "banana", "orange"];

    for item in items.iter() {
        println!("Processing {}", item);
    }
}

```

Here, the for loop iterates over each element in the items vector, and the loop body processes each item.

## Loop

The loop keyword in Rust creates an infinite loop. It's the simplest form of loop but offers control over when and how
to break out of the loop using break or continue.

### Practical Example: Retry Mechanism

Imagine implementing a retry mechanism where an operation should be attempted repeatedly until it succeeds:

```text
fn main() {
    let mut attempts = 0;

    loop {
        attempts += 1;
        let success = try_operation();

        if success {
            println!("Operation successful after {} attempts", attempts);
            break;
        } else if attempts >= 3 {
            println!("Operation failed after {} attempts", attempts);
            break;
        }
    }
}

fn try_operation() -> bool {
    // A placeholder for an actual operation that might succeed or fail
    false
}

```

In this example, loop is used to continuously attempt an operation until it's successful or a maximum number of attempts
is reached.

## Conclusion

Each type of loop in Rust serves a different purpose: `while` for condition-based looping, `for` for iterating over
collections, and `loop` for indefinite repetition with more explicit control over when to exit. Rust's strong type system
and pattern matching capabilities make these loops powerful and flexible tools for various scenarios in program flow
control.
