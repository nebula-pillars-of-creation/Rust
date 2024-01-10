# Decision-Making In Rust

## Overview

Decision-making is the process of choosing between different courses of action. In programming, decision-making is used
to
determine the code that should be executed next. In Rust, decision-making is done using `if` and `match`. This topic
will
cover `if` and `match`.

## `if` and `else`

The `if` and `else` keywords are used to perform conditional logic. The `if` keyword is followed by a condition, and if
that condition is true, the code block following the condition is executed. If the condition is false, the code block
following the `else` keyword is executed.

```text
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

## `if` and `else if`

You can have multiple conditions by using `else if`:

```text
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

## `match`

The `match` keyword is used to match a value against a series of patterns. The `match` keyword is followed by an
expression, and then a series of `arms`. Each arm consists of a pattern and the code that should be run if the value
matches that pattern.

```text
fn main() {
    let number = 3;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```
