# Repetition Pattern

Repetition patterns in Rust macros are used to repeat a particular pattern of code based on the inputs provided to the
macro.

## Repetition Patterns in Rust Macros

Rust macros support powerful repetition patterns that enable concise and flexible code generation. Below is a table of
common patterns:

| Pattern       | Description                                                | Example Usage  | Example Expansion |
|---------------|------------------------------------------------------------|----------------|-------------------|
| `$(...),*`    | Zero or more repetitions, separated by commas.             | `$(expr),*`    | `a, b, c`         |
| `$(...);*`    | Zero or more repetitions, separated by semicolons.         | `$(expr);*`    | `a; b; c;`        |
| `$(...),+`    | One or more repetitions, separated by commas.              | `$(expr),+`    | `a, b, c`         |
| `$(...);+`    | One or more repetitions, separated by semicolons.          | `$(expr);+`    | `a; b; c;`        |
| `$(...)$(* )` | Zero or more repetitions, with no separator.               | `$(expr)$(* )` | `abc`             |
| `$(...)\|*`   | Zero or more repetitions, separated by a custom separator. | `$(expr)\|*`   | `a\|b\|c`         |

These patterns allow macros to accept a variable number of arguments and apply a repeated structure to each argument.

## Usage

Here's how you would use these patterns in a macro:

```text
/// create_function! Macro
///
/// A macro for generating functions with given names.
///
/// # Arguments
///
/// * `func_name` - A list of identifiers that will be the names of the generated functions.
///
/// # Examples
///
/// ```
/// // This macro invocation:
/// create_function!(foo, bar);
/// 
/// // Expands to:
/// fn foo() {
///     println!("foo");
/// }
/// fn bar() {
///     println!("bar");
/// }
/// ```
///
/// # Usage
///
/// You can call `create_function!` with one or more identifiers separated by commas.
/// Each identifier will be a new function that prints its own name.
///
/// # Patterns
///
/// The macro uses the following repetition pattern:
/// * `$(...),*` - Zero or more repetitions, separated by commas.
macro_rules! create_function {
    // This macro takes an argument of "zero or more" repetitions.
    // Each repetition will be separated by commas.
    ($($func_name:ident),*) => {
        $(
            // This block will be repeated for each repetition.
            // The `stringify!` macro converts the identifier into a string.
            fn $func_name() {
                // The function, when called, will print its own name.
                println!(stringify!($func_name));
            }
        )*
    };
}

// Using the macro to create functions `foo` and `bar`.
create_function!(foo, bar);

fn main() {
    // After macro expansion, the functions can be called like any other function.
    foo(); // prints "foo"
    bar(); // prints "bar"
}
```
