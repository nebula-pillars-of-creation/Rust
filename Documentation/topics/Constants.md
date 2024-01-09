# Constants
<show-structure for="chapter" depth="3"/>

In Rust, constants and immutability are related concepts, but they serve different purposes and have different
characteristics. Understanding the distinction between them is crucial for effective Rust programming.

## Constants vs. Immutability in Rust

### Constants (`const`)

- **Definition:** Constants in Rust are values that are bound to a name and are not allowed to change. They are defined
  using the const keyword.
- **Scope:** Constants can be declared in any scope, including the global scope.
- **Type Annotation:** Constants require explicit type annotation.
- **Value Requirements:** The value of a constant must be known at compile time. It cannot be the result of a computed
  value that is only known at runtime.
- **Use Cases:** Constants are used for values that are truly constant and will never change. For example, mathematical
  constants like PI, configuration values, or fixed application settings.

#### Example of Constants

```text
const MAX_POINTS: u32 = 100_000;
```

### Immutability (`let`)

- **Definition:** In Rust, variables are immutable by default. This means that once a variable is assigned a value, it
  cannot be changed. However, unlike constants, the value of an immutable variable can be computed at runtime.
- **Scope:** Immutability applies to variables within their scope.
- **Type Annotation:** Type annotations are optional if the compiler can infer the type.
- **Value Requirements:** An immutable variable can hold a value that is computed at runtime.
- **Use Cases:** Immutability is the default for all variables in Rust. It's used to ensure safety and consistency,
  especially in concurrent programming.

#### Example of Immutability

```text
let x = 5; // x is immutable and cannot be changed later
```

## Key Differences

- **Value Determination:** Constants must always have values that are determinable at compile time. Immutable variables
  can have values assigned at runtime.
- **Type Inference:** Constants require explicit type annotations, whereas immutable variables can have their types
  inferred by the compiler.
- **Scope and Usage:** Constants are used for values that are truly constant throughout the entire program. Immutability
  is a property of variables that prevents them from being modified after they are set.

## Practical Example

### Scenario: Mathematical Constants

Imagine you are writing a program that requires the use of mathematical constants like speed of light. These values are
known at compile time and will never change. In this case, you would use constants to define these values.

#### Example of Mathematical Constants

```text
const SPEED_OF_LIGHT: f64 = 299_792_458.0;
const PLANCK_CONSTANT: f64 = 6.626_070_15e-34;
```

### Scenario: User Input

Now imagine you might use an immutable variable for a value like the start time of a program, which is set at runtime
and does not change thereafter.

#### Example of User Input

```text
let start_time = std::time::SystemTime::now();
```

