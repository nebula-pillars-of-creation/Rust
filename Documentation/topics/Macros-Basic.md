# Macros Basic
<show-structure for="chapter" depth="3"/>

- Macros in Rust are a way of writing code that writes other code, which is known as metaprogramming. Unlike functions,
  macros are expanded into source code that gets compiled with the rest of the program.
- Macros in Rust are powerful tools that allow for metaprogramming, used to write DRY code and to avoid repetition.

## Why Macros?

- Macros are used to reduce code repetition and to implement patterns of code in a DRY (Don't Repeat Yourself) way.
- For example, the println! macro we've used earlier can take any number of arguments of different types. Writing such a
  function without macros would be very complex and near impossible.

### Defining a Simple Macro

- Macros are defined using the `macro_rules!` attribute. They can take a variable number of arguments and are expanded
  into source code at compile

A simple macro looks like this:

```text
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}
```

You would call this macro in your code like this:

```text
fn main() {
    // This call will expand into `println!("Hello!");`
    say_hello!();
}
```

### Macros with Arguments

Macros can take arguments, and the arguments can be of different types. Here's a macro that behaves like `println!` but
always prefixes the message with "Hello:":

```text
macro_rules! say_hello {
    // `$(...),*` will match one or more expressions, separated by commas.
    ($($arg:tt),*) => {
        print!("Hello: ");
        println!($($arg),*);
    };
}
```

And it's used like this:

```text
fn main() {
    say_hello!("I'm a macro with", "multiple", "arguments.");
}
```

### How Macros Work

Macros work by matching against the pattern specified in the macro definition and then replacing the code with the
macro's code block. The syntax `$(...),*` is a repetition pattern indicating that the macro takes one or more arguments,
separated by commas.

The `$($arg:tt),*` syntax is a repetition pattern. It matches one or more tokens (`tt` stands for "token tree")
separated by commas. The `$arg` is a variable name that will be assigned to the value of each token matched by the
pattern.

The `println!` macro is a bit more complex than the macros we've seen so far. It can take any number of arguments, and
each argument can be of any type. Here's the definition of `println!`:

```text
macro_rules! println {
    // The `println!` macro is implemented in the standard library.
    // We can refer to it using the `std::` prefix.
    ($($arg:tt)*) => (std::io::println(format_args!($($arg)*)));
}
```

## Practical Example

### Scenario: Logging Messages

Imagine you are developing a software application where you need to log messages for debugging and tracking purposes.
You want to prefix each log message with the current date and time, the file, and the line number from where the log was
generated. Writing this manually for each log message would be repetitive and error-prone. This is where a macro can be
very helpful.

#### Example of Logging Messages

```text
// A macro to log messages with a timestamp, file name, and line number
macro_rules! log_message {
    ($($arg:tt)*) => {
        // Using standard library functions to get the current time, file, and line number
        println!(
            "[{}][{}:{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), // Timestamp
            file!(),  // Current file
            line!(),  // Line number
            format_args!($($arg)*)  // User-provided message
        );
    };
}

fn main() {
    let user_name = "Alice";
    let user_action = "logged in";

    // Using the macro to log a message
    log_message!("User {} has {}", user_name, user_action);
}
```

> chrono is a crate for date and time handling in Rust. You can add it to your project by adding the following line to
> your Cargo.toml file:
>
> ```toml
> [dependencies]
> chrono = "0.4.19"
> ```
>
{style="note"}

#### Explanation

The `log_message!` macro is defined to accept a variable number of arguments `($($arg:tt)*)` similar to the standard
`println!` macro. Inside the macro, we use `chrono::Local::now()` to get the current date and time, `file!()` to get the
current file name, and `line!()` to get the line number. The macro then prints out the formatted log message along with
the timestamp, file name, and line number.
