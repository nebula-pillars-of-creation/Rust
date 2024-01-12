# Borrowing Rules

<show-structure for="chapter" depth="1"/>

## Overview of Borrowing

Borrowing is a core feature in Rust that works hand-in-hand with the ownership system to ensure memory safety.
Understanding borrowing and lifetimes is essential for effective Rust programming.

### What is Borrowing?

Borrowing in Rust refers to the ability to have references to a piece of data without taking ownership of it. There are
two types of borrows:

- **Immutable Borrow (&T)**: You can have multiple immutable references to data, and none of those references can modify
  the data.
- **Mutable Borrow (&mut T)**: You can have only one mutable reference to a piece of data in a particular scope, and
  this reference can be used to modify the data. These borrowing rules allow Rust to prevent data races at compile time.

### Borrowing Rules

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

### Practical Example: Modifying Data through References

Consider a function that modifies user data. You want to pass the user data as a reference to avoid ownership transfer
and only allow modifications within the function.

```text
struct UserData {
    age: u32,
}

fn main() {
    let mut user = UserData { age: 30 };

    // Borrow `user` as a mutable reference
    update_age(&mut user);

    println!("Updated age: {}", user.age);
}

fn update_age(user: &mut UserData) {
    user.age += 1;
    // The data in `user` can be modified through the mutable reference
}

```

In this example:

- The user is borrowed as a mutable reference in the update_age function.
- Within update_age, the user's age is modified.
- This follows the borrowing rules: only one mutable reference to user exists, and it is valid for the duration of
  update_age.

## Immutable Borrow (&T)

Immutable borrowing allows you to create a reference to a value without taking ownership of it. Multiple immutable
references to the same data can coexist, but none of these references can be used to modify the data.

### Practical Example: Reading User Data

Consider a scenario where you have user data, and you need to read and display this data without modifying it.

```text
struct UserData {
    name: String,
    age: u32,
}

fn display_user_data(user: &UserData) {
    println!("Name: {}, Age: {}", user.name, user.age);
}

fn main() {
    let user = UserData {
        name: String::from("Alice"),
        age: 30,
    };

    // Immutable borrow
    display_user_data(&user);

    // `user` can still be used here
    println!("User after display: {}", user.name);
}

```

In this example, display_user_data takes an immutable reference to UserData. It can read the data but cannot modify it.

## Mutable Borrow (&mut T)

Mutable borrowing allows you to create a single mutable reference to a value. This reference can be used to modify the
data, but while it exists, no other references (either mutable or immutable) to the same data are allowed.

### Practical Example: Modifying Account Balance

Consider a scenario where you need to modify a user's account balance. This requires mutable access to the user data.

```text
struct Account {
    balance: f64,
}

fn add_funds(account: &mut Account, amount: f64) {
    account.balance += amount;
}

fn main() {
    let mut user_account = Account { balance: 1000.0 };

    // Mutable borrow to modify account balance
    add_funds(&mut user_account, 500.0);

    println!("Updated Balance: {}", user_account.balance);
}

```

In this example, `add_funds` takes a mutable reference to `Account`, allowing it to modify the balance.

## Lifetimes

Lifetimes are Rust's way of ensuring that references are valid for as long as they are used. A lifetime is a
compile-time construct that describes the scope for which a reference is valid.

### Example of Lifetimes in Structs

When structs contain references, lifetimes must be used to ensure that the data referred to is valid for the lifetime of
the struct.

```text
struct User<'a> {
    username: &'a str, // 'a is a lifetime parameter
}

fn main() {
    let username = String::from("alice");
    let user = User { username: &username };

    println!("Username: {}", user.username);
    // `user`'s `username` reference is valid as long as `username` exists
}

```

In this example, the `'a` lifetime parameter ensures that `User` cannot outlive the reference it holds in `username`.

### Example: Lifetimes in Function Parameters

When defining functions that accept references, you might need to specify lifetimes to ensure that the references are
valid.

```text
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("Rust");
    let result;
    {
        let string2 = String::from("C++");
        result = longest(string1.as_str(), string2.as_str());
    } // `string2` goes out of scope here

    println!("The longest string is {}", result);
    // `result` is valid because `string1` is still in scope
}

```

In this example, the lifetime `'a` is used to specify that the return type (a reference) will live at least as long as
the shortest of `x` and `y`.

### Example: Reference in Struct

Lifetimes ensure that a struct containing references doesn't outlive the data it references. For instance, consider a
struct holding a reference to part of a string.

```text
struct TextSegment<'a> {
    part: &'a str,
}

fn main() {
    let text = String::from("Hello, world!");
    let segment;

    {
        let part = &text[7..12];  // "world"
        segment = TextSegment { part };

        // Do something with segment
        println!("Segment: {}", segment.part);
    } // 'part' goes out of scope here, but 'segment' does not.

    // This is safe because 'segment' does not outlive 'part'.
    // Uncommenting the following line would result in a compile-time error
    // println!("Segment after scope: {}", segment.part);
}

```

Here, `TextSegment` contains a reference to a string slice. The lifetime `'a` ensures that any instance of `TextSegment`
does not outlive the string slice it references. This prevents dangling references.

## Conclusion

Borrowing and lifetimes are crucial for Rust's safety guarantees. They enable safe memory access patterns while avoiding
the overhead of runtime garbage collection. Borrowing ensures that data can be safely accessed from different parts of a
program without unnecessary data copying, and lifetimes ensure that this access does not lead to dangling references.
Understanding these concepts is key to mastering Rust's unique approach to memory safety.
