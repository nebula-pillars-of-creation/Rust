# Rustc Vs Cargo

## Rustc

rustc is the Rust compiler command that compiles Rust source files into binary executables or libraries. It's akin to
gcc for C or javac for Java. When you compile a program with rustc, you're turning source code directly into a runnable
program.

Here's how you would compile a program with rustc:

```text
rustc main.rs
```

## Cargo

Cargo is the Rust package manager and build system. It works with packages known as "crates" and automates many tasks
such as building code, downloading libraries, and building those libraries (dependencies). Cargo uses rustc under the
hood but provides additional features.

To start a new project with Cargo, you'd run:

```text
cargo new my_project
cd my_project
cargo build
```

### Key Differences

| Feature                    | `rustc`                                        | Cargo                                                          |
|----------------------------|------------------------------------------------|----------------------------------------------------------------|
| **Purpose**                | Rust compiler, compiles source code to binary. | Rust’s build system and package manager.                       |
| **Usage**                  | Direct compilation of Rust files.              | Project management, builds, and dependency management.         |
| **Complexity**             | Suitable for simple, single-file programs.     | Designed for complex projects with multiple files.             |
| **Dependency Management**  | No automatic dependency handling.              | Automatically downloads and compiles dependencies.             |
| **Build Process**          | Compiles the code you provide, manually.       | Uses a `Cargo.toml` to manage and automate the build process.  |
| **Project Initialization** | No project initialization capabilities.        | Can initialize new projects with `cargo new`.                  |
| **Configuration**          | No configuration files.                        | Uses `Cargo.toml` for configuration.                           |
| **Commands**               | `rustc file.rs` to compile.                    | `cargo build` to build, `cargo run` to run, etc.               |
| **Tooling**                | Just a compiler.                               | Integrated with a vast tooling ecosystem for Rust development. |
| **Community Adoption**     | Less commonly used for development.            | Widely adopted for Rust development.                           |


