# Factorial

## Description

For this example, we're going to use factorial. The factorial of a non-negative integer \( n \) is the product of all
positive
integers less than or equal to \( n \).

### Equation

It is denoted by \( n! \) and is generally defined as:

``` tex
\begin{equation}
n! = 
\begin{cases} 
1 & \text{if } n = 0, \\
n \cdot (n - 1)! & \text{if } n > 0.
\end{cases}
\end{equation}
```

The function can be implemented recursively, where \( n! \) is the product of \( n \) and the factorial of \( n - 1 \),
with the base case being \( 0! = 1 \).

### Flowchart

```mermaid
graph TD
    A[Start] --> B[Read n]
    B --> C{Is n < 0?}
    C -->|Yes| D[Print 'undefined for negative numbers']
    C -->|No| E{Is n == 0?}
    E -->|Yes| F[Result = 1]
    E -->|No| G[Factorial = n]
    G --> H[Set i = n - 1]
    H --> I{Is i > 1?}
    I -->|Yes| J[Factorial = Factorial * i] --> K[Decrement i] --> I
    I -->|No| L[Print Factorial]
    D --> M((End))
    F --> M
    L --> M
```

## Code Example

<tabs>
  <tab title="Rust">
    <code-block lang="c" src="factorials.rs" validate="false" />
  </tab>
  <tab title="C++">
    <code-block lang="c++" src="factorials.cpp" validate="false" />
  </tab>
  <tab title="Python">
    <code-block lang="python" src="factorials.py" validate="false" />
  </tab>
</tabs>
