# if and else

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

The condition must be a `bool`. If the condition is not a `bool`, you will get a compile-time error.

```text
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

```text

error[E0308]: mismatched types
 --> src\main.rs:4:8
  |

4 |     if number {
    |        ^^^^^^ expected `bool`, found integer
    |
    = note: expected type `bool`
                 found type `{integer}`
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

## `if` in a `let` Statement

Because `if` is an expression, we can use it on the right side of a `let` statement:

```text
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

The value of the `number` variable will be 5 because the condition is true. If the condition was false, the value of
`number` would be 6.

## Practical Examples

### Scenario: User Access Control

Imagine you are writing a program that requires user access control. You want to allow access to the program only if the
user is an admin or a moderator. Otherwise, you want to deny access.

#### Example of User Access Control

```text
fn main() {
    let user_role = "admin"; // This could be dynamically determined in a real application

    if user_role == "admin" {
        println!("Access granted: Full access");
    } else if user_role == "editor" {
        println!("Access granted: Editor access");
    } else {
        println!("Access denied");
    }
}
```

In this example, the user's role determines the level of access they receive. Using if and else if, you can clearly and
safely handle different cases.
