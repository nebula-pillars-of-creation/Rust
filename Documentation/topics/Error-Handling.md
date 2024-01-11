# Error Handling

## Overview

Error handling is a critical part of Rust programming, designed to handle the possibility of failure in a robust and
explicit manner. Rust does not use exceptions for error handling; instead, it uses the `Result` and `Option` types as
part of its type system to handle cases that could result in errors or absence of values.

## Result Type

The Result type is an enum used for returning and propagating errors. It has two variants:

- **Ok(T)**: A value T representing success.
- **Err(E)**: A value E representing error.

### Practical Example: Reading a File

Consider a scenario where you need to read from a file, which might fail if the file does not exist or is inaccessible:

```text
use std::fs::File;
use std::io::{self, Read};

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Failed to read file: {}", e),
    }
}

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

```

In this example, the `read_file_contents` function returns a `Result` type. If the file is read successfully, it
returns `Ok(contents`). If there is an error (like the file not existing), it returns `Err(e)`.

## Option Type

The Option type is an enum used to represent an optional value. It has two variants:

- **Some(T)**: Some value T is present.
- **None**: No value is present.

### Practical Example: Finding an Element in a List

Imagine you have a list of items and need to find an element in it, which might not be present:

```text
fn main() {
    let items = vec!["apple", "banana", "orange"];

    let item = find_item(&items, "banana");
    match item {
        Some(index) => println!("Item found at index: {}", index),
        None => println!("Item not found"),
    }
}

fn find_item(items: &[&str], query: &str) -> Option<usize> {
    for (index, &item) in items.iter().enumerate() {
        if item == query {
            return Some(index);
        }
    }
    None
}

```

Here, `find_item` searches for an item in a slice. If the item is found, it returns `Some(index)`. If the item is not
found, it returns `None`.

## Conclusion

`Result` and `Option` are integral to Rust's approach to error handling and represent a more explicit and safer way of
handling errors and optional values, avoiding common mistakes like null pointer dereferences. They force the programmer
to deal with the possibility of absence of values or the occurrence of errors, leading to more robust and reliable code.
