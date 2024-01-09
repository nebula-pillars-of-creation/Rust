# Shadowing
<show-structure for="chapter" depth="3"/>

## Variable Shadowing in Rust

Shadowing is a feature in Rust that allows you to declare a new variable with the same name as a previous variable. The
new variable shadows the previous variable, meaning that the original variable is still there but is now inaccessible.
The shadowed variable and the new variable can even have different types.

### Why Use Shadowing?

Shadowing is useful when you want to transform some data while still keeping the original variable immutable. It also
allows you to reuse variable names for different purposes in the same scope, which can be convenient in certain
situations.

#### Example of Shadowing

```text
fn main() {
    let x = 5; // Immutable variable
    let x = x + 1; // Shadowing with a new value

    {
        let x = x * 2; // Shadowing within a new scope
        println!("The value of x in the inner scope is: {}", x); // 12
    }

    println!("The value of x is: {}", x); // 6
}

```

In this example, the first x is shadowed by the second x, which takes the original value and adds 1 to it. Inside the
inner scope, x is shadowed again, multiplying the value by 2. Each shadowed x is a separate variable and does not affect
the value of the x outside its scope.

### Difference Between Shadowing and Mutability

Shadowing and mutability are two different concepts:

- Mutability allows you to change the value of a variable directly. When a variable is declared mutable with mut, its
  value can be altered without creating a new variable.

- Shadowing creates a new variable with the same name, hiding the original variable. The original variable remains
  immutable, and its value is not changed.

#### Example of Difference Between Shadowing and Mutability

```text
fn main() {
    let y = 5;
    let y = y + 1; // Shadowing; effectively creates a new variable 'y'
    // The original 'y' is now inaccessible

    let mut z = 5;
    z = z + 1; // Mutability; changes the value of 'z' directly
}
```

This documentation covers the concept of shadowing in Rust, including its purpose, usage, and how it differs from
mutability. Shadowing is particularly useful in scenarios where you need to transform a value but maintain the
immutability of the original variable.

## Practical Example

### Scenario: Parsing and Transforming User Input

Imagine you are writing a program where you need to take a string input from the user, like a date, and then transform
it into a different format for processing. You might initially receive the date as a String but then need to parse it
into a DateTime object for further manipulation.

#### Example of Parsing and Transforming User Input

```text
use chrono::{NaiveDate, ParseError}; // Assuming the use of the chrono crate for date handling

fn main() -> Result<(), ParseError> {
    let user_input = "2023-12-25"; // User input as a string (e.g., a date)

    // Shadowing: Convert the string to a NaiveDate object
    let user_input = NaiveDate::parse_from_str(user_input, "%Y-%m-%d")?;

    // Perform some operations on the date
    let next_day = user_input.succ(); // Using a method to get the next day

    println!("The day after {} is {}", user_input, next_day);
    Ok(())
}

```

In this example:

- The original user_input is a string containing a date.
- The user_input is then shadowed by a new NaiveDate object, parsed from the string.
- The program then calculates the next day and prints both the original and next day's dates.

#### Explanation

Shadowing is beneficial here because:

- It allows you to reuse the user_input variable name, which makes sense contextually.
- It avoids the need for multiple, similarly named variables (like user_input_str and user_input_date), making the code
  cleaner and more readable.
- The original string user_input and the parsed NaiveDate user_input are different data types. Shadowing seamlessly
  handles this type transition.
- This practical example demonstrates how shadowing can be used to transform and reuse variables, especially when
  dealing with different data types and transformations in a logical sequence.