# Fibonacci Sequence

## Description

The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones, usually starting
with 0 and 1. That is, the sequence starts 0, 1, 1, 2, 3, 5, 8, 13, ...

### Algorithm

The n-th number of the Fibonacci sequence F(n) is computed as:

```tex
\begin{equation}
F(n) = 
\begin{cases} 
0 & \text{if } n = 0, \\
1 & \text{if } n = 1, \\
F(n-1) + F(n-2) & \text{if } n > 1.
\end{cases}
\end{equation}
```

This function can be implemented iteratively or recursively. The iterative approach is usually more efficient as it does
not involve the overhead of recursive function calls.

### Flowchart

```mermaid
---
title: Fibonacci Sequence
---
graph TD
    A[Start] --> B[Read n]
    B --> C{Is n == 0?}
    C -->|Yes| D[Print 0]
    C -->|No| E{Is n == 1?}
    E -->|Yes| F[Print 1]
    E -->|No| G[Set fib0 = 0 and fib1 = 1]
    G --> H[Set i = 2]
    H --> I[Set next_fib = fib0 + fib1]
    I --> J[Print next_fib]
    J --> K[Set fib0 = fib1]
    K --> L[Set fib1 = next_fib]
    L --> M[Increment i]
    M --> N{Is i < n?}
    N -->|Yes| I
    N -->|No| O[End]
    D --> O
    F --> O
```

## Code Example

<tabs>
  <tab title="Rust">
    <code-block lang="c" src="rust/fibonacci_sequence.rs" validate="false" />
  </tab>
  <tab title="C++">
    <code-block lang="c++" src="cpp/fibonacci_sequence.cpp" validate="false" />
  </tab>
  <tab title="Python">
    <code-block lang="python" src="python/fibonacci_sequence.py" validate="false" />
  </tab>
</tabs>
