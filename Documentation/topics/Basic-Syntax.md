# Basic Syntax

<show-structure for="chapter" depth="3"/>

## Overview

In Rust, the basic syntax governs how you write statements, define variables, and use keywords. Rust's syntax is
block-structured, influenced by languages like C and C++.

### Statements and Expressions

A statement performs some action and does not return a value, while an expression evaluates to a value and does not
directly perform an action.

Here is an example that includes both statements and expressions:

<code-block src="syntax-and-basics/rust/basic-syntax-01.rs"/>

In this example, `let x = 5;` is a statement that doesn't return a value. The block `{ ... }` is an expression that
evaluates to the value of `x + 1`, which is `4`.

### Defining Variables

In Rust, variables are immutable by default. When a variable is declared using `let`, it cannot be changed after its
initial assignment:

<code-block src="syntax-and-basics/rust/basic-syntax-02.rs"/>

To make a variable mutable, use the `mut` keyword:

<code-block src="syntax-and-basics/rust/basic-syntax-03.rs"/>

### Data Types

Rust is statically typed, which means that it must know the types of all variables at compile time. The compiler can
usually infer what type we want to use based on the value and how we use it. When many types are possible, such as when
converting from a string to a numeric type, you must add a type annotation:

<code-block src="syntax-and-basics/rust/basic-syntax-04.rs"/>

### Functions

Functions are defined with `fn` and have a set of parameters and an optional return type. The function body is wrapped
in braces `{}`:

<code-block src="syntax-and-basics/rust/basic-syntax-05.rs"/>

### Comments

Comments in Rust start with two slashes `//` and continue until the end of the line. For comments that span multiple
lines, you can use a block comment `/* ... */`:

<code-block src="syntax-and-basics/rust/basic-syntax-06.rs"/>

## Example: Basic Rust Program

Let's look at a simple program that calculates the area of a rectangle. This program introduces multiple variables, a
function, and basic arithmetic operations:

<code-block src="syntax-and-basics/rust/basic-syntax-example.rs"/>

In this example, `calculate_area` is a function that takes two `u32` arguments and returns a `u32` value that is the
result of multiplying `width` by `height`.
