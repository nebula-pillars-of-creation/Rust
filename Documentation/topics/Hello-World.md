# Hello World

## Your first Rust program

The traditional `"Hello, World!"` program is a time-honored tradition in computer programming, serving as the
introductory
rite of passage for learning a new language. It's designed to get you started with the syntax and process of writing and
running a simple program. In Rust, this program is particularly simple, yet it introduces you to several important
concepts: the fn keyword, the println! macro, and the structure of a Rust program.

### Here's the classic "Hello, World!" in Rust

<code-block lang="plain text" src="syntax-and-basics/rust/hello-world.rs"/>

<procedure title="To write and run a Rust program like this, you'll need to:">
    <step>Install Rust using rustup (which we had covered in the <a href="Environment-Setup.md">environment setup</a>).</step>
    <step>Write your Rust code in a .rs file. For e.g. <code>hello-world.rs</code></step>
    <step>Compile the code using the Rust compiler (rustc).</step>
    <step>Run the compiled program.</step>
</procedure>

### Key Concepts

1. `fn` declares a new function. The `main` function is special: it's always the first code that runs in every
   executable Rust program.
2. `println!` is a macro that prints text to the console. Macros in Rust are a way of writing code that writes other
   code, which is a form of metaprogramming. They are denoted by a `!`.

### Understanding the compilation and execution process

Compilation
: When you run `rustc hello-world.rs`, the Rust compiler takes your source code and compiles it into machine code that
the computer can understand and execute. The output is a binary executable file.

Execution
: Once the program is compiled, you can run it by simply typing `./hello` (or `hello.exe` on Windows) in your terminal.
If it's successful, you'll see `Hello, World!` printed out.

