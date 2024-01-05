# Data Types

<show-structure for="chapter" depth="2"/>

## Overview

Rust has a variety of data types split into two subsets: scalar and compound. Let's explore each with examples.

## Scalar Types

Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans,
and characters.

### Integers

- Unsigned (no negative values) like `u8`, `u16`, `u32`, `u64`, `u128`, and `usize` (pointer-sized).
- Signed (negative and positive values) like `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.

#### Example of Integers

<code-block src="syntax-and-basics/rust/data-types/data-types-01.rs"/>

### Floating-Point Numbers

Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type
is `f64`.

#### Example of Floating-Point Numbers

<code-block src="syntax-and-basics/rust/data-types/data-types-02.rs"/>

### Booleans

The Boolean type in Rust is specified using `bool`. It can be either `true` or `false`.

#### Example of Booleans

<code-block src="syntax-and-basics/rust/data-types/data-types-03.rs"/>

### Characters

The `char` type is the language's most primitive alphabetic type, and the following code shows one way to use it.

#### Example of Characters

<code-block src="syntax-and-basics/rust/data-types/data-types-04.rs"/>

## Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuples

A tuple is a general way of grouping together a number of values with a variety of types into one compound type.

#### Example of Tuples

<code-block src="syntax-and-basics/rust/data-types/data-types-05.rs"/>

### Arrays

Unlike a tuple, every element of an array must have the same type. Arrays in Rust have a fixed length.

#### Example of Arrays

<code-block src="syntax-and-basics/rust/data-types/data-types-06.rs"/>

## Type Conversion

Rust does not perform implicit type conversion (coercion) between primitive types. Instead, you must explicitly convert
between types using the `as` keyword or using methods such as `to_string` for converting to a `String` from a numeric
type, or `parse` for turning a string into a numeric type.

#### Example of Type Conversion

<code-block src="syntax-and-basics/rust/data-types/data-types-07.rs"/>

Output
<code-block>
Casting: 65.4321 -> 65 -> A
</code-block>
