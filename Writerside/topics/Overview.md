# Overview

## What is Rust?

Rust is a system programming language focused on safety, speed, and concurrency.
It achieves these goals through features like ownership, zero-cost abstractions, and a rich type system.

## Why to use it?

It’s used because it guarantees memory safety and thread safety, thus preventing a whole class of bugs at compile-time.

### Memory Safety Example:

In many programming languages, accessing or modifying memory that the program does not own can lead to undefined
behavior, security vulnerabilities, or program crashes (segmentation faults). Rust prevents this through its ownership
system.

``` rust
fn main() {
    let v = vec![1, 2, 3];

    let v2 = v; // v is moved to v2
    println!("v[0] is: {}", v[0]); // This will cause a compile-time error
}
```

In the above code, the vector v is moved to v2. Rust enforces that v can no longer be used after the move. Trying to use
v after this point will result in a compile-time error, thus preventing invalid memory access.

### Thread Safety Example:

Rust's ownership system also helps prevent data races at compile-time, which is a common problem in concurrent
programming. A data race occurs when two threads access the same memory location concurrently, and at least one of the
accesses is for a write condition.

``` rust
use std::thread;

fn main() {
    let mut data = vec![1, 2, 3];

    thread::spawn(move || {
        data[0] = 4; // This is safe because 'data' is moved into the closure
    });

    // data.push(4); // This would cause a compile-time error
}
```

In this example, the vector data is moved into a closure, which is executed by a new thread. If we attempt to use data
after it has been moved into the closure (like the commented-out line), Rust will refuse to compile the code, preventing
a potential data race.

> NOTE: Rust's compile-time checks for ownership, borrowing, and lifetimes are the backbone of its memory safety
> guarantees,
> while the Send and Sync traits enforce thread safety. These checks ensure that unsafe behavior is caught during
> compilation, rather than at runtime, which is what prevents a whole class of bugs.


