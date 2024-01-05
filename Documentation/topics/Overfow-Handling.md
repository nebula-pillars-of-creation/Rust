# Integer Overflow Handling

## Overview

Rust provides guarantees about memory safety, and overflows can lead to security vulnerabilities if not managed
correctly.

By default, Rust checks for integer overflow in debug mode and causes the program to panic (crash). This safety check is
not present in release builds for performance reasons. However, you can handle potential overflows explicitly using
methods provided by Rust's standard library:

> This safety check is not present in release builds for performance reasons
>
{style="warning"}

<deflist>
    <def title="Wrapping">
         If a calculation overflows, it wraps around to the minimum value.
        <code-block src="syntax-and-basics/rust/data-types/integer-overflow-handling/overflow-handling-01.rs"/>
    </def>
    <def title="Saturating">
         If a calculation overflows, it "saturates" at the value's maximum (or minimum for underflow).
        <code-block src="syntax-and-basics/rust/data-types/integer-overflow-handling/overflow-handling-02.rs"/>
    </def>
    <def title="Checked">
         Performs the calculation and returns `None` if there is an overflow.
        <code-block src="syntax-and-basics/rust/data-types/integer-overflow-handling/overflow-handling-03.rs"/>
    </def>
    <def title="Overflowing">
         Returns the result of the calculation along with a boolean indicating whether an overflow has occurred.
        <code-block src="syntax-and-basics/rust/data-types/integer-overflow-handling/overflow-handling-04.rs"/>
    </def>
</deflist>
