# Immutability
<show-structure for="chapter" depth="3"/>

## Introduction

In Rust, one of the core principles is that variables are immutable by default. This feature is fundamental to Rust's
aim of ensuring safe concurrency and preventing various common programming errors. In this chapter, we'll explore what
immutability means in Rust and how it affects the way we write code.

### Why Immutability?

Immutability by default offers several benefits:

- **Safety:** Immutable data is inherently thread-safe, meaning it can be safely shared across threads without the risk
  of data races.
- **Predictability:** With immutable variables, you can be sure that the value of a variable won't change unexpectedly.
- **Easier to Reason About:** Code becomes easier to understand and debug when you don't have to track how and where a
  variable's value might change.

### Mutable Variables

While immutability is the default, Rust allows you to explicitly make variables mutable by using the `mut` keyword. This
flexibility lets you specify which variables need to be mutable while keeping the rest immutable.

#### Example of Mutable

```text
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This line would fail to compile because x is immutable

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // This is allowed because y is mutable
    println!("The value of y is now: {}", y);
}
```

In the example above, `x` is immutable and cannot be changed after its initial assignment. Attempting to modify `x` will
result in a `compile-time error`. On the other hand, `y` is declared as mutable with `let mut y`, allowing its value to
be changed.

### When to Use Mutable Variables

Use mutability when you know a variable's value needs to change. For instance, when you're accumulating values or
managing the state that changes over time within a function or a block of code. However, it's generally a good practice
to default to immutability and only opt for mutability when necessary.

> Immutability extends to data structures as well. For example, if you have an immutable **Vec**, you can't add elements
> to it. Similarly, if you have an immutable **struct**, you can't change its fields.
>
{style="note"}

